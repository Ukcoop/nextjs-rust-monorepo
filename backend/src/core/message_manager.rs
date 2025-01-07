use serde::{Deserialize, Serialize};
use sqlx::{self, query, query_as, FromRow, Pool, Postgres};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Message {
    pub message: String,
}

#[derive(Serialize)]
pub struct Messages {
    pub messages: Vec<Message>,
}

pub async fn get_messages(db: &Pool<Postgres>) -> Result<Messages, sqlx::Error> {
    let rows = query_as::<_, Message>("SELECT message FROM messages")
        .fetch_all(db)
        .await?;

    Ok(Messages { messages: rows })
}

pub async fn post_message(message_str: String, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    query("INSERT INTO messages (message) VALUES ($1)")
        .bind(message_str)
        .execute(db)
        .await?;

    Ok(())
}