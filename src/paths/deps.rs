use crate::{
    libs::{ffmpeg::download_ffmpeg, get_binary_urls, ytdlp::download_ytdlp},
    player::clean_old_output,
};

use super::*;

fn ensure_dirs() -> std::io::Result<()> {
    std::fs::create_dir_all(libs_dir())?;
    std::fs::create_dir_all(output_dir())?;
    Ok(())
}

pub async fn init_files() -> Result<(), Box<dyn std::error::Error>> {
    let system_info = get_binary_urls()?;
    ensure_dirs()?;

    match clean_old_output() {
        Ok(_) => {}
        Err(_) => {
            println!("Error deleting old tracks");
        }
    }

    if !ffmpeg_exe().exists() {
        println!("Downloading FFMPEG, this may take a while.");
        download_ffmpeg(&libs_dir(), system_info.ffmpeg, &system_info.os).await?;
        println!("Successfully downloaded FFMPEG");
    }

    if ytdlp_exe().exists() {
        println!("Updating yt-dlp, this may take a while.");
        download_ytdlp(system_info.ytdlp, &ytdlp_exe()).await?;
    } else {
        println!("Downloading yt-dlp, this may take a while.");
        download_ytdlp(system_info.ytdlp, &ytdlp_exe()).await?;
    }

    println!("Successfully downloaded yt-dlp");

    Ok(())
}
