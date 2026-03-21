#![allow(unused)]
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    libs::{
        ffmpeg::download_ffmpeg,
        get_binary_urls,
        ytdlp::{delete_ytdlp, download_ytdlp},
        LIBRARY_DIR,
    },
    music_info::{
        song::{NowPlaying, Song},
        youtube::{download_as_mp3, get_search_results, InnerTubeConfig},
    },
    networking::{
        local_ip::get_local_ip, mdns::register_mdns, qr::create_qr_code, server::create_server,
        AppState,
    },
    player::{
        clean_old_output,
        queue::{new_queue, queue_worker},
        sink::{create_player, player_poll},
        OUTPUT_DIR,
    },
};

mod libs;
mod music_info;
mod networking;
mod player;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from(OUTPUT_DIR);
    let lib_dir = PathBuf::from(LIBRARY_DIR);

    if !output_dir.exists() {
        println!("Creating Output Directory");
        std::fs::create_dir(output_dir)?;
    } else {
        print!("Deleting old tracks");
        match clean_old_output() {
            Ok(_) => {
                println!("Clean.");
            }
            Err(_) => {
                println!("Old tracks might still be there");
            }
        }
    }

    if !lib_dir.exists() {
        println!("Creating Library Directory");
        std::fs::create_dir(&lib_dir)?;
        let system_info = get_binary_urls()?;
        println!("Downloading FFMPEG, this may take a while.");
        download_ffmpeg(
            &lib_dir.to_string_lossy(),
            system_info.ffmpeg,
            &system_info.os,
        )
        .await?;
        println!("Downloading yt-dlp, this may take a while.");
        download_ytdlp(system_info.ytdlp, &lib_dir.to_string_lossy()).await?;
        println!("Successfully Installed Libraries");
    } else {
        println!("Updating yt-dlp, this may take a while.");
        match delete_ytdlp(&lib_dir.to_string_lossy()) {
            Ok(_) => {
                println!("Deleted Old ytdlp");
            }
            Err(_) => {
                println!("yt-dlp wasnt found")
            }
        }
        let system_info = get_binary_urls()?;
        download_ytdlp(system_info.ytdlp, &lib_dir.to_string_lossy()).await?;
        println!("Successfully updated yt-dlp.");
    }

    let innertube_config = InnerTubeConfig::new().await?;

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()?;

    let now_playing: NowPlaying = Arc::new(Mutex::new(Song::default()));
    let queue = new_queue();
    let downloaded_queue = new_queue();

    let (_stream, player) = create_player()?;

    let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(10);

    let _service_daemon = register_mdns(80u16)?;
    let local_ip = get_local_ip()?;

    match create_qr_code(format!("{}:{}", &local_ip, 80).as_str()) {
        Ok(_) => {
            println!("QR code created");
        }
        Err(_) => {
            println!("Error generating qr code");
        }
    }

    let app_state = AppState {
        queue_tx: queue_tx.clone(),
        queue: queue.clone(),
        downloaded_queue: downloaded_queue.clone(),
        now_playing: now_playing.clone(),
        player: player.clone(),
        inner_tube_config: innertube_config.clone(),
        client: client.clone(),
    };

    tokio::spawn(queue_worker(
        queue_rx,
        queue.clone(),
        downloaded_queue.clone(),
        now_playing.clone(),
        player.clone(),
    ));
    tokio::spawn(player_poll(queue_tx.clone(), player.clone()));

    create_server(app_state).await;

    Ok(())
}
