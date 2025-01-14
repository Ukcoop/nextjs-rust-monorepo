use actix_web::get;
use actix_web::{HttpResponse, Responder};

use crate::core::message_manager::Message;

#[get("/api/")]
pub async fn hello() -> impl Responder {
    let default_message = Message {
        username: "Alexander".to_string(),
        message: "This is the backend if my app, you can get messages from /api/getMessages"
            .to_string(),
    };

    return HttpResponse::Ok()
        .content_type("application/json")
        .json(default_message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;

        let request = test::TestRequest::get().uri("/api/").to_request();
        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success());
        let body: Message = test::read_body_json(response).await;
        assert_eq!(body.username, "Alexander");
        assert!(body.message.contains("This is the backend"));
    }
}
