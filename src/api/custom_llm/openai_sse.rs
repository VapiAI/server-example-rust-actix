use actix_web::{
    post,
    web::{Bytes, Json},
    HttpRequest, HttpResponse, Responder,
};
use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures::stream::StreamExt;
use serde_json;
use std::collections::HashMap;
use std::error::Error;

pub async fn openai_sse(
    req: HttpRequest,
    body: Json<CreateChatCompletionRequest>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let client = Client::new();

    let request = body.into_inner();
    // print request in console so that we can see what the request looks like
    println!("{:?}", request);

    let mut stream = client.chat().create_stream(request).await?;

    let mut response_body = String::new();

    while let Some(response) = stream.next().await {
        match response {
            Ok(ccr) => {
                let ccr_string = serde_json::to_string(&ccr).unwrap();
                response_body.push_str(&format!("data: {}\n\n", ccr_string));
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/event-stream")
        .body(response_body))
}
