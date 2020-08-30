use crate::{
    authentication::unauthorized_redirect,
    template,
    templates::profile_html,
    Template
};
use rocket::{http::CookieJar, response::Redirect};
use crate::authentication::CurrentUser;

#[get("/profile")]
pub async fn profile(user: CurrentUser) -> Template {
    template(|w| profile_html(w, &user, ""))
}

#[get("/profile", rank = 2)]
pub async fn profile_unauthorized(cookies: &CookieJar<'_>) -> Redirect {
    unauthorized_redirect(uri!(profile_unauthorized), cookies)
}
