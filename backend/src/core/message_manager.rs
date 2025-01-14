use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

use crate::services::database::Database;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug, PartialEq)]
pub struct Message {
    pub username: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::{get_messages, post_message, Message};
    use crate::{init_db, services::database::DbWrapper};
    use actix_web::web::Json;

    async fn initialize_test_db() -> DbWrapper {
        return init_db(true)
            .await
            .unwrap_or_else(|e| panic!("Error: failed to initialize database. {}", e));
    }

    fn create_test_message() -> Message {
        return Message {
            username: "testy".to_string(),
            message: "Hello, im testy!".to_string(),
        };
    }

    async fn post_test_message(db: &DbWrapper, message: Message) {
        return post_message(Json(message), db)
            .await
            .unwrap_or_else(|e| panic!("Error: failed to post message. {}", e));
    }

    #[tokio::test]
    async fn test_post_message() {
        let db = initialize_test_db().await;
        let test_message = create_test_message();
        post_test_message(&db, test_message).await;
    }

    #[tokio::test]
    async fn test_get_message() {
        let db = initialize_test_db().await;
        let test_message = create_test_message();
        post_test_message(&db, test_message.clone()).await;

        match get_messages(&db).await {
            Ok(result) => {
                assert_eq!(result.messages[0], test_message)
            }
            Err(e) => {
                panic!("Error: failed to get messages. {}", e)
            }
        }
    }
}
