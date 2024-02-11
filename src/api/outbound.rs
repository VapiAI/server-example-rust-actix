use crate::config::env;
use actix_web::{post, web, HttpResponse, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestBody {
    phoneNumberId: String,
    assistantId: String,
    customerNumber: String,
}

pub async fn outbound(request_body: web::Json<RequestBody>) -> impl Responder {
    let env_config = env::load_env_config();

    // let request_body_bytes = serde_json::to_vec(&request_body.into_inner()).unwrap();

    let request_body_bytes = serde_json::to_vec(&request_body).unwrap();

    let client = Client::new();
    let mut req = client
        .post(format!("{}/call/phone", env_config.vapi.base_url))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", env_config.vapi.api_key),
        )
        .body(request_body_bytes)
        .build()
        .unwrap();

    let resp = client.execute(req).await.unwrap();
    let status_code = resp.status();

    if !status_code.is_success() {
        return HttpResponse::InternalServerError()
            .json(format!("HTTP error! status: {}", status_code));
    }

    let data: serde_json::Value = resp.json().await.unwrap();
    HttpResponse::Ok().json(data)
}
