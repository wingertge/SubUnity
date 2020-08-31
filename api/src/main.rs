#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::{
    settings::{Authentication, Settings},
    user::UserService
};
use api_types::user::user_service_server::UserServiceServer;
use config::Config;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    result::Error as DieselError,
    SqliteConnection
};
use openid::{biscuit::Url, Client, CompactJson, Discovered, Jws, SingleOrMultiple, Userinfo};
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, sync::Arc};
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};

mod db;
mod settings;
mod user;
mod subtitles;

trait IntoStatus<T> {
    fn into_status(self) -> Result<T, Status>;
}

impl<T> IntoStatus<T> for Result<T, DieselError> {
    fn into_status(self) -> Result<T, Status> {
        self.map_err(|err| match err {
            DieselError::NotFound => Status::not_found(""),
            DieselError::DatabaseError(_, info) => Status::aborted(info.message()),
            _ => Status::aborted("database error")
        })
    }
}

impl<T> IntoStatus<T> for Result<T, r2d2::Error> {
    fn into_status(self) -> Result<T, Status> {
        self.map_err(|err| Status::aborted(format!("{}", err)))
    }
}

pub struct State {
    db: Database,
    #[allow(dead_code)]
    conf: Settings
}

impl State {
    pub fn db(&self) -> Result<DbConnection, Status> {
        self.db.get().into_status()
    }

    #[cfg(feature = "cloud-storage")]
    pub fn blob(&self) -> azure_sdk_storage_core::key_client::KeyClient {
        use azure_sdk_storage_core::client as blob_client;
        use settings::Storage;

        let Settings {
            storage: Storage {
                blob_account,
                blob_key
            },
            ..
        } = &self.conf;
        blob_client::with_access_key(blob_account, blob_key)
    }
}

type Database = Pool<ConnectionManager<SqliteConnection>>;
type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

embed_migrations!("migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

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
                },
            ..
        } = &settings;

        Client::discover(
            client_id.to_string(),
            client_secret.to_string(),
            None,
            issuer.parse()?
        )
    }
    .await
    .unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    {
        let conn = pool.get().unwrap();
        embedded_migrations::run(&conn).unwrap();
    }

    let state = Arc::new(State {
        db: pool,
        conf: settings
    });

    let addr = "[::1]:50051".parse().unwrap();
    let user = UserServiceServer::with_interceptor(UserService(state.clone()), authorize(auth));
    println!("Server listening on {}", addr);
    Server::builder().add_service(user).serve(addr).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
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
    scp: String,
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

fn authorize(
    client: Client<Discovered, Claims>
) -> impl Fn(Request<()>) -> Result<Request<()>, Status> {
    move |mut req| {
        if let Some(token) = req.metadata().get("authorization") {
            let token = token
                .to_str()
                .map_err(|_| Status::aborted("bad authorization header"))?
                .replacen("Bearer ", "", 1);

            let mut token = Jws::new_encoded(&token);
            client.decode_token(&mut token).unwrap();
            client.validate_token(&mut token, None, None).unwrap();
            let token = token.payload().unwrap();
            let json = serde_json::to_string(&token).unwrap();
            let meta = MetadataValue::from_str(&json).unwrap();
            req.metadata_mut().insert("user", meta);

            Ok(req)
        } else {
            Ok(req)
        }
    }
}
