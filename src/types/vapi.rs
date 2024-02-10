use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub model: String,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
    pub functions: Option<Vec<Function>>,
    pub provider: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub is_async: Option<bool>,
    pub description: Option<String>,
    pub parameters: Option<HashMap<String, String>>,
}

pub type PlayHTEmotion = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {
    pub provider: String,
    pub voice_id: String,
    pub speed: Option<f32>,
    pub stability: Option<f32>,
    pub similarity_boost: Option<f32>,
    pub style: Option<i32>,
    pub use_speaker_boost: Option<bool>,
    pub temperature: Option<f32>,
    pub emotion: Option<PlayHTEmotion>,
    pub voice_guidance: Option<i32>,
    pub style_guidance: Option<i32>,
    pub text_guidance: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assistant {
    pub name: Option<String>,
    pub transcriber: Option<Transcriber>,
    pub model: Option<Model>,
    pub voice: Option<Voice>,
    pub language: Option<String>,
    pub forwarding_phone_number: Option<String>,
    pub first_message: Option<String>,
    pub voicemail_message: Option<String>,
    pub end_call_message: Option<String>,
    pub end_call_phrases: Option<Vec<String>>,
    pub interruptions_enabled: Option<bool>,
    pub recording_enabled: Option<bool>,
    pub end_call_function_enabled: Option<bool>,
    pub dial_keypad_function_enabled: Option<bool>,
    pub fillers_enabled: Option<bool>,
    pub client_messages: Option<Vec<HashMap<String, String>>>,
    pub server_messages: Option<Vec<HashMap<String, String>>>,
    pub silence_timeout_seconds: Option<i32>,
    pub response_delay_seconds: Option<i32>,
    pub live_transcripts_enabled: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub parent_id: Option<String>,
    pub server_url: Option<String>,
    pub server_url_secret: Option<String>,
    pub id: Option<String>,
    pub org_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcriber {
    pub provider: String,
    pub model: Option<String>,
    pub keywords: Option<Vec<String>>,
}

pub type VapiCallStatus = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String,
    pub message: Option<String>,
    pub name: Option<String>,
    pub args: Option<String>,
    pub result: Option<String>,
    pub time: i64,
    pub end_time: Option<i64>,
    pub seconds_from_start: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseVapiPayload {
    pub call: VapiCall,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantRequestPayload {
    pub call: VapiCall,
    #[serde(rename = "type")]
    pub payload_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusUpdatePayload {
    pub call: VapiCall,
    #[serde(rename = "type")]
    pub payload_type: String,
    pub status: VapiCallStatus,
    pub messages: Option<Vec<async_openai::types::ChatCompletionRequestMessage>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCallPayload {
    pub call: VapiCall,
    #[serde(rename = "type")]
    pub payload_type: String,
    pub functionCall: OpenAIFunctionCall,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIFunctionCall {
    pub name: String,
    pub parameters: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfCallReportPayload {
    pub call: VapiCall,
    #[serde(rename = "type")]
    pub payload_type: String,
    pub ended_reason: String,
    pub transcript: String,
    pub messages: Vec<ConversationMessage>,
    pub summary: String,
    pub recording_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HangPayload {
    pub call: VapiCall,
    #[serde(rename = "type")]
    pub payload_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeechUpdatePayload {
    pub call: VapiCall,
    #[serde(rename = "type")]
    pub payload_type: String,
    pub status: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptPayload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub role: String,
    pub transcript_type: String,
    pub transcript: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VapiCall {
    // Define the fields for VapiCall here
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum VapiPayload {
    #[serde(rename = "assistant-request")]
    AssistantRequestPayload(AssistantRequestPayload),
    #[serde(rename = "status-update")]
    StatusUpdatePayload(StatusUpdatePayload),
    #[serde(rename = "function-call")]
    FunctionCallPayload(FunctionCallPayload),
    #[serde(rename = "end-of-call-report")]
    EndOfCallReportPayload(EndOfCallReportPayload),
    #[serde(rename = "hang")]
    HangPayload(HangPayload),
    #[serde(rename = "speech-update")]
    SpeechUpdatePayload(SpeechUpdatePayload),
    #[serde(rename = "transcript")]
    TranscriptPayload(TranscriptPayload),
}

impl<'de> Deserialize<'de> for VapiPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let message_type = value
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| serde::de::Error::custom("type field is missing or not a string"))?;

        match message_type {
            "assistant-request" => Ok(VapiPayload::AssistantRequestPayload(
                serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            )),
            "status-update" => Ok(VapiPayload::StatusUpdatePayload(
                serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            )),
            "end-of-call-report" => Ok(VapiPayload::EndOfCallReportPayload(
                serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            )),
            "function-call" => Ok(VapiPayload::FunctionCallPayload(
                serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            )),
            "hang" => Ok(VapiPayload::HangPayload(
                serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            )),
            "speech-update" => Ok(VapiPayload::SpeechUpdatePayload(
                serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            )),
            "transcript" => Ok(VapiPayload::TranscriptPayload(
                serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            )),
            // Add cases for other types...
            _ => Err(serde::de::Error::custom(format!(
                "unknown message type: {}",
                message_type
            ))),
        }
        // struct VapiPayloadVisitor;

        // impl<'de> Visitor<'de> for VapiPayloadVisitor {
        //     type Value = VapiPayload;

        //     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        //         formatter.write_str("struct VapiPayload")
        //     }

        //     fn visit_map<V>(self, mut map: V) -> Result<VapiPayload, V::Error>
        //     where
        //         V: MapAccess<'de>,
        //     {
        //         let mut payload_type: Option<String> = None;
        //         let mut call: Option<VapiCall> = None;
        //         while let Some(key) = map.next_key()? {
        //             println!("Key: {}", key);
        //             match key {
        //                 "type" => {
        //                     if payload_type.is_some() {
        //                         return Err(de::Error::duplicate_field("type"));
        //                     }
        //                     payload_type = Some(map.next_value()?);
        //                     println!("Payload type in while: {:?}", payload_type);
        //                     break;
        //                 }
        //                 "call" => {
        //                     if call.is_some() {
        //                         return Err(de::Error::duplicate_field("call"));
        //                     }
        //                     call = Some(map.next_value()?);
        //                 }
        //                 _ => {
        //                     // Consume the value associated with the unexpected key
        //                     let _: serde_json::Value = map.next_value()?;
        //                     println!("Unexpected key while deserializing VapiPayload: {}", key);
        //                 }
        //             }
        //         }

        //         println!("Before payload_type");
        //         let payload_type = payload_type.ok_or_else(|| de::Error::missing_field("type"))?;
        //         let call = call.ok_or_else(|| de::Error::missing_field("call"))?;

        //         println!("Payload type: {}", payload_type);
        //         println!("Call: {:?}", call);

        //         match payload_type.as_str() {
        //             "assistant-request" => {
        //                 let payload: AssistantRequestPayload = map.next_value()?;

        //                 println!("Assistant request payload: {:?}", payload);
        //                 Ok(VapiPayload::AssistantRequestPayload(payload))
        //             }
        //             "status-update" => {
        //                 let payload: StatusUpdatePayload = map.next_value()?;
        //                 Ok(VapiPayload::StatusUpdatePayload(payload))
        //             }
        //             "function-call" => {
        //                 println!("Function call payload before payload?");
        //                 let payload: FunctionCallPayload = map.next_value()?;
        //                 println!("Function call payload: {:?}", payload);
        //                 Ok(VapiPayload::FunctionCallPayload(payload))
        //             }
        //             "speech-update" => {
        //                 let payload: SpeechUpdatePayload = map.next_value()?;
        //                 Ok(VapiPayload::SpeechUpdatePayload(payload))
        //             }
        //             "transcript" => {
        //                 let payload: TranscriptPayload = map.next_value()?;
        //                 Ok(VapiPayload::TranscriptPayload(payload))
        //             }
        //             "hang" => {
        //                 let payload: HangPayload = map.next_value()?;
        //                 Ok(VapiPayload::HangPayload(payload))
        //             }
        //             "end-of-call-report" => {
        //                 let payload: EndOfCallReportPayload = map.next_value()?;
        //                 Ok(VapiPayload::EndOfCallReportPayload(payload))
        //             }
        //             // Add similar match arms for the other variants of VapiPayload
        //             _ => {
        //                 println!("Unknown variant for VapiPayload: {}", payload_type);
        //                 Err(de::Error::unknown_variant(
        //                     &payload_type,
        //                     &[
        //                         "assistant-request",
        //                         "status-update",
        //                         "function-call",
        //                         "speech-update",
        //                         "transcript",
        //                         "hang",
        //                         "end-of-call-report",
        //                     ],
        //                 ))
        //             }
        //         }
        //     }
        // }

        // const FIELDS: &'static [&'static str] = &["type", "call"];
        // deserializer.deserialize_struct("VapiPayload", FIELDS, VapiPayloadVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCallMessageResponse {
    pub result: Option<String>,
    // pub error: Option<String>,
    pub forwardToClientEnabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantRequestMessageResponse {
    pub assistant: Option<Assistant>,
    pub error: Option<String>,
}

pub type StatusUpdateMessageResponse = HashMap<String, String>;
pub type SpeechUpdateMessageResponse = HashMap<String, String>;
pub type TranscriptMessageResponse = HashMap<String, String>;
pub type HangMessageResponse = HashMap<String, String>;
pub type EndOfCallReportMessageResponse = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub enum VapiResponse {
    FunctionCallMessageResponse(FunctionCallMessageResponse),
    AssistantRequestMessageResponse(AssistantRequestMessageResponse),
    StatusUpdateMessageResponse(StatusUpdateMessageResponse),
    SpeechUpdateMessageResponse(SpeechUpdateMessageResponse),
    TranscriptMessageResponse(TranscriptMessageResponse),
    HangMessageResponse(HangMessageResponse),
    EndOfCallReportMessageResponse(EndOfCallReportMessageResponse),
}
