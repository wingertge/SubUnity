#[macro_use]
extern crate rocket;

use tonic::transport::Channel;
use std::error::Error;
use api_types::user::user_client::UserClient;
use rocket::State;
use tonic::Request;
use api_types::user::SayRequest;

type UserAPI<'a> = State<'a, UserClient<Channel>>;

#[get("/")]
async fn index(api: UserAPI<'_>) -> String {
    let request = Request::new(SayRequest {
        name: "Lisa".to_string()
    });
    let response = api.clone().send(request).await.unwrap().into_inner();
    response.message
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let user = {
        let channel = Channel::from_static("http://[::1]:50051")
            .connect()
            .await?;
        UserClient::new(channel)
    };

    rocket::ignite()
        .mount("/", routes![index])
        .manage(user)
        .launch().await?;

    Ok(())
}
