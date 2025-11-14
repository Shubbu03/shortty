use crate::error::AppError;
use sqlx::{PgPool, Row};

pub struct UrlRecord {
    pub short_code: String,
    pub original_url: String,
    pub click_count: i64,
}

pub async fn find_by_short_code(pool: &PgPool, short_code: &str) -> Result<UrlRecord, AppError> {
    let row =
        sqlx::query("SELECT short_code, original_url, click_count FROM urls WHERE short_code = $1")
            .bind(short_code)
            .fetch_optional(pool)
            .await?;

    match row {
        Some(row) => Ok(UrlRecord {
            short_code: row.get("short_code"),
            original_url: row.get("original_url"),
            click_count: row.get("click_count"),
        }),
        None => Err(AppError::NotFound),
    }
}

pub async fn find_by_original_url(
    pool: &PgPool,
    original_url: &str,
) -> Result<Option<UrlRecord>, AppError> {
    let row = sqlx::query(
        "SELECT short_code, original_url, click_count FROM urls WHERE original_url = $1 LIMIT 1",
    )
    .bind(original_url)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| UrlRecord {
        short_code: row.get("short_code"),
        original_url: row.get("original_url"),
        click_count: row.get("click_count"),
    }))
}

pub async fn increment_click_count(pool: &PgPool, short_code: &str) -> Result<(), AppError> {
    sqlx::query("UPDATE urls SET click_count = click_count + 1 WHERE short_code = $1")
        .bind(short_code)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn create_url_with_collision_handling(
    pool: &PgPool,
    original_url: &str,
    hash_length: usize,
) -> Result<UrlRecord, AppError> {
    use crate::utils::create_hash;
    use sha2::Digest;

    let base_hash = create_hash(original_url, hash_length);
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 10;

    loop {
        let short_code = if attempts == 0 {
            base_hash.clone()
        } else {
            let mut hasher = sha2::Sha256::new();
            hasher.update(format!("{}{}", original_url, attempts).as_bytes());
            let hash = hex::encode(hasher.finalize());
            hash[..hash_length.min(hash.len())].to_string()
        };

        let row = sqlx::query(
            "INSERT INTO urls (short_code, original_url) VALUES ($1, $2) 
             ON CONFLICT (short_code) DO NOTHING
             RETURNING short_code, original_url, click_count",
        )
        .bind(&short_code)
        .bind(original_url)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            return Ok(UrlRecord {
                short_code: row.get("short_code"),
                original_url: row.get("original_url"),
                click_count: row.get("click_count"),
            });
        }

        let existing_url =
            sqlx::query_scalar::<_, String>("SELECT original_url FROM urls WHERE short_code = $1")
                .bind(&short_code)
                .fetch_optional(pool)
                .await?;

        if let Some(existing) = existing_url {
            if existing == original_url {
                return find_by_short_code(pool, &short_code).await;
            }
        }

        attempts += 1;
        if attempts >= MAX_ATTEMPTS {
            return Err(AppError::Internal);
        }
    }
}
