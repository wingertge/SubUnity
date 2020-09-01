use crate::AuthAPI;
use rocket_contrib::json::Json;
use api_types::subtitles::{Subtitles, SubtitleId};
use rocket::response::status::BadRequest;

#[get("/<video_id>?<lang>")]
pub async fn get_subtitles(video_id: String, lang: String, api: AuthAPI<'_>) -> Json<Subtitles> {
    println!("Getting subtitles");
    let response = api.subtitles().get_subtitles(SubtitleId {
        video_id,
        language: lang
    }).await.unwrap().into_inner();
    Json(response)
}

#[post("/", format = "json", data = "<body>")]
pub async fn set_subtitles(api: AuthAPI<'_>, body: Json<Subtitles>) -> Result<(), BadRequest<String>> {
    api.subtitles().set_subtitles(body.into_inner())
        .await
        .map_err(|err| BadRequest(Some(err.message().to_string())))?;
    Ok(())
}