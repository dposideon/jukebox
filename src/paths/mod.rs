use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::libs::{FFMPEG, LIBRARY_DIR, YTDLP};
use crate::player::OUTPUT_DIR;

pub mod deps;

static BASE_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn init_base_dir() {
    let home = match std::env::var("JUKEBOX_HOME") {
        Ok(h) => {
            let p = PathBuf::from(&h);
            if !p.is_dir() {
                panic!("JUKEBOX_HOME is set to {}, but it does not exist.", h);
            }
            p
        }
        Err(_) => dirs::home_dir().expect("Failed to determine home directory."),
    };

    let base = if cfg!(target_os = "macos") {
        home.join("Library")
            .join("Application Support")
            .join("jukebox")
    } else {
        match std::env::var("XDG_DATA_HOME") {
            Ok(xdg) => PathBuf::from(xdg).join("jukebox"),
            Err(_) => home.join(".local").join("share").join("jukebox"),
        }
    };

    BASE_DIR
        .set(base)
        .expect("base directory already initialized")
}

pub fn base_dir() -> &'static Path {
    BASE_DIR.get().expect("base directory not initialized")
}

pub fn libs_dir() -> PathBuf {
    base_dir().join(LIBRARY_DIR)
}

pub fn output_dir() -> PathBuf {
    base_dir().join(OUTPUT_DIR)
}

pub fn ytdlp_exe() -> PathBuf {
    libs_dir().join(YTDLP)
}

pub fn ffmpeg_exe() -> PathBuf {
    libs_dir().join(FFMPEG)
}
