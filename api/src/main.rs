#[macro_use] extern crate async_trait;
#[macro_use] extern crate diesel;

use tonic::{Response, Status, Request};
use std::error::Error;
use tonic::transport::Server;
use api_types::user::{SayRequest, SayResponse};
use api_types::user::user_server::{User, UserServer};
use std::env;
use diesel::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;
use openid::{Jws, SingleOrMultiple, Userinfo, Client, Discovered, CompactJson};
use openid::biscuit::Url;
use serde::{Serialize, Deserialize};
use config::Config;
use tonic::metadata::MetadataValue;
use crate::db::models::{NewUser, self};

mod db;

type Database = Pool<ConnectionManager<SqliteConnection>>;

pub struct UserService { db: Database }

fn init_user(claims: Claims, name: &str, conn: &SqliteConnection) -> models::User {
    use crate::db::schema::users;

    let user = NewUser {
        id: &claims.sub,
        username: name,
        email: claims.emails.first().map(|s| &**s)
    };

    let res = diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
        .expect("Error saving user");
    assert_eq!(res, 1);

    users::table.find(&claims.sub)
        .load::<models::User>(conn)
        .expect("Error loading user")
        .pop()
        .unwrap()
}

#[async_trait]
impl User for UserService {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        use crate::db::schema::users::dsl::*;
        use crate::db::models::User;

        let user = request.metadata().get("user").unwrap().to_str().unwrap();
        let claims: Claims = serde_json::from_str(user).unwrap();
        println!("{:?}", claims);

        let name=  request.into_inner().name;

        let conn = self.db.get().unwrap();
        let user: User = users.find(&claims.sub)
            .load::<User>(&conn)
            .expect("Error loading user")
            .pop()
            .unwrap_or_else(|| init_user(claims, &name, &conn));

        println!("Result: {:?}", user);
        Ok(Response::new(SayResponse {
            message: format!("hello {}", user.username)
        }))
    }
}

#[derive(Default, Deserialize)]
pub struct Settings {
    pub authentication: Authentication
}

#[derive(Default, Deserialize)]
pub struct Authentication {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub api_url: String,

    pub signin_policy: String,
    pub edit_profile_policy: String,
    pub reset_password_policy: String
}

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
            authentication: Authentication {
                client_id,
                client_secret,
                issuer,
                ..
            }
        } = &settings;

        Client::discover(
            client_id.to_string(),
            client_secret.to_string(),
            None,
            issuer.parse()?
        )
    }.await.unwrap();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    let addr = "[::1]:50051".parse().unwrap();
    let user = UserServer::with_interceptor(UserService { db: pool.clone() }, authorize(auth));
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(user)
        .serve(addr)
        .await?;
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

fn authorize(client: Client<Discovered, Claims>) -> impl Fn(Request<()>) -> Result<Request<()>, Status> {
    move |mut req| {
        if let Some(token) = req.metadata().get("authorization") {
            let token = token.to_str()
                .map_err(|_| Status::aborted("bad authorization header"))?
                .replacen("Bearer ", "", 1);

            let mut token = Jws::new_encoded(&token);
            client.decode_token(&mut token).unwrap();
            client.validate_token(&mut token, None, None).unwrap();
            let token = match token {
                Jws::Decoded { payload, .. } => payload,
                Jws::Encoded(_) => unreachable!()
            };
            let json = serde_json::to_string(&token).unwrap();
            let meta = MetadataValue::from_str(&json).unwrap();
            req.metadata_mut().insert("user", meta);

            Ok(req)
        } else { Ok(req) }
    }
}