use actix_web::{web::Json, HttpRequest, HttpResponse};
use async_openai::{types::CreateChatCompletionRequest, Client};
use futures::stream::StreamExt;
use serde_json;
use std::error::Error;

pub async fn openai_sse(
    req: HttpRequest,
    body: Json<CreateChatCompletionRequest>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let client = Client::new();

    let request = body.into_inner();
    // print request in console so that we can see what the request looks like
    println!("{:?}", request);

    let mut response_body = String::new();

    // Check if the stream is false in the request
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
        // If stream is false, call the normal chat create
        let response = match client.chat().create(request).await {
            Ok(res) => res,
            Err(e) => return Err(Box::new(e)),
        };
        Ok(HttpResponse::Ok().json(response))
    }
}
