use actix_web::{post, HttpResponse, Responder};

pub async fn webhook() -> impl Responder {
    HttpResponse::Ok().body("Webhook endpoint")
}
