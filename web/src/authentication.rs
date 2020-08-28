use crate::{settings::Settings, ApiConn, Auth};
use api_types::user::user_client::UserClient;
use openid::{Bearer, Options, Token, Userinfo, CompactJson, SingleOrMultiple, Client, Discovered};
use rocket::{
    futures::executor,
    http::{Cookie, CookieJar},
    outcome::IntoOutcome,
    request::{FromRequest, Outcome},
    response::{status::Unauthorized, Redirect},
    Request, State
};
use std::convert::{Infallible};
use tonic::{
    metadata::{Ascii, MetadataValue},
    transport::Channel
};
use openid::biscuit::Url;
use serde::{Serialize, Deserialize};
use rocket::http::uri::Origin;
use chrono::{Utc, NaiveDateTime, DateTime};

const ACCESS_TOKEN_NAME: &str = "__auth_access__";
const ID_TOKEN_NAME: &str = "__auth_identity__";
const REFRESH_TOKEN_NAME: &str = "__auth_refresh__";

pub struct User {
    pub(crate) token: Token<Claims>,
    pub username: String
}

pub type AuthClient = Client<Discovered, Claims>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    iss: Url,
    exp: i64,
    nbf: i64,
    aud: SingleOrMultiple<String>,
    oid: String,
    sub: String,
    name: String,
    #[serde(default)]
    emails: Vec<String>,
    tfp: String,
    #[serde(default)]
    scp: Option<String>,
    #[serde(default)]
    azp: Option<String>,
    ver: String,
    iat: i64
}

const USER_INFO: Userinfo = Userinfo {
    sub: None,
    name: None,
    given_name: None,
    family_name: None,
    middle_name: None,
    nickname: None,
    preferred_username: None,
    profile: None,
    picture: None,
    website: None,
    email: None,
    email_verified: false,
    gender: None,
    birthdate: None,
    zoneinfo: None,
    locale: None,
    phone_number: None,
    phone_number_verified: false,
    address: None,
    updated_at: None
};

impl openid::Claims for Claims {
    fn iss(&self) -> &Url {
        &self.iss
    }

    fn sub(&self) -> &str {
        &self.sub
    }

    fn aud(&self) -> &SingleOrMultiple<String> {
        &self.aud
    }

    fn exp(&self) -> i64 {
        self.exp
    }

    fn iat(&self) -> i64 {
        self.iat
    }

    fn auth_time(&self) -> Option<i64> {
        None
    }

    fn nonce(&self) -> Option<&String> {
        None
    }

    fn at_hash(&self) -> Option<&String> {
        None
    }

    fn c_hash(&self) -> Option<&String> {
        None
    }

    fn acr(&self) -> Option<&String> {
        None
    }

    fn amr(&self) -> Option<&Vec<String>> {
        None
    }

    fn azp(&self) -> Option<&String> {
        self.azp.as_ref()
    }

    fn userinfo(&self) -> &Userinfo {
        &USER_INFO
    }
}
impl CompactJson for Claims {}

fn refresh_if_expired(
    request: &Request<'_>,
    mut token: Token<Claims>,
    auth: &AuthClient
) -> Option<Token<Claims>> {
    if token.bearer.expired() {
        let cookies = request.cookies();
        let bearer = executor::block_on(auth.ensure_token(token.bearer)).ok()?;
        for cookie in auth_cookies(&bearer) {
            cookies.add_private(cookie);
        }
        token = bearer.into();
        if let Some(id_token) = token.id_token.as_mut() {
            auth.decode_token(id_token).ok()?;
        }
    }
    Some(token)
}

fn validate(token: Token<Claims>, auth: &AuthClient) -> Option<Token<Claims>> {
    if let Some(id_token) = token.id_token.as_ref() {
        auth.validate_token(id_token, None, None).ok()?;
    }
    Some(token)
}

fn scopes(settings: &Settings) -> Option<String> {
    Some(format!(
        "email offline_access {}/User",
        settings.authentication.api_url
    ))
}

fn token_from_cookies(cookies: &CookieJar<'_>, settings: &Settings, auth: &AuthClient) -> Option<Token<Claims>> {
    let access = cookies.get_private(ACCESS_TOKEN_NAME)?.value().to_string();
    let id = cookies.get_private(ID_TOKEN_NAME).map(|cookie| cookie.value().to_string());
    let refresh = cookies.get_private(REFRESH_TOKEN_NAME)?.value().to_string();

    let mut token: Token<Claims> = Bearer {
        access_token: access,
        scope: scopes(settings),
        refresh_token: Some(refresh),
        expires: None,
        id_token: id
    }.into();
    if let Some(token) = token.id_token.as_mut() {
        auth.decode_token(token).ok()?;
    }
    let exp = token.id_token
        .as_ref()
        .map(|token| token.payload().unwrap())
        .map(|token| NaiveDateTime::from_timestamp(token.exp, 0))
        .map(|naive_date| DateTime::from_utc(naive_date, Utc));
    token.bearer.expires = exp;
    Some(token)
}

fn token_claims<C: openid::Claims + CompactJson>(token: &Token<C>) -> Option<&C> {
    token.id_token.as_ref()?.payload().ok()
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = Infallible;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        request.managed_state::<AuthClient>()
            .zip(request.managed_state::<Settings>())
            .and_then(|(auth, settings)| {
                token_from_cookies(request.cookies(), settings, auth)
                    .and_then(|token| refresh_if_expired(request, token, auth))
                    .and_then(|token| validate(token, auth))
                    .and_then(|token| {
                        let claims = token_claims(&token)?;
                        Some(User {
                            username: claims.name.to_string(),
                            token
                        })
                    })
            })
            .or_forward(())
    }
}

fn bearer(user: &User) -> MetadataValue<Ascii> {
    let token = &user.token.bearer.access_token;
    let mut result = String::with_capacity(token.len() + 7);
    result.push_str("Bearer ");
    result.push_str(token);
    MetadataValue::from_str(&token).unwrap()
}

pub struct AuthenticatedApiConn<'r> {
    inner: &'r Channel,
    token: MetadataValue<Ascii>
}

impl<'r> AuthenticatedApiConn<'r> {
    pub fn user(&self) -> UserClient<Channel> {
        let token = self.token.clone();
        UserClient::with_interceptor(self.inner.clone(), move |mut req: tonic::Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedApiConn<'r> {
    type Error = Infallible;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        request.guard::<User>().await.succeeded()
            .zip(request.managed_state::<ApiConn>())
            .map(|(user, api)| AuthenticatedApiConn {
                inner: &api.0,
                token: bearer(&user)
            })
            .or_forward(())
    }
}

fn auth_cookies(token: &Bearer) -> Vec<Cookie<'static>> {
    let access = Cookie::new(ACCESS_TOKEN_NAME, token.access_token.clone());
    let refresh = Cookie::new(REFRESH_TOKEN_NAME, token.refresh_token.clone().unwrap());
    if let Some(id_token) = &token.id_token {
        vec![
            access,
            Cookie::new(ID_TOKEN_NAME, id_token.clone()),
            refresh
        ]
    } else {
        vec![access, refresh]
    }
}

#[get("/login/oauth2/code/oidc?<code>")]
pub async fn login(
    auth: Auth<'_>,
    code: String,
    cookies: &CookieJar<'_>
) -> Result<Redirect, Unauthorized<String>> {
    match request_token(auth, code).await {
        Ok(None) => Err(Unauthorized(None)),
        Err(err) => Err(Unauthorized(Some(format!("{:?}", err)))),
        Ok(Some(token)) => {
            for cookie in auth_cookies(&token.bearer) {
                cookies.add_private(cookie);
            }
            let redirect_cookie = cookies.get("redirect_to");
            if let Some(cookie) = redirect_cookie {
                let value = cookie.value().to_string();
                cookies.remove(cookie.into_cookie());
                Ok(Redirect::found(value))
            } else { Ok(Redirect::found(uri!(super::index))) }
        }
    }
}

async fn request_token(
    auth: Auth<'_>,
    code: String
) -> Result<Option<Token<Claims>>, openid::error::Error> {
    let mut token: Token<Claims> = auth.request_token(&code).await?.into();
    if let Some(mut id_token) = token.id_token.as_mut() {
        auth.decode_token(&mut id_token)?;
        auth.validate_token(&id_token, None, None)?;
        eprintln!("token: {:#?}", id_token);
    } else {
        return Ok(None);
    }

    Ok(Some(token))
}

#[get("/login")]
pub async fn authorize(auth: Auth<'_>, settings: State<'_, Settings>) -> Redirect {
    let auth_url = auth.auth_url(&Options {
        scope: Some(format!(
            "email offline_access {}/User",
            settings.authentication.api_url
        )),
        ..Default::default()
    });

    eprintln!("authorize: {}", auth_url);

    Redirect::found(auth_url.to_string())
}

pub fn unauthorized_redirect(current_uri: Origin, cookies: &CookieJar<'_>) -> Redirect {
    cookies.add(Cookie::new("redirect_to", current_uri.to_string()));
    Redirect::to(uri!(authorize))
}