use actix_web::{post, HttpResponse, Responder};

pub async fn inbound() -> impl Responder {
    HttpResponse::Ok().body("Inbound endpoint")
}
