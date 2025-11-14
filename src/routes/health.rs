use axum::{routing::get, Router};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/ping", get(health_check))
}

async fn health_check() -> &'static str {
    "pong"
}

