use super::*;

use std::path::Path;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub type NowPlaying = Arc<Mutex<Song>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    pub title: String,
    pub duration: u64,
    pub link: String,
    pub channel: String,
    pub views: String,
    pub path: Option<String>,
}

impl Default for Song {
    fn default() -> Self {
        Song {
            title: "No Songs Queued".to_string(),
            duration: 0u64,
            link: "No link".to_string(),
            channel: "JUKEBOX2".to_string(),
            views: "69420".to_string(),
            path: None,
        }
    }
}

#[derive(Serialize)]
pub struct SongResponse {
    pub title: String,
    pub duration: u64,
    pub channel: String,
    pub views: String,
}

impl From<&Song> for SongResponse {
    fn from(song: &Song) -> Self {
        SongResponse {
            title: song.title.clone(),
            duration: song.duration,
            channel: song.channel.clone(),
            views: song.views.clone(),
        }
    }
}
