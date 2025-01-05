use actix_cors::Cors;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/api/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(serde_json::json!({"message": "Hello world!"}))
}

// this is here becaue i am going to use post requests in the future
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(hello)
            .service(echo)
    })
    .bind(("0.0.0.0", 3080))?
    .run()
    .await
}
