use std::fmt::Display;
use std::future::Future;

use serde::{Deserialize, Serialize};

pub use crate::runpod::RunpodProvider;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,
}

pub trait LLMProvider: Send + Sync {
    type Error: Display + Send + Sync + 'static;

    fn complete(
        &self,
        request: &ChatRequest,
    ) -> impl Future<Output = Result<ChatResponse, Self::Error>> + Send;
}
