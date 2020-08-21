use std::sync::Arc;
use crate::{State, Claims, IntoStatus};
use std::ops::Deref;
use diesel::{SqliteConnection, QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::db::models;
use crate::db::models::NewUser;
use tonic::{Request, Status, Response};
use api_types::user::{SayRequest, SayResponse, ImageUploadRequest};
use image::imageops::FilterType;
use uuid::Uuid;
use api_types::user::user_server::User;
use std::fs::remove_file;

const PFP_SIZE: u32 = 256;

pub struct UserService(pub Arc<State>);

impl Deref for UserService {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn init_user(claims: Claims, conn: &SqliteConnection) -> QueryResult<models::User> {
    use crate::db::schema::users;

    let user = NewUser {
        id: &claims.sub,
        username: &claims.name,
        email: claims.emails.first().map(|s| &**s),
    };

    let res = diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)?;
    assert_eq!(res, 1);

    users::table
        .find(&claims.sub)
        .load::<models::User>(conn)?
        .pop()
        .ok_or_else(|| diesel::NotFound)
}

fn get_user<T>(request: &Request<T>, conn: &SqliteConnection) -> Result<models::User, Status> {
    use crate::db::models::User;
    use crate::db::schema::users::dsl::*;

    let user = request
        .metadata()
        .get("user")
        .ok_or_else(|| Status::unauthenticated("not authenticated"))?
        .to_str()
        .unwrap();
    let claims: Claims = serde_json::from_str(user).unwrap();
    Ok(users
        .find(&claims.sub)
        .load::<User>(conn)
        .into_status()?
        .pop()
        .unwrap_or_else(|| init_user(claims, conn).unwrap()))
}

#[async_trait]
impl User for UserService {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        let conn = self.db()?;
        let user = get_user(&request, &conn)?;

        println!("Result: {:?}", user);
        Ok(Response::new(SayResponse {
            message: format!("hello {}", user.username),
        }))
    }

    async fn set_profile_picture(
        &self,
        request: Request<ImageUploadRequest>,
    ) -> Result<Response<api_types::user::Status>, Status> {
        use crate::db::schema::users::dsl::*;

        let req = request.get_ref();

        let image = image::load_from_memory(&req.content)
            .map_err(|e| Status::invalid_argument(e.to_string()))?
            .crop(req.offset_x, req.offset_y, req.crop_size, req.crop_size)
            .resize(PFP_SIZE, PFP_SIZE, FilterType::Nearest);

        let image_id = Uuid::new_v4().to_string();

        //Temporary dev thing
        image.save(format!("../target/user_profiles/{}.png", image_id)).unwrap();

        //Proper thing using azure blob storage
/*                let pixels = image.pixels().collect();
                let mut image_data = Vec::new();
                let encoder = PNGEncoder::new_with_quality(
                    BufWriter::new(&mut image_data),
                    CompressionType::Best,
                    png::FilterType::Sub,
                );
                encoder.encode(&pixels, PFP_SIZE, PFP_SIZE, image.color());

                let client = self.blob();
                client
                    .put_block_blob()
                    .with_container_name("profile_pictures")
                    .with_blob_name(&image_id)
                    .with_body(&image_data)
                    .finalize()
                    .await
                    .map_err(|e| Status::aborted(format!("{}", e)))?;*/

        let conn = self.db()?;
        let user = get_user(&request, &conn)?;

        if let Some(current_picture) = user.picture {
            //dev thing
            remove_file(format!("../target/user_profiles/{}.png", current_picture)).unwrap();

            //real thing
            /*            client.delete_blob()
                            .with_container_name("profile_pictures")
                            .with_blob_name(&picture)
                            .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
                            .finalize()
                            .map_err(|e| Status::aborted(format!("{}", e)))?;*/
        }

        diesel::update(users.find(user.id))
            .set(picture.eq(image_id))
            .execute(&conn)
            .into_status()?;

        Ok(Response::new(api_types::user::Status { code: 400, message: String::new() }))
    }
}