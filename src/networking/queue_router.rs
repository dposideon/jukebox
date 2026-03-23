use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::{
    music_info::{
        song::{Song, SongResponse},
        youtube::get_search_results,
    },
    networking::AppState,
};

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub async fn get_queue(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let q: Vec<SongResponse> = state
        .queue
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .iter()
        .map(SongResponse::from)
        .collect();
    Ok(Json(q))
}

pub async fn get_downloaded_queue(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let q: Vec<SongResponse> = state
        .downloaded_queue
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .iter()
        .map(SongResponse::from)
        .collect();
    Ok(Json(q))
}

pub async fn get_now_playing(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let np = state
        .now_playing
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SongResponse::from(&*np)))
}

pub async fn search_api(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<Song>>, StatusCode> {
    get_search_results(&params.q, &state.inner_tube_config, state.client.clone())
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn add_to_queue(State(state): State<AppState>, Json(song): Json<Song>) -> StatusCode {
    if let Ok(mut q) = state.queue.lock() {
        q.push_back(song);
    } else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    match state
        .queue_tx
        .send(crate::player::queue::QueueCommand::Add)
        .await
    {
        Ok(_) => {
            println!("Sent queue Command Add");
        }
        Err(e) => {
            println!("Error in queue command {}", e);
        }
    }

    StatusCode::OK
}
