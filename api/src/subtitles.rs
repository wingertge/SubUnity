use api_types::subtitles::video_subs_server::VideoSubs;
use tonic::{Status, Response, Request};
use api_types::subtitles::{Subtitles, SetSubtitleResponse};
use diesel::{RunQueryDsl, QueryDsl};
use crate::{State, IntoStatus};
use std::ops::Deref;
use crate::db::models::{NewSubtitles, NewChange};
use async_trait::async_trait;
use crate::db::models;
use api_types::subtitles::subtitles::Entry;
use chrono::Utc;
use crate::user::get_user;
use serde::{Serialize, Deserialize};
use diesel::ExpressionMethods;

struct VideoSubService(State);

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

#[async_trait]
impl VideoSubs for VideoSubService {
    async fn set_subtitles(&self, request: Request<Subtitles>) -> Result<Response<SetSubtitleResponse>, Status> {
        use crate::db::schema::{subtitles::dsl::*, changes};

        let conn = self.db()?;
        let user = get_user(&request, &conn)?;

        let req = request.into_inner();
        let mut existing: Option<models::Subtitles> = subtitles.find((&req.video_id, &req.language))
            .load::<models::Subtitles>(&conn)
            .into_status()?
            .pop();

        if existing.is_none() {
            let new = NewSubtitles {
                video_id: &req.video_id,
                language: &req.language,
                subs_json: "[]"
            };
            diesel::insert_into(subtitles)
                .values(&new)
                .execute(&conn)
                .into_status()?;
            existing.replace(models::Subtitles {
                video_id: String::new(),
                language: String::new(),
                subs_json: "[]".to_string()
            });
        }

        let existing = existing.unwrap();
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
}