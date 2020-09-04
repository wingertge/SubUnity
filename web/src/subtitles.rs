use crate::AuthAPI;
use rocket_contrib::json::Json;
use api_types::subtitles::{Subtitles, SubtitleId, DownloadRequest};
use rocket::response::status::BadRequest;
use rocket::response::Stream;
use api_types::subtitles::download_request::Format;
use rocket::futures::{TryStreamExt, io};
use rocket::futures::io::{ErrorKind};
use tokio::prelude::AsyncRead;
use tokio_util::compat::FuturesAsyncReadCompatExt;

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

#[get("/download/<video_id>?<lang>")]
pub async fn download_subtitles(api: AuthAPI<'_>, video_id: String, lang: String) -> Stream<impl AsyncRead> {
    let res = api.subtitles().download_subtitles(DownloadRequest {
        video_id,
        language: lang,
        format: Format::Srt as i32
    }).await.unwrap().into_inner().map_err(|err| {
        io::Error::new(ErrorKind::Other, err.message())
    });
    Stream::chunked(res.into_async_read().compat(), 1024)
}