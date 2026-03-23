use std::io::BufReader;
use std::sync::{Arc, Mutex};

use rodio::{Decoder, MixerDeviceSink, Player};
use tokio::sync::mpsc;

use crate::{
    music_info::song::{NowPlaying, Song},
    player::queue::QueueCommand,
};

pub type ProtectedPlayer = Arc<Mutex<Player>>;

pub fn create_player() -> Result<(MixerDeviceSink, ProtectedPlayer), Box<dyn std::error::Error>> {
    let handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(handle.mixer());

    Ok((handle, Arc::new(Mutex::new(player))))
}

pub async fn player_poll(queue_tx: mpsc::Sender<QueueCommand>, player: ProtectedPlayer) {
    loop {
        let is_empty = player.lock().is_ok_and(|g| g.empty());

        if is_empty {
            match queue_tx.send(QueueCommand::Empty).await {
                Ok(()) => {
                    println!("Player Poll Message: Empty");
                }
                Err(e) => {
                    println!("Player Poll Message: Error\nError: {}", e);
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(4)).await;
    }
}

pub fn cycle_player(next_song: Song, now_playing: NowPlaying, player: ProtectedPlayer) {
    if let Some(path) = &next_song.path {
        match std::fs::File::open(path) {
            Ok(file) => match Decoder::new(BufReader::new(file)) {
                Ok(source) => {
                    if let Ok(p) = player.lock() {
                        p.append(source);
                    }
                    if let Ok(mut np) = now_playing.lock() {
                        *np = next_song;
                    }
                }
                Err(e) => {
                    println!("Player Cycle Message: Error\nError: {}", e);
                }
            },
            Err(e) => {
                println!("Player Cycle Message: Error\nError: {}", e);
            }
        }
    }
}
