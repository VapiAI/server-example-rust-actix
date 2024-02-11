use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::types::{self, vapi::VapiPayload};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub message: VapiPayload,
}

pub async fn inbound(c: web::Json<Payload>) -> impl Responder {
    let name = "Paula".to_string();
    let model_name = "gpt-3.5-turbo".to_string();
    let temp = 0.7;
    let system_prompt = "You're Paula, an AI assistant who can help user draft beautiful emails to their clients based on the user requirements. Then Call sendEmail function to actually send the email.".to_string();
    let function_description =
        "Send email to the given email address and with the given content.".to_string();
    let first_message = "Hi, I'm Paula, your personal email assistant.".to_string();

    let assistant = types::vapi::Assistant {
        name: Some(name),
        model: Some(types::vapi::Model {
            provider: "openai".to_string(),
            model: model_name,
            temperature: Some(temp),
            system_prompt: Some(system_prompt),
            url: None,
            functions: Some(vec![types::vapi::Function {
                name: "sendEmail".to_string(),
                description: Some(function_description),
                is_async: Some(false),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "email": {
                            "type": "string",
                            "description": "Email to which we want to send the content."
                        },
                        "content": {
                            "type": "string",
                            "description": "Actual Content of the email to be sent."
                        }
                    },
                    "required": ["email"]
                })),
            }]),
        }),
        voice: Some(types::vapi::Voice {
            provider: "11labs".to_string(),
            voice_id: "paula".to_string(),
            speed: None,
            stability: None,
            similarity_boost: None,
            style: None,
            emotion: None,
            use_speaker_boost: None,
            temperature: None,
            voice_guidance: None,
            style_guidance: None,
            text_guidance: None,
        }),
        first_message: Some(first_message),
        ..Default::default()
    };

    HttpResponse::Ok().json(json!({"assistant": assistant}))
}
