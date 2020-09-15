use regex::{Regex, RegexBuilder, Match};
use serde::Deserialize;
use htmlescape::decode_html;
use api_types::subtitles::subtitles::Entry;
use api_types::subtitles::Subtitles;
use crate::subtitles::get_video_info;
use itertools::Itertools;

#[derive(Deserialize, Debug)]
struct CaptionTracks {
    #[serde(rename = "captionTracks")]
    caption_tracks: Vec<Track>
}

#[derive(Deserialize, Debug)]
struct Track {
    #[serde(rename = "baseUrl")]
    base_url: String,
    #[serde(rename = "languageCode")]
    language_code: String
}

pub async fn get_subtitles(video_id: &str, lang: &str) -> Option<Subtitles> {
    println!("Getting subtitles");
    let client = reqwest::Client::new();

    let video_info = client.get("https://youtube.com/get_video_info")
        .query(&[("video_id", video_id)])
        .send().await.ok()?
        .text().await.ok()?;

    println!("Finding the right bits");
    let regex = Regex::new(r#"captionTracks.*?isTranslatable.*?(true|false)"#).unwrap();
    let match0 = regex.find(&video_info)?;
    println!("Decoding...");
    let tracks = urldecode::decode(match0.as_str().to_string())
        .replace("\\u0026", "&");

    // "{" + parsed_string + "}]}"
    let tracks = serde_json::from_str::<CaptionTracks>(&format!("{{\"{}}}]}}", tracks)).ok()?;
    let matching_track = tracks.caption_tracks.into_iter()
        .find(|track| &track.language_code == lang)?;

    let transcript = client.get(&matching_track.base_url)
        .send().await.ok()?
        .text().await.ok()?;

    let start_regex = Regex::new(r#"start="([\d.]+)""#).unwrap();
    let duration_regex = Regex::new(r#"dur="([\d.]+)"#).unwrap();

    let text_regex = Regex::new(r#"<text.+>"#).unwrap();
    let amp_regex = RegexBuilder::new("&amp;").case_insensitive(true).build().unwrap();
    let tag_regex = Regex::new(r#"</?[^>]+(>|$)"#).unwrap();

    fn parse_unwrap(match_: Match) -> f32 {
        match_.as_str().parse().unwrap()
    }

    println!("Parsing...");
    let entries = transcript
        .replace(r#"<?xml version="1.0" encoding="utf-8" ?><transcript>"#, "")
        .replace("</transcript>", "")
        .split("</text>")
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let start = start_regex.captures(line)?.get(1)?;
            let duration = duration_regex.captures(line)?.get(1)?;

            let html_text = text_regex.replace(line, "");
            let html_text = amp_regex.replace_all(&html_text, "&");
            let html_text = tag_regex.replace_all(&html_text, "");
            let text = decode_html(&html_text).ok()?;

            //println!("start: {}, duration: {}, text: {}", start.as_str(), duration.as_str(), text);
            Some((parse_unwrap(start), parse_unwrap(duration), text))
        })
        .map(|opt| opt.map(|(start, duration, text)| Entry {
            start_seconds: start,
            end_seconds: start + duration,
            text
        }))
        .collect::<Option<Vec<_>>>()?;
    let entries = entries.into_iter().chunks(2);

    let entries = (&entries).into_iter()
        .map(|mut pair| {
            let first = pair.next().unwrap();
            let second = pair.next();

            if let Some(second) = second {
                Entry {
                    start_seconds: first.start_seconds,
                    end_seconds: first.end_seconds,
                    text: format!("{} {}", first.text, second.text)
                }
            } else {
                first
            }
        })
        .collect();

    println!("Done!");

    let video_info = get_video_info(video_id).await.ok()?;

    Some(Subtitles {
        video_id: video_id.to_string(),
        language: lang.to_string(),
        entries,
        video_title: video_info.snippet.title,
        uploader_id: video_info.snippet.channel_id,
        uploader_name: video_info.snippet.channel_title
    })
}