use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serverless_llm_relay::memory::MemoryStore;
use serverless_llm_relay::persona::{format_prompt, ALARA_DEFAULT_PERSONA};
use serverless_llm_relay::provider::{ChatMessage, ChatRequest, ChatResponse};
use tokio::net::TcpListener;

async fn mock_chat_handler(
    State(should_fail): State<bool>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, StatusCode> {
    if should_fail {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    let last_user = req.messages.last().map(|m| m.content.as_str()).unwrap_or("");
    Ok(Json(ChatResponse {
        content: format!("echo:{last_user}"),
        model: req.model,
    }))
}
