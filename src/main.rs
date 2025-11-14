mod config;
mod db;
mod error;
mod routes;
mod state;
mod utils;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::Config,
    routes::create_router,
    state::AppState,
    utils::connect_db,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "shortty=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let pool = connect_db(&config).await?;

    let app_state = AppState {
        pool,
        config: config.clone(),
    };

    let app = create_router().with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
        .await?;

    tracing::info!("Server starting on port {}", config.server_port);

    axum::serve(listener, app).await?;

    Ok(())
}
