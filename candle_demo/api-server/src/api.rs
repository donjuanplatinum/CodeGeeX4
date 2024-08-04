use actix_web::{
    get, post,
    web::{self, Data},
    HttpRequest, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default)]
    pub messages: Vec<ChatMessage>,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_top_p")]
    pub top_p: f64,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default)]
    pub stop: Vec<String>,
    #[serde(default = "default_stream")]
    pub stream: bool,
    #[serde(default)]
    pub presence_penalty: Option<f32>,
}

fn default_model() -> String {
    "THUDM/codegeex4-all-9b".to_string()
}

fn default_temperature() -> f64 {
    0.2
}

fn default_top_p() -> f64 {
    0.2
}

fn default_max_tokens() -> usize {
    1024
}

fn default_stream() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeltaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponseStreamChoice {
    pub index: i32,
    pub delta: DeltaMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionStreamResponse {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<ChatCompletionResponseStreamChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponseChoice {
    pub index: i32,
    pub message: ChatMessage,
    pub finish_reason: Option<FinishResaon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletionResponseChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FinishResaon {
    STOP,
    LENGTH,
}
use std::time::{SystemTime, UNIX_EPOCH};
impl ChatCompletionResponse {
    pub fn empty() -> Self {
        let current_time = SystemTime::now();
        Self {
            id: format!("chatcmpl-{}", short_uuid::ShortUuid::generate()),
            object: "chat.completion".to_string(),
            created: current_time
                .duration_since(UNIX_EPOCH)
                .expect("failed to get time")
                .as_secs()
                .into(),
            model: "codegeex4".to_string(),
            choices: vec![ChatCompletionResponseChoice::empty()],
        }
    }
}

impl ChatCompletionResponseChoice {
    pub fn empty() -> Self {
        Self {
            index: 0,
            message: ChatMessage {
                role: "assistant".to_string(),
                content: "".to_string(),
            },
            finish_reason: None,
        }
    }
}

impl ChatCompletionRequest {
    pub fn empty() -> Self {
        Self {
            model: "codegeex4".to_string(),
            messages: vec![ChatMessage {
                role: "assistant".to_string(),
                content: "".to_string(),
            }],
            temperature: 0.2_f64,
            top_p: 0.2_f64,
            max_tokens: 1024_usize,
            stop: vec![
                "<|user|>".to_string(),
                "<|assistant|>".to_string(),
                "<|observation|>".to_string(),
                "<|endoftext|>".to_string(),
            ],
            stream: true,
            presence_penalty: None,
        }
    }
}

// impl DeltaMessage {
//     pub fn new() -> Self {
// 	role:
//     }
// }
