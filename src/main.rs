extern crate actix_web;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::io;

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json("Hello World!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
        .bind("127.0.0.1:8888")
        .unwrap()
        .run()
        .await
}
