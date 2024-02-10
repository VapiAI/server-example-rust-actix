use actix_web::{post, HttpResponse, Responder};

pub async fn inbound() -> impl Responder {
    println!("Hello, world!");
    HttpResponse::Ok().body("Inbound endpoint")
}
