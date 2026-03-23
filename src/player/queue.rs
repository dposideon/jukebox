use super::*;
use crate::music_info::song::{NowPlaying, Song};
use crate::music_info::youtube::download_as_mp3;
use crate::player::sink::{cycle_player, ProtectedPlayer};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;

pub const MAX_QUEUE_DEPTH: usize = 10;

pub type Queue = Arc<Mutex<VecDeque<Song>>>;

pub enum QueueCommand {
    Add,
    Empty,
}

pub fn new_queue() -> Queue {
    Arc::new(Mutex::new(VecDeque::new()))
}

fn get_next_song(queue: Queue) -> Option<Song> {
    queue.lock().ok().and_then(|mut q| q.pop_front())
}

pub fn add_to_downloaded_queue(queue: Queue, downloaded_queue: Queue, counter: &mut usize) {
    if let Some(mut song) = get_next_song(queue) {
        let output_path = output_dir().join(format!("track_{}.mp3", counter));

        match download_as_mp3(&song.link, &output_path) {
            Ok(()) => {
                song.path = Some(output_path);
                if let Ok(mut dq) = downloaded_queue.lock() {
                    dq.push_back(song);
                } else {
                    println!("Downloader Message: Error\nTHE DLQ LOCK IS POISONED!");
                }
                println!(
                    "Downloader Message: Download Success\nPath: track_{}.mp3",
                    counter,
                );

                if *counter < MAX_QUEUE_DEPTH {
                    *counter += 1;
                } else {
                    *counter = 0;
                }
            }
            Err(e) => {
                println!("Downloader Message: Error\nError: {}", e);
            }
        }
    }
}

pub async fn queue_worker(
    mut queue_rx: mpsc::Receiver<QueueCommand>,
    queue: Queue,
    downloaded_queue: Queue,
    now_playing: NowPlaying,
    player: ProtectedPlayer,
) {
    let mut counter: usize = 0;

    while let Some(command) = queue_rx.recv().await {
        match command {
            QueueCommand::Add => {
                let should_proceed = {
                    let q1 = queue.lock().ok();
                    let q2 = downloaded_queue.lock().ok();

                    match (q1, q2) {
                        (Some(q1), Some(q2)) => !q1.is_empty() && q2.len() < MAX_QUEUE_DEPTH + 1,
                        _ => false,
                    }
                };

                if should_proceed {
                    add_to_downloaded_queue(queue.clone(), downloaded_queue.clone(), &mut counter);
                }
            }
            QueueCommand::Empty => {
                if let Some(next_song) = get_next_song(downloaded_queue.clone()) {
                    cycle_player(next_song, now_playing.clone(), player.clone());
                    add_to_downloaded_queue(queue.clone(), downloaded_queue.clone(), &mut counter);
                }
            }
        }
    }
}
