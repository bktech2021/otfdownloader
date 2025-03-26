#![warn(clippy::pedantic)]

use anyhow::Result;
use m3u8_rs::MediaPlaylist;
use std::path::Path;
use uris::Uri;

mod db;
mod download;

use db::FileDB;
use download::{fetch_url, fetch_url_to_file};

fn check_file(file_name: &str) -> bool {
    Path::new(file_name).exists()
}

fn get_file_name_from_segment(race_name: &str, segment: &m3u8_rs::MediaSegment) -> String {
    format!(
        "parts/{}-{}",
        race_name,
        segment.uri.split('/').last().unwrap()
    )
}

static RACE_NAME: &str = "race_name_unimplemented";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let link = download::iframe_link("https://overtakefans.com/f1-race-archive/watch/index.php?race=2025-chinese-grand-prix-formula-1-sprint").await.unwrap();
    let m3u8_link = download::m3u8_link(&link).await.unwrap();

    // TODO: variable names
    let uri = Uri::parse(link).unwrap();
    let hostname = String::from_utf8(uri.authority().unwrap().host().to_vec()).unwrap();
    let mut m3u8_releative: Vec<Vec<u8>> = m3u8_link.split('/').map(|a| {a.as_bytes().to_vec()}).collect();
    let mut video_releative: Vec<Vec<u8>> = uri.path().clone();
    video_releative.pop();
    video_releative.append(&mut m3u8_releative);
    let video_str: String = video_releative.iter().map(|arr| String::from_utf8(arr.clone()).unwrap()).collect::<Vec<String>>().join("/");
    let complete_link = "https://".to_string() + &hostname + &video_str;
    let playlist = fetch_url(complete_link).await.unwrap();

    video_releative.pop();
    let video_str: String = video_releative.iter().map(|arr| String::from_utf8(arr.clone()).unwrap()).collect::<Vec<String>>().join("/");
    let complete_link = "https://".to_string() + &hostname + &video_str  + "/";

    let db = FileDB::new();

    let parsed: MediaPlaylist = m3u8_rs::parse_media_playlist_res(&playlist).unwrap();

    let mut files = vec![];

    for segment in &parsed.segments {
        // TODO: implement different race names
        let file_name = get_file_name_from_segment(RACE_NAME, segment);
        let file_exists = db.check_entry_exists(&file_name, RACE_NAME);

        if file_exists && check_file(&file_name) {
            println!("skipping download for file {file_name}");
            continue;
        }

        println!("downloading {:#?}", complete_link.clone() + &segment.uri.clone());
        fetch_url_to_file(complete_link.clone() + &segment.uri.clone(), file_name.clone())
            .await
            .unwrap();
        db.new_entry(&file_name, RACE_NAME).unwrap();
        files.push(file_name);
    }
    Ok(())
}
