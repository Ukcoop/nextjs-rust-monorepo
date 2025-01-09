use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::{self, query, query_as, FromRow, Pool, Postgres};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Message {
    pub username: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct Messages {
    pub messages: Vec<Message>,
}

pub async fn get_messages(db: &Pool<Postgres>) -> Result<Messages, sqlx::Error> {
    let rows = query_as::<_, Message>("SELECT username, message FROM messages")
        .fetch_all(db)
        .await?;

    Ok(Messages { messages: rows })
}

pub async fn post_message(
    message: web::Json<Message>,
    db: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    query("INSERT INTO messages (username, message) VALUES ($1, $2)")
        .bind(message.username.clone())
        .bind(message.message.clone())
        .execute(db)
        .await?;

    Ok(())
}
