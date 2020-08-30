-- Your SQL goes here
CREATE TABLE subtitles(
    id integer primary key,
    video_id varchar(255) unique not null,
    language varchar(10) not null,
    subs_json text not null
)