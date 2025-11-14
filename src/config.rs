use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub max_connections: u32,
    pub hash_length: usize,
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DB_URL")
                .or_else(|_| env::var("DATABASE_URL"))
                .expect("DB_URL or DATABASE_URL must be set in environment or .env file"),
            server_port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            max_connections: env::var("MAX_CONNECTIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(10),
            hash_length: env::var("HASH_LENGTH")
                .ok()
                .and_then(|l| l.parse().ok())
                .unwrap_or(8),
            base_url: env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
        }
    }
}
