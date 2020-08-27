#[macro_use]
extern crate rocket;
#[macro_use]
extern crate async_trait;

use crate::{
    authentication::AuthenticatedApiConn,
    settings::{Authentication, Settings},
    templates::{index_html, statics::StaticFile}
};
use api_types::user::{user_client::UserClient, SayRequest};
use config::Config;
use failure::Fail;
use openid::{Client, Discovered};
use rocket::{
    response::{content::Html, status::NotFound},
    State
};
use rocket_contrib::helmet::SpaceHelmet;
use std::{error::Error, path::PathBuf, io};
use tonic::transport::Channel;
pub use authentication::User;
use crate::authentication::{AuthClient, Claims};

mod authentication;
mod profile;
mod settings;
mod templates;

type Template = Html<Vec<u8>>;
pub type API<'a> = State<'a, ApiConn>;
pub type AuthAPI<'a> = AuthenticatedApiConn<'a>;
type Auth<'a> = State<'a, AuthClient>;

fn template<F: FnOnce(&mut Vec<u8>) -> io::Result<()>>(f: F) -> Template {
    let mut buf = Vec::new();
    f(&mut buf).unwrap();
    Html(buf)
}

#[get("/")]
async fn index(api: AuthAPI<'_>) -> Template {
    let request = tonic::Request::new(SayRequest {
        name: "Lisa".to_string()
    });
    let response = api.user().send(request).await.unwrap().into_inner();
    template(|out| index_html(out, &response.message, &["test item"]))
}

#[get("/", rank = 2)]
async fn index2() -> Template {
    template(|out| index_html(out, "Lisa", &["test item"]))
}

#[get("/static/<path..>")]
fn asset(path: PathBuf) -> Result<&'static [u8], NotFound<()>> {
    StaticFile::get(path.to_str().unwrap())
        .map(|file| file.content)
        .ok_or_else(|| NotFound(()))
}

pub struct ApiConn(pub Channel);

impl ApiConn {
    pub fn user(&self) -> UserClient<Channel> {
        UserClient::new(self.0.clone())
    }
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
            authentication:
                Authentication {
                    client_id,
                    client_secret,
                    issuer,
                    ..
                }
        } = &settings;

        Client::<Discovered, Claims>::discover(
            client_id.to_string(),
            client_secret.to_string(),
            Some("http://localhost:8000/login/oauth2/code/oidc".to_string()),
            issuer.parse()?
        )
    }
    .await
    .map_err(Fail::compat)?;

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    rocket::ignite()
        .attach(SpaceHelmet::default())
        .manage(auth)
        .manage(ApiConn(channel))
        .manage(settings)
        .mount(
            "/",
            routes![
                index,
                index2,
                asset,
                authentication::login,
                authentication::authorize,
                profile::profile,
                profile::profile_unauthorized
            ]
        )
        .launch()
        .await?;

    Ok(())
}
