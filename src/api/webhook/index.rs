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

pub async fn webhook(payload: web::Json<Payload>) -> impl Responder {
    let response: VapiResponse = match &payload.message {
        VapiPayload::AssistantRequestPayload(_) => handle_assistant_request(&payload.message),
        VapiPayload::StatusUpdatePayload(_) => handle_status_update(&payload.message),
        VapiPayload::FunctionCallPayload(_) => handle_function_call(&payload.message).await,
        VapiPayload::EndOfCallReportPayload(_) => handle_end_of_call_report(&payload.message),
        VapiPayload::SpeechUpdatePayload(_) => handle_speech_update(&payload.message),
        VapiPayload::TranscriptPayload(_) => handle_transcript(&payload.message),
        VapiPayload::HangPayload(_) => handle_hang(&payload.message),
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
            // ... (rest of the match cases)
            "getCharacterInspiration" => {
                let params: GetCharacterInspirationParams =
                    serde_json::from_value(data.functionCall.parameters.clone())
                        .unwrap_or_else(|_| GetCharacterInspirationParams::default());
                let inspiration_response = get_character_inspiration(params).await;
                VapiResponse::FunctionCallMessageResponse(FunctionCallMessageResponse {
                    result: Some(inspiration_response.result),
                    forwardToClientEnabled: Some(inspiration_response.forward_to_client_enabled),
                })
            }
            _ => VapiResponse::FunctionCallMessageResponse(FunctionCallMessageResponse {
                result: Some("Invalid function name".to_string()),
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

fn handle_status_update(message: &VapiPayload) -> VapiResponse {
    // Handle status update event
    VapiResponse::StatusUpdateMessageResponse({
        let mut map = StatusUpdateMessageResponse::new();
        map.insert("message".to_string(), "Status update handled".to_string());
        map
    })
}

fn handle_assistant_request(message: &VapiPayload) -> VapiResponse {
    // Handle assistant request event
    VapiResponse::AssistantRequestMessageResponse(AssistantRequestMessageResponse {
        assistant: None,
        error: Some("Assistant request handled".to_string()),
    })
}

fn handle_end_of_call_report(message: &VapiPayload) -> VapiResponse {
    // Handle end of call report event
    VapiResponse::EndOfCallReportMessageResponse({
        let mut map = EndOfCallReportMessageResponse::new();
        map.insert(
            "message".to_string(),
            "End of call report handled".to_string(),
        );
        map
    })
}

fn handle_speech_update(message: &VapiPayload) -> VapiResponse {
    // Handle speech update event
    VapiResponse::SpeechUpdateMessageResponse({
        let mut map = SpeechUpdateMessageResponse::new();
        map.insert("message".to_string(), "Speech update handled".to_string());
        map
    })
}

fn handle_transcript(message: &VapiPayload) -> VapiResponse {
    // Handle transcript event
    VapiResponse::TranscriptMessageResponse({
        let mut map = TranscriptMessageResponse::new();
        map.insert("message".to_string(), "Transcript handled".to_string());
        map
    })
}

fn handle_hang(message: &VapiPayload) -> VapiResponse {
    // Handle hang event
    VapiResponse::HangMessageResponse({
        let mut map = HangMessageResponse::new();
        map.insert("message".to_string(), "Hang handled".to_string());
        map
    })
}
