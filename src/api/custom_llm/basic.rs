use actix_web::{web::Json, HttpRequest, HttpResponse, Responder};
use async_openai::types::CreateChatCompletionRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Option<Vec<Message>>,
    max_tokens: Option<usize>,
    temperature: Option<f32>,
    stream: Option<bool>,
    call: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    system_fingerprint: Option<String>,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    index: usize,
    delta: Delta,
    logprobs: Option<()>,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Delta {
    content: String,
}

pub async fn basic(body: Json<CreateChatCompletionRequest>) -> impl Responder {
    let response = ChatCompletionResponse {
        id: "chatcmpl-8mcLf78g0quztp4BMtwd3hEj58Uof".to_string(),
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp(),
        model: "gpt-3.5-turbo-0613".to_string(),
        system_fingerprint: None,
        choices: vec![Choice {
            index: 0,
            delta: Delta {
                content: "I am a highly intelligent question-answering AI. I can help you with any question you have.".to_string(),
            },
            logprobs: None,
            finish_reason: "stop".to_string(),
        }],
    };

    HttpResponse::Created().json(response)
}
