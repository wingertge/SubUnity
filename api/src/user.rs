use crate::{
    db::{models, models::NewUser},
    Claims, IntoStatus, State
};
use api_types::user::{user_server::User, ImageUploadRequest, SayRequest, SayResponse};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};
use image::imageops::FilterType;
use std::{fs::remove_file, ops::Deref, sync::Arc};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use image::DynamicImage;

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
        email: claims.emails.first().map(|s| &**s)
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
    use crate::db::{models::User, schema::users::dsl::*};

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

impl UserService {
    #[cfg(feature = "cloud-storage")]
    async fn save_image(&self, image: DynamicImage, id: &str) {
        use image::png::{PNGEncoder, CompressionType, self};
        use image::GenericImageView;
        use std::io::BufWriter;
        use azure_sdk_storage_blob::Blob;

        let pixels = image.pixels().collect();
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
            .with_blob_name(id)
            .with_body(&image_data)
            .finalize()
            .await
            .map_err(|e| Status::aborted(format!("{}", e)))?;
    }

    #[cfg(not(feature = "cloud-storage"))]
    async fn save_image(&self, image: DynamicImage, id: &str) {
        image
            .save(format!("../target/user_profiles/{}.png", id))
            .unwrap();
    }

    #[cfg(feature = "cloud-storage")]
    async fn remove_image(&self, id: &str) {
        use azure_sdk_storage_blob::Blob;

        let client = self.blob();

        client
            .delete_blob()
            .with_container_name("profile_pictures")
            .with_blob_name(id)
            .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
            .finalize()
            .await
            .map_err(|e| Status::aborted(format!("{}", e)))?;
    }

    async fn remove_image(&self, id: &str) {
        remove_file(format!("../target/user_profiles/{}.png", id)).unwrap();
    }
}

#[async_trait]
impl User for UserService {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        let conn = self.db()?;
        let user = get_user(&request, &conn)?;

        println!("Result: {:?}", user);
        Ok(Response::new(SayResponse {
            message: format!("hello {}", user.username)
        }))
    }

    async fn set_profile_picture(
        &self,
        request: Request<ImageUploadRequest>
    ) -> Result<Response<api_types::user::Status>, Status> {
        use crate::db::schema::users::dsl::*;

        let req = request.get_ref();

        let image = image::load_from_memory(&req.content)
            .map_err(|e| Status::invalid_argument(e.to_string()))?
            .crop(req.offset_x, req.offset_y, req.crop_size, req.crop_size)
            .resize(PFP_SIZE, PFP_SIZE, FilterType::Nearest);

        let image_id = Uuid::new_v4().to_string();

        self.save_image(image, &image_id).await;

        let conn = self.db()?;
        let user = get_user(&request, &conn)?;

        if let Some(current_picture) = user.picture {
            self.remove_image(&current_picture).await;
        }

        diesel::update(users.find(user.id))
            .set(picture.eq(image_id))
            .execute(&conn)
            .into_status()?;

        Ok(Response::new(api_types::user::Status {
            code: 400,
            message: String::new()
        }))
    }
}
