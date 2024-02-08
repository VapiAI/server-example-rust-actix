use actix_web::{post, HttpRequest, HttpResponse, Responder};

pub async fn openai_sse(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("OpenAI SSE")
}
