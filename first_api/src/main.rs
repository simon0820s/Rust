use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Number {
    number: u64
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("My first API")
}

#[post("/echo")]
async fn echo(req_body: web::Json<Number>) -> impl Responder {
    if req_body.number % 2 == 0 {
        HttpResponse::Ok().body("Your number is even")
    } else {
        HttpResponse::Ok().body("Your number is not even")
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
