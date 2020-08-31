use super::schema::users;
use super::schema::subtitles;
use super::schema::changes;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub picture: Option<String>
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub email: Option<&'a str>
}

#[derive(Queryable, Debug)]
#[derive(Identifiable)]
#[primary_key(video_id, language)]
#[table_name = "subtitles"]
pub struct Subtitles {
    pub video_id: String,
    pub language: String,
    pub subs_json: String
}

#[derive(Insertable)]
#[table_name = "subtitles"]
pub struct NewSubtitles<'a> {
    pub video_id: &'a str,
    pub language: &'a str,
    pub subs_json: &'a str
}

#[derive(Queryable, Debug)]
pub struct Change {
    id: u32,
    time_stamp: NaiveDateTime,
    author: String,
    changes_json: String
}

#[derive(Insertable)]
#[table_name = "changes"]
pub struct NewChange<'a> {
    pub timestamp: &'a NaiveDateTime,
    pub author: &'a str,
    pub changes_json: &'a str
}