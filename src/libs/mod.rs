//TODO add checks for libs and reset logic in main

pub mod ffmpeg;
pub mod ytdlp;

use std::env::consts::{ARCH, OS};

pub const LIBRARY_DIR: &str = "libs";
pub const YTDLP: &str = "ytdlp";
pub const FFMPEG: &str = "ffmpeg";

pub enum Os {
    Linux,
    Mac,
}

pub enum Arch {
    X86,
    Aarch64,
}

pub struct BinaryUrls {
    pub os: Os,
    pub arch: Arch,
    pub ytdlp: &'static str,
    pub ffmpeg: &'static str,
    pub ytdlp_filename: &'static str,
    pub ffmpeg_filename: &'static str,
}

pub fn get_binary_urls() -> Result<BinaryUrls, String> {
    match (OS, ARCH) {
        ("linux", "x86_64") => Ok(BinaryUrls {
            os: Os::Linux,
            arch: Arch::X86,
            ytdlp: "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux",
            ffmpeg: "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz",
            ytdlp_filename: "yt-dlp",
            ffmpeg_filename: "ffmpeg-master-latest-linux64-gpl.tar.xz",
        }),
        ("linux", "aarch64") => Ok(BinaryUrls {
            os: Os::Linux,
            arch: Arch::Aarch64,
            ytdlp: "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux_aarch64",
            ffmpeg: "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linuxarm64-gpl.tar.xz",
            ytdlp_filename: "yt-dlp",
            ffmpeg_filename: "ffmpeg-master-latest-linuxarm64-gpl.tar.xz",
        }),
        ("macos", "x86_64") => Ok(BinaryUrls {
            os: Os::Mac,
            arch: Arch::X86,
            ytdlp: "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos",
            ffmpeg: "https://evermeet.cx/ffmpeg/getrelease/zip",
            ytdlp_filename: "yt-dlp",
            ffmpeg_filename: "ffmpeg.zip",
        }),
        ("macos", "aarch64") => Ok(BinaryUrls {
            os: Os::Mac,
            arch: Arch::Aarch64,
            ytdlp: "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos",
            ffmpeg: "https://evermeet.cx/ffmpeg/getrelease/zip",
            ytdlp_filename: "yt-dlp",
            ffmpeg_filename: "ffmpeg.zip",
        }),
        _ => Err(format!("Unsupported platform: {OS} {ARCH}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::{
        ffmpeg::{delete_ffmpeg, download_ffmpeg},
        ytdlp::{delete_ytdlp, download_ytdlp},
    };

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_ytdlp_download() {
        let info = get_binary_urls().unwrap();
        let path = download_ytdlp(info.ytdlp, LIBRARY_DIR).await.unwrap();

        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);

        let output = std::process::Command::new(&path)
            .arg("--version")
            .output()
            .unwrap();
        assert!(output.status.success());

        delete_ytdlp(LIBRARY_DIR).unwrap();
        assert!(!path.exists());
    }

    #[tokio::test]
    #[ignore]
    async fn test_ffmpeg_download() {
        let info = get_binary_urls().unwrap();
        let path = download_ffmpeg(LIBRARY_DIR, info.ffmpeg, &info.os)
            .await
            .unwrap();

        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);

        let output = std::process::Command::new(&path)
            .arg("-version")
            .output()
            .unwrap();

        assert!(output.status.success());

        delete_ffmpeg(LIBRARY_DIR).unwrap();
        assert!(!path.exists());
    }
}
