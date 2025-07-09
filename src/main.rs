use actix_web::{web, App, HttpResponse, HttpServer, Responder};
//use serde_json::json;
use serde::{Deserialize, Serialize};

// Define your data structures
#[derive(Serialize, Deserialize, Debug)]
struct MessageJson {
    message: String
}

async fn message_123() -> impl Responder {
    let new_mes = MessageJson {message: "123".to_string()};
    HttpResponse::Ok().json(&new_mes)
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn message_hey() -> impl Responder {
    let new_mes = MessageJson {message: "Hey there!".to_string()};
    HttpResponse::Ok().json(&new_mes)
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/data")
        .service(
            web::scope("/echo")
                .route("", web::post().to(echo))
        )
        .service(
            web::scope("/123")
                .route("", web::get().to(message_123))
        )
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .configure(config)
            .route("/hey", web::get().to(message_hey))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
