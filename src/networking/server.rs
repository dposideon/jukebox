use super::*;
use crate::networking::{
    page_router::{admin, handler_404, index, qr_handler},
    player_router::{pause, play, skip},
    queue_router::{add_to_queue, get_downloaded_queue, get_now_playing, get_queue, search_api},
    AppState, STATIC_DIR,
};

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn create_server(state: AppState) {
    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .expect("Unable to bind listener");

    let app = Router::new()
        .route("/", get(index))
        .route("qr.png", get(qr_handler))
        .route("/api/search", get(search_api))
        .route("/api/queue", get(get_queue))
        .route("/api/queue/add", post(add_to_queue))
        .route("/api/now_playing", get(get_now_playing))
        .route("/api/downloaded_queue", get(get_downloaded_queue))
        .route("/admin", get(admin))
        .route("/admin/api/play", post(play))
        .route("/admin/api/pause", post(pause))
        .route("/admin/api/skip", post(skip))
        .with_state(state)
        .fallback(handler_404);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Unable to create App");
}
