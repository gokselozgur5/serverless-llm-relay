use crate::provider::{ChatRequest, ChatResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum CohereError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },
    #[error("Empty response")]
    EmptyResponse,
}

#[derive(Debug, Serialize)]
struct CohereMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct CohereChatRequest {
    model: String,
    messages: Vec<CohereMessage>,
    temperature: f32,
    max_tokens: u32,
    stream: bool,
}
