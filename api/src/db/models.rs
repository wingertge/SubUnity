use super::schema::users;
use super::schema::subtitles;

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
pub struct Subtitles {
    pub id: u32,
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