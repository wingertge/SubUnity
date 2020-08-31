-- Your SQL goes here
CREATE TABLE subtitles(
    video_id varchar(255) not null,
    language varchar(10) not null,
    subs_json text not null,
    primary key (video_id, language)
)