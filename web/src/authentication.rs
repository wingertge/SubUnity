use crate::{settings::Settings, ApiConn, Auth};
use api_types::user::{user_service_client::UserServiceClient, User, UserIdentity};
use chrono::{DateTime, NaiveDateTime, Utc};
use openid::{Bearer, Options, Token, DiscoveredClient, StandardClaims};
use rocket::{
    futures::executor,
    http::{uri::Origin, Cookie, CookieJar},
    outcome::IntoOutcome,
    request::{FromRequest, Outcome},
    response::{status::Unauthorized, Redirect},
    Request, State
};
use std::convert::Infallible;
use tonic::{
    metadata::{Ascii, MetadataValue},
    transport::Channel
};
use std::collections::BTreeMap;
use parking_lot::RwLock;
use std::ops::Deref;
use api_types::subtitles::video_subs_client::VideoSubsClient;

const ACCESS_TOKEN_NAME: &str = "__auth_access__";
const ID_TOKEN_NAME: &str = "__auth_identity__";
const REFRESH_TOKEN_NAME: &str = "__auth_refresh__";

pub struct CurrentUser(User);

impl Deref for CurrentUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type AuthClient = DiscoveredClient;

fn refresh_if_expired(
    request: &Request<'_>,
    mut token: Token,
    auth: &AuthClient
) -> Option<Token> {
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

fn validate(token: Token, auth: &AuthClient) -> Option<Token> {
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

fn token_from_cookies(
    cookies: &CookieJar<'_>,
    settings: &Settings,
    auth: &AuthClient
) -> Option<Token> {
    let access = cookies.get_private(ACCESS_TOKEN_NAME)?.value().to_string();
    let id = cookies
        .get_private(ID_TOKEN_NAME)
        .map(|cookie| cookie.value().to_string());
    let refresh = cookies.get_private(REFRESH_TOKEN_NAME)?.value().to_string();

    let mut token: Token = Bearer {
        access_token: access,
        scope: scopes(settings),
        refresh_token: Some(refresh),
        expires: None,
        id_token: id
    }
    .into();
    if let Some(token) = token.id_token.as_mut() {
        auth.decode_token(token).ok()?;
    }
    let exp = token
        .id_token
        .as_ref()
        .map(|token| token.payload().unwrap())
        .map(|token| NaiveDateTime::from_timestamp(token.exp, 0))
        .map(|naive_date| DateTime::from_utc(naive_date, Utc));
    token.bearer.expires = exp;
    Some(token)
}

fn token_claims(token: &Token) -> Option<&StandardClaims> {
    token.id_token.as_ref()?.payload().ok()
}

pub struct UserCache {
    cache: RwLock<BTreeMap<String, User>>
}

impl UserCache {
    pub fn new() -> Self {
        UserCache {
            cache: RwLock::new(BTreeMap::new())
        }
    }

    pub async fn get(&self, id: &str, api: &AuthenticatedApiConn<'_>) -> Option<User> {
        let mut existing = {
            let cache = self.cache.read();
            cache.get(id).cloned()
        };
        if existing.is_none() {
            let user = {
                let mut api = api.user();
                api.get_user(tonic::Request::new(UserIdentity {
                    sub: id.to_string()
                })).await.unwrap().into_inner()
            };

            {
                let mut cache = self.cache.write();
                cache.insert(id.to_string(), user.clone());
            }

            existing.replace(user);
        }
        existing
    }
}

pub struct AuthToken(pub Token);

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = Infallible;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        request
            .managed_state::<AuthClient>()
            .zip(request.managed_state::<Settings>())
            .and_then(|(auth, settings)| {
                token_from_cookies(request.cookies(), settings, auth)
                    .and_then(|token| refresh_if_expired(request, token, auth))
                    .and_then(|token| validate(token, auth))
            })
            .map(|token| AuthToken(token))
            .or_forward(())
    }
}

trait Flatten<A, B, C> {
    fn flatten(self) -> (A, B, C);
}

impl<A, B, C> Flatten<A, B, C> for ((A, B), C) {
    fn flatten(self) -> (A, B, C) {
        ((self.0).0, (self.0).1, self.1)
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = Infallible;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token = request.guard::<AuthToken>().await.succeeded()
            .zip(request.managed_state::<ApiConn>())
            .map(|(token, api)| {
                let conn = AuthenticatedApiConn::from_parts(&api.0, &token);
                (token, conn)
            })
            .zip(request.managed_state::<UserCache>())
            .map(Flatten::flatten);

        async fn fetch_user(token: Token, cache: &UserCache, api: &AuthenticatedApiConn<'_>) -> Option<CurrentUser> {
            let claims = token_claims(&token)?;
            let user = cache.get(&claims.sub, api).await?;
            println!(r#"User {{
                id: {},
                username: {},
                email: {}
            }}"#, user.id, user.username, user.email);
            Some(CurrentUser(user))
        }

        let res = if let Some((token, api, cache)) = token {
            fetch_user(token.0, cache, &api).await
        } else { None };
        res.or_forward(())
    }
}

fn bearer(user: &AuthToken) -> MetadataValue<Ascii> {
    let token = &user.0.bearer.access_token;
    let mut result = String::with_capacity(token.len() + 7);
    result.push_str("Bearer ");
    result.push_str(token);
    MetadataValue::from_str(&token).unwrap()
}

pub struct AuthenticatedApiConn<'r> {
    inner: &'r Channel,
    token: MetadataValue<Ascii>
}

fn interceptor(token: MetadataValue<Ascii>) -> impl Into<tonic::Interceptor> {
    move |mut req: tonic::Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    }
}

impl<'r> AuthenticatedApiConn<'r> {
    pub fn from_parts(inner: &'r Channel, token: &AuthToken) -> Self {
        Self {
            inner,
            token: bearer(token)
        }
    }

    pub fn user(&self) -> UserServiceClient<Channel> {
        UserServiceClient::with_interceptor(
            self.inner.clone(),
            interceptor(self.token.clone())
        )
    }

    pub fn subtitles(&self) -> VideoSubsClient<Channel> {
        VideoSubsClient::with_interceptor(
            self.inner.clone(),
            interceptor(self.token.clone())
        )
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedApiConn<'r> {
    type Error = Infallible;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        request
            .guard::<AuthToken>()
            .await
            .succeeded()
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
            refresh,
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
                cookies.remove(cookie.clone());
                Ok(Redirect::found(value))
            } else {
                Ok(Redirect::found(uri!(super::index)))
            }
        }
    }
}

async fn request_token(
    auth: Auth<'_>,
    code: String
) -> Result<Option<Token>, openid::error::Error> {
    let mut token: Token = auth.request_token(&code).await?.into();
    if let Some(mut id_token) = token.id_token.as_mut() {
        auth.decode_token(&mut id_token)?;
        auth.validate_token(&id_token, None, None)?;
        //eprintln!("token: {:#?}", id_token);
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
