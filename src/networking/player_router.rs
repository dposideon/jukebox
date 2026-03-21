use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{networking::AppState, player::queue::Queue};

pub async fn pause(State(state): State<AppState>) {
    if let Ok(p) = state.player.lock() {
        p.pause();
    }
}

pub async fn play(State(state): State<AppState>) {
    if let Ok(p) = state.player.lock() {
        p.play();
    }
}

pub async fn skip(State(state): State<AppState>) {
    if let Ok(p) = state.player.lock() {
        p.skip_one();
    }
}
