use std::io;

use sqlx::{
    postgres::PgPoolOptions,
    sqlite::SqlitePoolOptions,
    {Error, Pool, Postgres, Sqlite},
};

use crate::core::message_manager::Message;

#[async_trait::async_trait]
pub trait Database: Send + Sync {
    async fn read_db<'a>(&self, query: &str) -> Result<Vec<Message>, Error>;
    async fn write_db<'a>(&self, query: &str, params: Vec<String>) -> Result<(), Error>;
}

pub enum DbWrapper {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
}

#[async_trait::async_trait]
impl Database for DbWrapper {
    async fn read_db<'a>(&self, query: &str) -> Result<Vec<Message>, Error> {
        match self {
            DbWrapper::Postgres(pool) => {
                let rows = sqlx::query_as::<_, Message>(query).fetch_all(pool).await?;
                return Ok(rows);
            }
            DbWrapper::Sqlite(pool) => {
                let rows = sqlx::query_as::<_, Message>(query).fetch_all(pool).await?;
                return Ok(rows);
            }
        }
    }

    async fn write_db<'a>(&self, query: &str, data: Vec<String>) -> Result<(), Error> {
        match self {
            DbWrapper::Postgres(pool) => {
                let mut query_builder = sqlx::query(query);
                for item in data.iter() {
                    query_builder = query_builder.bind(item);
                }
                query_builder.execute(pool).await?;
            }
            DbWrapper::Sqlite(pool) => {
                let mut query_builder = sqlx::query(query);
                for item in data.iter() {
                    query_builder = query_builder.bind(item);
                }
                query_builder.execute(pool).await?;
            }
        }

        return Ok(());
    }
}

pub fn convert_sqlx_error<T>(result: Result<T, Error>) -> Result<T, io::Error> {
    return result.map_err(|e| match e {
        Error::Io(io_error) => io_error,
        _ => io::Error::new(io::ErrorKind::Other, format!("error: {}", e)),
    });
}

pub async fn init_db(test: bool) -> Result<DbWrapper, sqlx::Error> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(result) => result,
        Err(_) => "postgres://postgres:mysecretpassword@localhost/postgres".to_string(),
    };

    let create_db_query = "
    CREATE TABLE IF NOT EXISTS messages (
        username TEXT NOT NULL,
        message TEXT NOT NULL
    )";

    let pool: DbWrapper = if test {
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(":memory:")
            .await?;

        sqlx::query(&create_db_query).execute(&sqlite_pool).await?;

        DbWrapper::Sqlite(sqlite_pool)
    } else {
        let postgres_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        sqlx::query(&create_db_query)
            .execute(&postgres_pool)
            .await?;

        DbWrapper::Postgres(postgres_pool)
    };

    return Ok(pool);
}

#[cfg(test)]
mod tests {
    use super::init_db;

    #[tokio::test]
    async fn test_init_db() {
        match init_db(true).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        }
    }
}
