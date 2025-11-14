use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};

use crate::{db, error::AppError, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/{short_code}", get(redirect_to_url))
}

async fn redirect_to_url(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let record = db::find_by_short_code(&state.pool, &short_code).await?;

    db::increment_click_count(&state.pool, &short_code).await?;

    tracing::info!("Redirecting {} to {}", short_code, record.original_url);

    Ok(Redirect::permanent(&record.original_url))
}
