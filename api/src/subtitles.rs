use api_types::subtitles::video_subs_server::VideoSubs;
use tonic::{Status, Response, Request};
use api_types::subtitles::{Subtitles, SetSubtitleResponse, SubtitleId};
use diesel::{RunQueryDsl, QueryDsl};
use crate::{State, IntoStatus, DbConnection, youtube_caption_scraper};
use std::ops::Deref;
use crate::db::models::{NewSubtitles, NewChange};
use async_trait::async_trait;
use crate::db::models;
use api_types::subtitles::subtitles::Entry;
use chrono::Utc;
use crate::user::get_user;
use serde::{Serialize, Deserialize};
use diesel::ExpressionMethods;
use std::sync::Arc;

pub struct VideoSubService(pub Arc<State>);

impl Deref for VideoSubService {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Difference {
    index: u32,
    old: Option<Entry>,
    new: Option<Entry>
}

fn diff(old: &[Entry], new: &[Entry]) -> Vec<Difference> {
    let len = old.len().max(new.len());
    (0..len)
        .map(|i| (i, old.get(i), new.get(i)))
        .filter(|(_, old, new)| old != new)
        .map(|(i, old, new)| Difference {
            index: i as u32,
            old: old.cloned(),
            new: new.cloned()
        })
        .collect()
}

async fn init_subtitles(conn: DbConnection, video_id: &str, language: &str) -> Result<models::Subtitles, Status> {
    use crate::db::schema::subtitles;

    let generated_subs = youtube_caption_scraper::get_subtitles(video_id, language).await.unwrap();
    let json = serde_json::to_string(&generated_subs.entries).unwrap();
    let new = NewSubtitles {
        video_id: &generated_subs.video_id,
        language: &generated_subs.language,
        subs_json: &json
    };
    diesel::insert_into(subtitles::table)
        .values(&new)
        .execute(&conn)
        .into_status()?;

    Ok(subtitles::table.find((video_id, language))
        .load::<models::Subtitles>(&conn)
        .into_status()?
        .pop().unwrap())
}

async fn get_or_init_subtitles(conn: DbConnection, video_id: &str, language: &str) -> Result<models::Subtitles, Status> {
    use crate::db::schema::subtitles;

    let existing: Option<models::Subtitles> = subtitles::table.find((video_id, language))
        .load::<models::Subtitles>(&conn)
        .into_status()?
        .pop();

    if let Some(existing) = existing {
        Ok(existing)
    } else {
        init_subtitles(conn, video_id, language).await
    }
}

#[async_trait]
impl VideoSubs for VideoSubService {
    async fn set_subtitles(&self, request: Request<Subtitles>) -> Result<Response<SetSubtitleResponse>, Status> {
        use crate::db::schema::{subtitles::dsl::*, changes};

        let conn = self.db()?;
        let user = get_user(&request, &conn)?;

        let req = request.into_inner();
        let existing = get_or_init_subtitles(conn, &req.video_id, &req.language).await?;
        let conn = self.db()?;

        let existing_subs = serde_json::from_str::<Vec<Entry>>(&existing.subs_json).unwrap();
        let diff = diff(&existing_subs, &req.entries);

        if diff.len() > 0 {
            let now = Utc::now().naive_utc();
            let changes_json = serde_json::to_string(&diff).unwrap();
            let new_changes = NewChange {
                timestamp: &now,
                author: &user.id,
                changes_json: &changes_json
            };
            diesel::insert_into(changes::table)
                .values(&new_changes)
                .execute(&conn)
                .into_status()?;

            let json = serde_json::to_string(&req.entries)
                .map_err(|_| Status::invalid_argument("Can't serialize to JSON"))?;

            diesel::update(&existing)
                .set(subs_json.eq(json))
                .execute(&conn)
                .into_status()?;
        }

        Ok(Response::new(SetSubtitleResponse {}))
    }

    async fn get_subtitles(&self, request: Request<SubtitleId>) -> Result<Response<Subtitles>, Status> {
        let req = request.into_inner();
        let subs = get_or_init_subtitles(self.db()?, &req.video_id, &req.language).await?;
        let entries = serde_json::from_str::<Vec<Entry>>(&subs.subs_json).unwrap();
        Ok(Response::new(Subtitles {
            entries,
            video_id: subs.video_id,
            language: subs.language
        }))
    }
}