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

#[derive(Debug, Deserialize)]
struct CohereContentBlock {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CohereResponseMessage {
    content: Option<Vec<CohereContentBlock>>,
}

#[derive(Debug, Deserialize)]
struct CohereChatResponse {
    message: Option<CohereResponseMessage>,
}

#[derive(Debug, Clone)]
pub struct CohereProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl CohereProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.cohere.com/v2".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    fn build_payload(&self, req: &ChatRequest) -> CohereChatRequest {
        CohereChatRequest {
            model: req.model.clone(),
            messages: req
                .messages
                .iter()
                .map(|m| CohereMessage {
                    role: m.role.clone(),
                    content: m.content.clone(),
                })
                .collect(),
            temperature: req.temperature,
            max_tokens: req.max_tokens,
            stream: false,
        }
    }

    pub async fn complete(&self, req: &ChatRequest) -> Result<ChatResponse, CohereError> {
        let url = format!("{}/chat", self.base_url);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&self.build_payload(req))
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(CohereError::Api { status, message });
        }
        let body: CohereChatResponse = resp.json().await?;
        let text = body
            .message
            .and_then(|m| m.content)
            .and_then(|c| c.into_iter().next())
            .and_then(|b| b.text)
            .ok_or(CohereError::EmptyResponse)?;
        Ok(ChatResponse {
            content: text,
            model: req.model.clone(),
        })
    }
}
