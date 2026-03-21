use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::networking::{AppState, STATIC_DIR};

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

pub async fn admin() -> Html<&'static str> {
    Html(include_str!("../../static/admin.html"))
}

pub async fn handler_404() -> (axum::http::StatusCode, Html<&'static str>) {
    (
        axum::http::StatusCode::NOT_FOUND,
        Html(include_str!("../../static/404.html")),
    )
}
