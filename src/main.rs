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
        local_ip::get_local_ip,
        mdns::register_mdns,
        qr::{create_qr_code, EMPTY_PNG},
        server::create_server,
        AppState,
    },
    paths::{deps::init_files, init_base_dir},
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
mod paths;
mod player;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a == "--version") {
        println!("jukebox {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    init_base_dir();
    init_files().await?;

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

    println!("Listening on {}:80", &local_ip);

    let qr = match create_qr_code(format!("{}:{}", &local_ip, 80).as_str()) {
        Ok(code) => {
            println!("QR code created");
            code
        }
        Err(_) => {
            println!("Error generating qr code");
            EMPTY_PNG.to_vec()
        }
    };

    let app_state = AppState {
        queue_tx: queue_tx.clone(),
        queue: queue.clone(),
        downloaded_queue: downloaded_queue.clone(),
        now_playing: now_playing.clone(),
        player: player.clone(),
        qr,
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
