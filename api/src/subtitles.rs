use api_types::subtitles::video_subs_server::VideoSubs;
use tonic::{Status, Response, Request};
use api_types::subtitles::{Subtitles, SetSubtitleResponse};
use diesel::RunQueryDsl;
use crate::{State, IntoStatus};
use std::ops::Deref;
use crate::db::models::NewSubtitles;
use async_trait::async_trait;

struct VideoSubService(State);

impl Deref for VideoSubService {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl VideoSubs for VideoSubService {
    async fn set_subtitles(&self, request: Request<Subtitles>) -> Result<Response<SetSubtitleResponse>, Status> {
        use crate::db::schema::subtitles::dsl::*;

        let req = request.into_inner();
        let json = serde_json::to_string(&req.entries)
            .map_err(|_| Status::invalid_argument("Can't serialize to JSON"))?;

        let new_subs = NewSubtitles {
            video_id: &req.video_id,
            language: &req.language,
            subs_json: &json
        };
        let conn = self.db()?;
        diesel::insert_into(subtitles)
            .values(&new_subs)
            .execute(&conn)
            .into_status()?;
        Ok(Response::new(SetSubtitleResponse {}))
    }
}