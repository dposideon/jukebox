use crate::paths::{libs_dir, ytdlp_exe};

use super::song::Song;
use super::*;

use std::path::Path;
use std::process::Command;

use regex::Regex;
use reqwest::Client;

#[derive(Clone)]
pub struct InnerTubeConfig {
    api_key: String,
    client_version: String,
}

impl InnerTubeConfig {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0")
            .build()?;
        let html = client.get(YOUTUBE_BASE_URL).send().await?.text().await?;

        let api_key = match get_innertube_api_key(&html) {
            Ok(key) => key,
            Err(e) => {
                println!("{}\nUsing Fallback", e);
                INNERTUBE_FALLBACK_KEY.to_string()
            }
        };
        let client_version = get_innertube_client_version(&html)?;

        Ok(InnerTubeConfig {
            api_key,
            client_version,
        })
    }
}

pub async fn get_search_results(
    query: &str,
    innertube_config: &InnerTubeConfig,
    client: Client,
) -> Result<Vec<Song>, Box<dyn std::error::Error>> {
    let url = format!("{}{}", YOUTUBE_SEARCH_BASE_URL, innertube_config.api_key);

    let body = serde_json::json!({
        "context": {
            "client": {
                "clientName": "WEB",
                "clientVersion": innertube_config.client_version
            }
        },
        "query": query
    });

    let response: serde_json::Value = client.post(&url).json(&body).send().await?.json().await?;

    let mut songs: Vec<Song> = Vec::with_capacity(MAX_SEARCH_RESULTS);

    parse_search_results(response, &mut songs);

    Ok(songs)
}

fn parse_search_results(response: serde_json::Value, songs: &mut Vec<Song>) {
    let sections = &response["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
        ["sectionListRenderer"]["contents"];

    for section in sections.as_array().unwrap_or(&vec![]) {
        let items = &section["itemSectionRenderer"]["contents"];

        for item in items.as_array().unwrap_or(&vec![]) {
            let video = &item["videoRenderer"];

            if video.is_null() {
                continue;
            }

            let duration_text = video["lengthText"]["simpleText"].as_str().unwrap_or("");

            if duration_text.is_empty() {
                continue;
            }

            if !check_song_length(duration_text) {
                continue;
            }

            let duration_secs = parse_duration_to_secs(duration_text);

            if duration_secs > MAX_SONG_DURATION_SECONDS {
                continue;
            }

            let title = video["title"]["runs"][0]["text"]
                .as_str()
                .unwrap_or("")
                .to_string();

            let channel = video["ownerText"]["runs"][0]["text"]
                .as_str()
                .unwrap_or("")
                .to_string();

            let video_id = video["videoId"].as_str().unwrap_or("dQw4w9WgXcQ");

            let link = format!("{}{}", YOUTUBE_WATCH_BASE_URL, video_id);

            let views = video["viewCountText"]["simpleText"]
                .as_str()
                .unwrap_or("")
                .to_string();

            songs.push(Song {
                title,
                duration: duration_secs,
                link,
                channel,
                views,
                path: None,
            });

            if songs.len() >= MAX_SEARCH_RESULTS {
                break;
            }
        }
    }
}

pub fn download_as_mp3(url: &str, output_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new(ytdlp_exe())
        .arg("--ffmpeg-location")
        .arg(libs_dir())
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg(output_file)
        .arg(url)
        .status()?;

    if !status.success() {
        return Err(format!("yt-dlp exited with {}", status).into());
    }

    Ok(())
}

fn get_innertube_api_key(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r#""INNERTUBE_API_KEY":"([^"]+)""#)?;

    if let Some(caps) = re.captures(html) {
        Ok(caps[1].to_string())
    } else {
        Err("INNERTUBE_API_KEY not found".into())
    }
}

fn get_innertube_client_version(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r#""INNERTUBE_CLIENT_VERSION":"([^"]+)""#)?;

    if let Some(caps) = re.captures(html) {
        Ok(caps[1].to_string())
    } else {
        Err("INNERTUBE_CLIENT_VERSION not found".into())
    }
}
