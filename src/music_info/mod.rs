pub mod song;
pub mod youtube;

pub const YOUTUBE_BASE_URL: &str = "https://www.youtube.com";
pub const YOUTUBE_SEARCH_BASE_URL: &str = "https://www.youtube.com/youtubei/v1/search?key=";
pub const YOUTUBE_WATCH_BASE_URL: &str = "https://www.youtube.com/watch?v=";
pub const INNERTUBE_FALLBACK_KEY: &str = "AIzaSyAReplaceWithCurrentWebKey";
pub const MAX_SONG_DURATION_SECONDS: u64 = 600;
pub const MAX_TIME_SLICES: usize = 2;
pub const MAX_SEARCH_RESULTS: usize = 5;

pub fn parse_duration_to_secs(time: &str) -> u64 {
    time.split(":")
        .fold(0, |acc, part| acc * 60 + part.parse::<u64>().unwrap_or(0))
}

pub fn check_song_length(time: &str) -> bool {
    let split = time.split(":").collect::<Vec<_>>();

    if split.len() <= MAX_TIME_SLICES {
        return true;
    }

    false
}
