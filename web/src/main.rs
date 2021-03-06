#[macro_use]
extern crate rocket;
#[macro_use]
extern crate async_trait;

use crate::{
    authentication::{AuthClient, AuthenticatedApiConn},
    settings::{Authentication, Settings},
    templates::{edit_html, index_html, statics::StaticFile}
};
use api_types::user::{user_service_client::UserServiceClient, SayRequest};
use config::Config;
use failure::Fail;
use rocket::{
    response::{content::Html, status::NotFound},
    State
};
use rocket_contrib::{helmet::SpaceHelmet, serve::StaticFiles};
use std::{error::Error, io, path::PathBuf, env};
use tonic::transport::Channel;
pub use api_types::user::User;
use crate::authentication::{UserCache, unauthorized_redirect};
use api_types::subtitles::video_subs_client::VideoSubsClient;
use rocket::http::{CookieJar};
use rocket::response::Redirect;

mod authentication;
mod profile;
mod settings;
mod templates;
mod subtitles;

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

#[get("/edit/<video_id>?<lang>")]
fn edit(video_id: String, lang: String, _conn: AuthenticatedApiConn<'_>) -> Template {
    template(|w| edit_html(w, &video_id, &lang))
}

#[get("/edit/<video_id>?<lang>", rank = 2)]
fn edit_redirect(video_id: String, lang: String, cookies: &CookieJar) -> Redirect {
    unauthorized_redirect(uri!(edit: video_id = video_id, lang = lang), cookies)
}

pub struct ApiConn(pub Channel);

impl ApiConn {
    pub fn user(&self) -> UserServiceClient<Channel> {
        UserServiceClient::new(self.0.clone())
    }
    pub fn subtitles(&self) -> VideoSubsClient<Channel> {
        VideoSubsClient::new(self.0.clone())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if env::var("ROCKET_ENV").is_err() {
        env::set_var("ROCKET_ENV", "development");
    }

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

        AuthClient::discover(
            client_id.to_string(),
            client_secret.to_string(),
            Some("http://localhost:8000/login/oauth2/code/oidc".to_string()),
            issuer.parse()?
        )
    }
    .await
    .map_err(Fail::compat)?;

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    let user_cache = UserCache::new();

    rocket::ignite()
        .attach(SpaceHelmet::default())
        .manage(auth)
        .manage(ApiConn(channel))
        .manage(settings)
        .manage(user_cache)
        .mount(
            "/",
            routes![
                index,
                index2,
                asset,
                authentication::login,
                authentication::authorize,
                profile::profile,
                profile::profile_unauthorized,
                edit,
                edit_redirect
            ]
        )
        .mount("/subtitles", routes![
            subtitles::get_subtitles,
            subtitles::set_subtitles,
            subtitles::download_subtitles
        ])
        .mount("/js", StaticFiles::from("./js"))
        .mount("/asset", StaticFiles::from("./assets"))
        .launch()
        .await?;

    Ok(())
}
