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
