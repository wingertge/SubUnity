#[macro_use]
extern crate rocket;

use tonic::transport::Channel;
use std::error::Error;
use api_types::user::user_client::UserClient;
use rocket::State;
use tonic::Request;
use api_types::user::SayRequest;
use crate::templates::index_html;
use rocket::response::content::Html;

mod templates;

type Template = Html<Vec<u8>>;
type UserAPI<'a> = State<'a, UserClient<Channel>>;

fn write_html<F: FnOnce(&mut Vec<u8>)>(f: F) -> Template {
    let mut buf = Vec::new();
    f(&mut buf);
    Html(buf)
}

#[get("/")]
async fn index(api: UserAPI<'_>) -> Template {
    let request = Request::new(SayRequest {
        name: "Lisa".to_string()
    });
    let response = api.clone().send(request).await.unwrap().into_inner();
    write_html(|out| {
        index_html(out, &response.message, &["test item"]).unwrap()
    })
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
