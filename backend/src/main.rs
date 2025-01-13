use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

mod core;
mod services;

use core::message_manager::Message;
use core::message_manager::Messages;

use services::database::DbWrapper;

use core::message_manager::get_messages;
use core::message_manager::post_message;

use services::database::convert_sqlx_error;
use services::database::init_db;

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
        Err(e) => Messages {
            messages: vec![Message {
                username: "server".to_string(),
                message: format!("could not retrive messages: {}", e),
            }],
        },
    };

    return HttpResponse::Ok()
        .content_type("application/json")
        .json(messages);
}

#[post("/api/postMessage")]
async fn api_post_message(data: Json<Message>, app_state: Data<AppState>) -> impl Responder {
    if data.message == *"" {
        return HttpResponse::Ok().body("error: could not post message");
    }

    match post_message(data, &app_state.db).await {
        Ok(_) => {}
        Err(e) => return HttpResponse::Ok().body(format!("error: could not post message, {}", e)),
    };

    return HttpResponse::Ok().into();
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
