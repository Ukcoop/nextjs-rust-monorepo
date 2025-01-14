use actix_web::get;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};

use crate::core::message_manager::get_messages;
use crate::{AppState, ErrorResponse};

#[get("/api/getMessages")]
async fn api_get_messages(app_state: Data<AppState>) -> impl Responder {
    let messages = match get_messages(&app_state.db).await {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Error fetching messages: {}", e),
            });
        }
    };

    return HttpResponse::Ok()
        .content_type("application/json")
        .json(messages);
}

#[cfg(test)]
mod tests {
    use crate::services::database::init_db;

    use super::*;
    use actix_web::{test, App};

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

        let request = test::TestRequest::get()
            .uri("/api/getMessages")
            .to_request();
        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success());
    }
}
