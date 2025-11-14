pub mod health;
pub mod redirect;
pub mod shorten;

use axum::Router;
use crate::state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(health::router())
        .merge(shorten::router())
        .merge(redirect::router())
}

