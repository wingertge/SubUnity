use crate::{Auth, ApiConn};
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::response::status::Unauthorized;
use openid::{Token, Options, Bearer, DiscoveredClient};
use rocket::{State, Request};
use crate::settings::Settings;
use rocket::request::{FromRequest, Outcome};
use rocket::outcome::IntoOutcome;
use std::convert::Infallible;
use tonic::metadata::{MetadataValue, Ascii};
use tonic::transport::Channel;
use api_types::user::user_client::UserClient;
use rocket::futures::executor;

pub struct User {
    pub(crate) token: Token
}

fn refresh_if_expired(request: &Request<'_>, mut bearer: Bearer, auth: &DiscoveredClient) -> Option<Bearer> {
    if bearer.expired() {
        bearer = executor::block_on(auth.ensure_token(bearer)).ok()?;
        request.cookies().add_private(auth_cookie(&bearer));
    }
    Some(bearer)
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = Infallible;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let auth = request.managed_state::<DiscoveredClient>().unwrap();
        request.cookies()
            .get_private("__auth__")
            .and_then(|cookie| serde_json::from_str::<Bearer>(cookie.value()).ok())
            .and_then(|bearer| refresh_if_expired(request, bearer, auth))
            .map(|bearer| User { token: bearer.into() })
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

impl <'r> AuthenticatedApiConn<'r> {
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
        let user = request.guard::<User>().await;
        let api = request.managed_state::<ApiConn>();
        match (user, api) {
            (Outcome::Success(user), Some(api)) => Outcome::Success(AuthenticatedApiConn {
                inner: &api.0,
                token: bearer(&user)
            }),
            _ => Outcome::Forward(())
        }
    }
}

fn auth_cookie(token: &Bearer) -> Cookie<'static> {
    Cookie::new("__auth__", serde_json::to_string(token).unwrap())
}

#[get("/login/oauth2/code/oidc?<code>")]
pub async fn login(auth: Auth<'_>, code: String, cookies: &CookieJar<'_>) -> Result<Redirect, Unauthorized<String>> {
    match request_token(auth, code).await {
        Ok(None) => Err(Unauthorized(None)),
        Err(err) => Err(Unauthorized(Some(format!("{:?}", err)))),
        Ok(Some(token)) => {
            cookies.add_private(auth_cookie(&token.bearer));
            Ok(Redirect::found(uri!(super::index)))
        }
    }
}

async fn request_token(auth: Auth<'_>, code: String) -> Result<Option<Token>, openid::error::Error> {
    let mut token: Token = auth.request_token(&code).await?.into();
    if let Some(mut id_token) = token.id_token.as_mut() {
        auth.decode_token(&mut id_token)?;
        auth.validate_token(&id_token, None, None)?;
        eprintln!("token: {:#?}", id_token);
    } else {
        return Ok(None)
    }

    Ok(Some(token))
}

#[get("/login")]
pub async fn authorize(auth: Auth<'_>, settings: State<'_, Settings>) -> Redirect {
    let auth_url = auth.auth_url(&Options {
        scope: Some(format!("email offline_access {}/User", settings.authentication.api_url)),
        ..Default::default()
    });

    eprintln!("authorize: {}", auth_url);

    Redirect::found(auth_url.to_string())
}