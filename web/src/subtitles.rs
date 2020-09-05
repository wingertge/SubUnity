use crate::AuthAPI;
use rocket_contrib::json::Json;
use api_types::subtitles::{Subtitles, SubtitleId, DownloadRequest};
use rocket::response::status::BadRequest;
use rocket::response::{Stream, Responder};
use api_types::subtitles::download_request::Format;
use rocket::futures::{TryStreamExt, io};
use rocket::futures::io::{ErrorKind};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use rocket::http::{ContentType, Header};
use rocket::{Request, Response, response};

pub struct File<R>(R, ContentType, Header<'static>);

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for File<R> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        Response::build()
            .merge(self.0.respond_to(request)?)
            .header(self.1)
            .header(self.2)
            .ok()
    }
}

impl<R> File<R> {
    pub fn new(file_name: &str, inner: R) -> Self {
        let disposition = Header::new(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file_name)
        );
        Self(inner, ContentType::Binary, disposition)
    }
}

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
pub async fn download_subtitles(api: AuthAPI<'_>, video_id: String, lang: String) -> impl Responder<'_, '_> {
    let file_name = format!("subs_{}_{}.srt", video_id, lang);
    let res = api.subtitles().download_subtitles(DownloadRequest {
        video_id,
        language: lang,
        format: Format::Srt as i32
    }).await.unwrap().into_inner().map_err(|err| {
        io::Error::new(ErrorKind::Other, err.message())
    });
    let stream = Stream::chunked(res.into_async_read().compat(), 1024);

    File::new(&file_name, stream)
}