# shortty

A fast URL shortener built with Rust and Axum.

## Setup

1. Create a PostgreSQL database and run the migration:
```sql
CREATE TABLE IF NOT EXISTS urls (
    id BIGSERIAL PRIMARY KEY,
    short_code VARCHAR(255) UNIQUE NOT NULL,
    original_url TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    click_count BIGINT DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_short_code ON urls(short_code);
CREATE INDEX IF NOT EXISTS idx_original_url ON urls(original_url);
```

2. Create a `.env` file:
```bash
DB_URL=postgresql://user:password@localhost/shortty
BASE_URL=http://localhost:3000
PORT=3000
```

3. Run:
```bash
cargo run
```

## API

### Create Short URL
`POST /shorten`

Request:
```json
{
  "url_to_shorten": "https://example.com"
}
```

Response:
```json
{
  "short_code": "ac6bb669",
  "short_url": "http://localhost:3000/ac6bb669",
  "original_url": "https://example.com"
}
```

### Redirect
`GET /{short_code}`

Redirects to the original URL (308 Permanent Redirect).

### Health Check
`GET /ping`

Returns `pong`.

## Environment Variables

- `DB_URL` - PostgreSQL connection string (required)
- `BASE_URL` - Base URL for shortened links (default: `http://localhost:3000`)
- `PORT` - Server port (default: `3000`)
- `MAX_CONNECTIONS` - Database connection pool size (default: `10`)
- `HASH_LENGTH` - Length of short code (default: `8`)
