use actix_web::{post, HttpResponse, Responder};

pub async fn outbound() -> impl Responder {
    HttpResponse::Ok().body("Outbound endpoint")
}
