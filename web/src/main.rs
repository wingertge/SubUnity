#[macro_use]
extern crate rocket;
#[macro_use]
extern crate async_trait;

use tonic::transport::Channel;
use std::error::Error;
use api_types::user::user_client::UserClient;
use rocket::{State, Request};
use api_types::user::SayRequest;
use crate::templates::index_html;
use rocket::response::content::Html;
use crate::settings::{Settings, Authentication};
use config::Config;
use openid::{DiscoveredClient, Options, Token, Bearer};
use rocket::response::Redirect;
use failure::Fail;
use rocket::response::status::Unauthorized;
use rocket::http::{Cookies, Cookie};
use rocket::request::{FromRequest, Outcome};
use rocket::outcome::IntoOutcome;

mod templates;
mod settings;

type Template = Html<Vec<u8>>;
type UserAPI<'a> = State<'a, UserClient<Channel>>;
type Auth<'a> = State<'a, DiscoveredClient>;

pub struct User {
    token: Token
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = Redirect;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("__auth__")
            .and_then(|cookie| serde_json::from_str::<Bearer>(cookie.value()).ok())
            .map(|bearer| User { token: bearer.into() })
            .or_forward(())
    }
}

fn write_html<F: FnOnce(&mut Vec<u8>)>(f: F) -> Template {
    let mut buf = Vec::new();
    f(&mut buf);
    Html(buf)
}

#[get("/")]
async fn index(api: UserAPI<'_>) -> Template {
    let request = tonic::Request::new(SayRequest {
        name: "Lisa".to_string()
    });
    let response = api.clone().send(request).await.unwrap().into_inner();
    write_html(|out| {
        index_html(out, &response.message, &["test item"]).unwrap()
    })
}

#[get("/login/oauth2/code/oidc?<code>")]
async fn login(auth: Auth<'_>, code: String, mut cookies: Cookies<'_>) -> Result<Redirect, Unauthorized<String>> {
    match request_token(auth, code).await {
        Ok(None) => Err(Unauthorized(None)),
        Err(err) => Err(Unauthorized(Some(format!("{:?}", err)))),
        Ok(Some(token)) => {
            cookies.add_private(Cookie::new(
                "__auth__",
                serde_json::to_string(&token.bearer).unwrap()
            ));
            Ok(Redirect::found(uri!(index)))
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

#[get("/static/<path..>")]
fn asset(path: PathBuf) -> Result<&'static [u8], NotFound<()>> {
    StaticFile::get(path.to_str().unwrap())
        .map(|file| file.content)
        .ok_or_else(|| NotFound(()))
}

#[get("/login")]
async fn authorize(auth: Auth<'_>, settings: State<'_, Settings>) -> Redirect {
    let auth_url = auth.auth_url(&Options {
        scope: Some(format!("email offline_access {}/User", settings.authentication.api_url)),
        ..Default::default()
    });

    eprintln!("authorize: {}", auth_url);

    Redirect::found(auth_url.to_string())
}

#[get("/profile")]
async fn profile(user: User) -> &'static str {
    "you're logged in!"
}

#[get("/profile", rank = 2)]
async fn profile_unauthorized() -> Redirect {
    Redirect::to(uri!(authorize))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::default();
    config
        .merge(config::File::with_name("Config").required(false))?
        .merge(config::File::with_name("Config.dev.toml").required(true))?
        .merge(config::Environment::with_prefix("APP"))?;
    let settings: Settings = config.try_into()?;

    let auth = {
        let Settings {
            authentication: Authentication {
                client_id,
                client_secret,
                issuer,
                ..
            }
        } = &settings;

        DiscoveredClient::discover(
            client_id.to_string(),
            client_secret.to_string(),
            Some("http://localhost:8000/login/oauth2/code/oidc".to_string()),
            issuer.parse()?
        )
    }.await.map_err(Fail::compat)?;

    let user = {
        let channel = Channel::from_static("http://[::1]:50051")
            .connect()
            .await?;
        UserClient::new(channel)
    };

    rocket::ignite()
        .mount("/", routes![index, login, authorize, profile, profile_unauthorized])
        .manage(auth)
        .manage(user)
        .manage(settings)
        .launch().await?;

    Ok(())
}
