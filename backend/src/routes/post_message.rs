use actix_web::post;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};

use crate::core::message_manager::{post_message, Message};
use crate::{AppState, ErrorResponse};

#[post("/api/postMessage")]
async fn api_post_message(data: Json<Message>, app_state: Data<AppState>) -> impl Responder {
    if data.message.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Message cannot be empty".to_string(),
        });
    }

    return match post_message(data, &app_state.db).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Error posting message: {}", e),
        }),
    };
}

#[cfg(test)]
mod tests {
    use crate::services::database::init_db;

    use super::*;
    use actix_web::{test, App};
    use serde_json::json;

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

        let request = test::TestRequest::post()
            .uri("/api/postMessage")
            .set_json(&test_message)
            .to_request();
        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success());

        let error_request = test::TestRequest::post()
            .uri("/api/postMessage")
            .set_json(&json!({
                "username": "testy",
                "message": "",
            }))
            .to_request();
        let error_response = test::call_service(&app, error_request).await;

        let body = test::read_body(error_response).await;
        assert_eq!(body, "{\"error\":\"Message cannot be empty\"}");
    }
}
