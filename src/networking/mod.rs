use crate::{
    music_info::{song::NowPlaying, youtube::InnerTubeConfig},
    player::{
        queue::{Queue, QueueCommand},
        sink::ProtectedPlayer,
    },
};

use reqwest::Client;
use tokio::sync::mpsc;

pub mod local_ip;
pub mod mdns;
pub mod page_router;
pub mod player_router;
pub mod qr;
pub mod queue_router;
pub mod server;

#[derive(Clone)]
pub struct AppState {
    pub queue_tx: mpsc::Sender<QueueCommand>,
    pub queue: Queue,
    pub downloaded_queue: Queue,
    pub now_playing: NowPlaying,
    pub player: ProtectedPlayer,
    pub qr: Vec<u8>,
    pub inner_tube_config: InnerTubeConfig,
    pub client: Client,
}
