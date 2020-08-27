use crate::authentication::{User, unauthorized_redirect};
use rocket::response::Redirect;
use crate::{template, Template};
use crate::templates::profile_html;
use rocket::http::CookieJar;

#[get("/profile")]
pub async fn profile(user: User) -> Template {
    template(|w| profile_html(w, user, ""))
}

#[get("/profile", rank = 2)]
pub async fn profile_unauthorized(cookies: &CookieJar<'_>) -> Redirect {
    unauthorized_redirect(uri!(profile_unauthorized), cookies)
}
