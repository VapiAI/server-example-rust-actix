use actix_web::{post, HttpResponse, Responder};

pub async fn rag() -> impl Responder {
    HttpResponse::Ok().body("RAG function")
}
