use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::config::Config;

pub async fn connect_db(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await?;
    
    tracing::info!("Connected to database");
    
    Ok(pool)
}
