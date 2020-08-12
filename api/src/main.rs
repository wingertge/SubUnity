#[macro_use] extern crate async_trait;

use tonic::{Response, Status, Request};
use std::error::Error;
use tonic::transport::Server;
use api_types::user::{SayRequest, SayResponse};
use api_types::user::user_server::{User, UserServer};

#[derive(Default)]
pub struct UserService;

#[async_trait]
impl User for UserService {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        Ok(Response::new(SayResponse {
            message: format!("hello {}", request.get_ref().name)
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user = UserServer::new(UserService::default());
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(user)
        .serve(addr)
        .await?;
    Ok(())
}
