use actix_web::{post, HttpRequest, HttpResponse, Responder};

pub async fn openai_advanced(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("OpenAI Advanced")
}
