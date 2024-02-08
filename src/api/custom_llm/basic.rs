use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn basic(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Custom LLM Basic")
}
