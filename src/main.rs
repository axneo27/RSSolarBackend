use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

async fn message_123() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "123"
    }))
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn message_hey() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Hey there!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("asdfasdf");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/data")
                .service(
                    web::scope("/echo")
                        .route("", web::post().to(echo))
                )
                .service(
                    web::scope("/123")
                        .route("", web::get().to(message_123))
                )
            )
            .route("/hey", web::get().to(message_hey))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
