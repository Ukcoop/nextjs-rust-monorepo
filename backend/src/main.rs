use serde::Serialize;
use actix_cors::Cors;
use actix_web::{
    web::{Data, Json},
    {get, post, App, HttpResponse, HttpServer, Responder},
};

mod core;
mod services;

use core::message_manager::{get_messages, post_message, Message};

use services::database::{convert_sqlx_error, init_db, DbWrapper};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub struct AppState {
    pub db: DbWrapper,
}

#[get("/api/")]
async fn hello() -> impl Responder {
    let default_message = Message {
        username: "Alexander".to_string(),
        message: "This is the backend if my app, you can get messages from /api/getMessages"
            .to_string(),
    };

    return HttpResponse::Ok()
        .content_type("application/json")
        .json(default_message);
}

#[get("/api/getMessages")]
async fn api_get_messages(app_state: Data<AppState>) -> impl Responder {
    let messages = match get_messages(&app_state.db).await {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Error fetching messages: {}", e),
            });
        },
    };

    return HttpResponse::Ok()
        .content_type("application/json")
        .json(messages);
}

#[post("/api/postMessage")]
async fn api_post_message(data: Json<Message>, app_state: Data<AppState>) -> impl Responder {
    if data.message.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Message cannot be empty".to_string(),
        });
    }

    return match post_message(data, &app_state.db).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => HttpResponse::InternalServerError()
            .json(ErrorResponse {
                error: format!("Error posting message: {}", e),
            }),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = convert_sqlx_error(init_db(false).await)?;
    let shared_state = Data::new(AppState { db });

    return HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(hello)
            .service(api_get_messages)
            .service(api_post_message)
    })
    .bind(("0.0.0.0", 3080))?
    .run()
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use serde_json::json;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;

        let req = test::TestRequest::get().uri("/api/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: Message = test::read_body_json(resp).await;
        assert_eq!(body.username, "Alexander");
        assert!(body.message.contains("This is the backend"));
    }

    #[actix_web::test]
    async fn test_api_get_messages() {
        let app_state = AppState {
            db: init_db(true).await.unwrap(),
        };
        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state))
                .service(api_get_messages),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/getMessages")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_api_post_message() {
        let app_state = AppState {
            db: init_db(true).await.unwrap(),
        };
        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state))
                .service(api_post_message),
        )
        .await;

        let test_message = json!({
            "username": "testy",
            "message": "Hello, im testy!",
        });

        let req = test::TestRequest::post()
            .uri("/api/postMessage")
            .set_json(&test_message)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let error_req = test::TestRequest::post()
            .uri("/api/postMessage")
            .set_json(&json!({
                "username": "testy",
                "message": "",
            }))
            .to_request();
        let error_resp = test::call_service(&app, error_req).await;

        let body = test::read_body(error_resp).await;
        assert_eq!(body, "{\"error\":\"Message cannot be empty\"}");
    }
}
