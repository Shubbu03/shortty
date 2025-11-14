use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{
    db,
    error::AppError,
    state::AppState,
    utils::{is_valid_domain, normalize_url},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/shorten", post(shorten_url))
}

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url_to_shorten: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
}

async fn shorten_url(
    State(state): State<AppState>,
    Json(input): Json<ShortenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let url = input.url_to_shorten.trim();

    if url.is_empty() {
        return Err(AppError::InvalidUrl("URL cannot be empty".to_string()));
    }

    if url.len() > 2048 {
        return Err(AppError::InvalidUrl(
            "URL too long (max 2048 characters)".to_string(),
        ));
    }

    let normalized_url = normalize_url(url)
        .map_err(|e| AppError::InvalidUrl(format!("Failed to normalize URL: {}", e)))?;

    if !is_valid_domain(&normalized_url) {
        return Err(AppError::InvalidUrl("Invalid domain format".to_string()));
    }

    if let Some(existing) = db::find_by_original_url(&state.pool, &normalized_url).await? {
        tracing::info!("Returning existing short code for URL: {}", normalized_url);
        let short_url = format!("{}/{}", state.config.base_url, existing.short_code);
        return Ok((
            StatusCode::OK,
            Json(ShortenResponse {
                short_code: existing.short_code,
                short_url,
                original_url: existing.original_url,
            }),
        ));
    }

    let record = db::create_url_with_collision_handling(
        &state.pool,
        &normalized_url,
        state.config.hash_length,
    )
    .await?;

    let short_url = format!("{}/{}", state.config.base_url, record.short_code);
    tracing::info!(
        "Created short code {} for URL: {}",
        record.short_code,
        normalized_url
    );

    Ok((
        StatusCode::CREATED,
        Json(ShortenResponse {
            short_code: record.short_code,
            short_url,
            original_url: record.original_url,
        }),
    ))
}
