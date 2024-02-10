use actix_web::{post, HttpResponse, Responder};

pub async fn basic() -> impl Responder {
    HttpResponse::Ok().body("Basic function")
}
