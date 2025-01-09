use std::io;

use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use sqlx::{Pool, Postgres};

pub fn convert_sqlx_error<T>(result: Result<T, Error>) -> Result<T, io::Error> {
    result.map_err(|e| match e {
        Error::Io(io_error) => io_error,
        _ => io::Error::new(io::ErrorKind::Other, format!("error: {}", e)),
    })
}

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(result) => result,
        Err(_) => "postgres://postgres:mysecretpassword@localhost/postgres".to_string(),
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL,
            message TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
