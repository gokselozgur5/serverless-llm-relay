use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunpodError {
    #[error("missing RUNPOD_API_KEY")]
    MissingApiKey,
    #[error("missing RUNPOD_ENDPOINT_ID")]
    MissingEndpointId,
    #[error("request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("failed to parse response: {0}")]
    Json(#[from] serde_json::Error),
    #[error("runpod error: {0}")]
    Api(String),
}

#[derive(Debug, serde::Deserialize)]
struct RunpodSyncResponse {
    #[allow(dead_code)]
    pub id: Option<String>,
    pub status: String,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
}

pub fn extract_content(output: &serde_json::Value) -> Option<String> {
    if let Some(text) = output.as_str() {
        return Some(text.to_string());
    }
    if let Some(arr) = output.as_array() {
        for item in arr {
            if let Some(res) = extract_content(item) {
                return Some(res);
            }
        }
    }
    if let Some(choices) = output.get("choices").and_then(|c| c.as_array()) {
        for choice in choices {
            if let Some(c) = choice.get("message").and_then(|m| m.get("content")).and_then(|s| s.as_str()) {
                return Some(c.to_string());
            }
            if let Some(t) = choice.get("text").and_then(|s| s.as_str()) {
                return Some(t.to_string());
            }
        }
    }
    output.get("text").and_then(|t| t.as_str()).map(|s| s.to_string())
}

#[derive(Clone, Debug)]
pub struct RunpodProvider {
    pub api_key: String,
    pub endpoint_id: String,
    pub base_url: String,
    pub client: reqwest::Client,
}

impl RunpodProvider {
    pub fn new(api_key: impl Into<String>, endpoint_id: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            endpoint_id: endpoint_id.into(),
            base_url: "https://api.runpod.ai/v2".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn from_config(config: &crate::Config) -> Result<Self, RunpodError> {
        let api_key = config.runpod_api_key.as_deref().ok_or(RunpodError::MissingApiKey)?;
        let endpoint_id = config.runpod_endpoint_id.as_deref().ok_or(RunpodError::MissingEndpointId)?;
        Ok(Self::new(api_key, endpoint_id))
    }

    pub async fn complete(
        &self,
        req: &crate::provider::ChatRequest,
    ) -> Result<crate::provider::ChatResponse, RunpodError> {
        let url = format!("{}/{}/runsync", self.base_url, self.endpoint_id);
        let prompt = req.messages.last().map(|m| m.content.as_str()).unwrap_or("");
        let payload = serde_json::json!({
            "input": { "prompt": prompt, "messages": &req.messages, "temperature": req.temperature, "max_tokens": req.max_tokens }
        });
        let resp = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload).send().await?;
        if !resp.status().is_success() {
            return Err(RunpodError::Api(format!("HTTP {}", resp.status())));
        }
        let parsed: RunpodSyncResponse = resp.json().await?;
        if parsed.status != "COMPLETED" {
            return Err(RunpodError::Api(parsed.error.unwrap_or(parsed.status)));
        }
        let out = parsed.output.ok_or_else(|| RunpodError::Api("missing output".to_string()))?;
        let content = extract_content(&out).ok_or_else(|| RunpodError::Api("unknown output".to_string()))?;
        Ok(crate::provider::ChatResponse { content, model: req.model.clone() })
    }
}






