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
    None
}


