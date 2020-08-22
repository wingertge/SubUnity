use crate::authentication::User;
use rocket::response::Redirect;

#[get("/profile")]
pub async fn profile(_user: User) -> &'static str {
    "you're logged in!"
}

#[get("/profile", rank = 2)]
pub async fn profile_unauthorized() -> Redirect {
    Redirect::to(uri!(crate::authentication::authorize))
}
