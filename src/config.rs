use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub cohere_api_key: Option<String>,
    pub runpod_api_key: Option<String>,
    pub runpod_endpoint_id: Option<String>,
    pub default_model: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            cohere_api_key: env::var("COHERE_API_KEY").ok(),
            runpod_api_key: env::var("RUNPOD_API_KEY").ok(),
            runpod_endpoint_id: env::var("RUNPOD_ENDPOINT_ID").ok(),
            default_model: env::var("DEFAULT_MODEL")
                .unwrap_or_else(|_| "command-r".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
        }
    }
}
