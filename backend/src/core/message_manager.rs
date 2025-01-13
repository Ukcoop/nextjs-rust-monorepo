use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

use crate::services::database::Database;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Message {
    pub username: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct Messages {
    pub messages: Vec<Message>,
}

pub async fn get_messages(db: &dyn Database) -> Result<Messages, Error> {
    let rows = db.read_db("SELECT username, message FROM messages").await?;

    return Ok(Messages { messages: rows });
}

pub async fn post_message(message: Json<Message>, db: &dyn Database) -> Result<(), Error> {
    let data = vec![message.username.clone(), message.message.clone()];

    db.write_db(
        "INSERT INTO messages (username, message) VALUES ($1, $2)",
        data,
    )
    .await?;

    return Ok(());
}
