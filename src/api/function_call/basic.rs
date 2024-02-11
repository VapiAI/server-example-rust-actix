use crate::functions::{
    get_character_inspiration::{self, GetCharacterInspirationParams},
    get_random_name::{self, NameParams},
};
use actix_web::{post, web, HttpResponse, Responder};
use get_character_inspiration::get_character_inspiration;
use get_random_name::get_random_name;
use serde::{Deserialize, Serialize};

use crate::types::vapi::{
    AssistantRequestMessageResponse, EndOfCallReportMessageResponse, FunctionCallMessageResponse,
    HangMessageResponse, SpeechUpdateMessageResponse, StatusUpdateMessageResponse,
    TranscriptMessageResponse, VapiPayload, VapiResponse,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub message: VapiPayload,
}

pub async fn basic(payload: web::Json<Payload>) -> impl Responder {
    let response: VapiResponse = match &payload.message {
        VapiPayload::FunctionCallPayload(_) => handle_function_call(&payload.message).await,
        _ => return HttpResponse::BadRequest().finish(),
    };
    match serde_json::to_string(&response) {
        Ok(body) => HttpResponse::Ok()
            .content_type("application/json")
            .body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn handle_function_call(message: &VapiPayload) -> VapiResponse {
    if let VapiPayload::FunctionCallPayload(data) = message {
        match data.functionCall.name.as_str() {
            "getRandomName" => {
                let params: NameParams = match serde_json::from_value(
                    data.functionCall.parameters.clone(),
                ) {
                    Ok(params) => params,
                    Err(_) => return VapiResponse::FunctionCallMessageResponse(
                        FunctionCallMessageResponse {
                            result: Some(
                                "Not enough information provided to generate name. Can u tell me "
                                    .to_string(),
                            ),
                            forwardToClientEnabled: Some(false),
                        },
                    ),
                };
                VapiResponse::FunctionCallMessageResponse(FunctionCallMessageResponse {
                    result: match get_random_name(params).await {
                        Ok(name) => Some(name),
                        Err(_) => Some("Failed to get random name".to_string()),
                    },
                    forwardToClientEnabled: Some(false),
                })
            }
            _ => VapiResponse::FunctionCallMessageResponse(FunctionCallMessageResponse {
                result: Some("".to_string()),
                forwardToClientEnabled: Some(false),
            }),
        }
    } else {
        println!("Invalid message type for function call");
        VapiResponse::FunctionCallMessageResponse(FunctionCallMessageResponse {
            result: Some("Invalid message type".to_string()),
            forwardToClientEnabled: Some(false),
        })
    }
}
