use actix_web::{web::Json, HttpRequest, HttpResponse};
use async_openai::{
    types::ChatCompletionRequestMessage, types::ChatCompletionRequestUserMessageContent,
    types::CreateChatCompletionRequest, Client,
};
use futures::stream::StreamExt;
use serde_json;
use std::error::Error;

pub async fn openai_advanced(
    req: HttpRequest,
    body: Json<CreateChatCompletionRequest>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let client = Client::new();

    let mut request = body.into_inner();

    // Modify the last message in the request if it is a user message
    if let Some(ChatCompletionRequestMessage::User(user_message)) = request.messages.last_mut() {
        match &mut user_message.content {
            ChatCompletionRequestUserMessageContent::Text(text) => {
                let new_prompt = format!("Modified PROMPT: {}", text);
                *text = new_prompt;
            }
            _ => println!("User message content is not text"),
        }
    }

    let mut response_body = String::new();

    if request.stream.unwrap_or(true) {
        let mut stream = client.chat().create_stream(request).await?;

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
            .append_header(("Content-Type", "text/event-stream"))
            .body(response_body))
    } else {
        let response = match client.chat().create(request).await {
            Ok(res) => res,
            Err(e) => return Err(Box::new(e)),
        };
        Ok(HttpResponse::Ok().json(response))
    }
}
