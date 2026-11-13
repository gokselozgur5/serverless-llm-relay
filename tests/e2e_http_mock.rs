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

async fn spawn_mock_upstream(should_fail: bool) -> String {
    let app = Router::new()
        .route("/v1/chat", post(mock_chat_handler))
        .with_state(should_fail);
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");
    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("serve");
    });
    format!("http://{addr}/v1/chat")
}

fn build_http_chat_request(store: &MemoryStore, user_id: &str, text: &str) -> ChatRequest {
    let mut messages = vec![ChatMessage {
        role: "system".to_string(),
        content: format_prompt(ALARA_DEFAULT_PERSONA, user_id),
    }];
    for (role, content) in store.get_history(user_id, 10).expect("history") {
        messages.push(ChatMessage { role, content });
    }
    messages.push(ChatMessage { role: "user".to_string(), content: text.to_string() });
    ChatRequest { messages, model: "mock-vllm".to_string(), temperature: 0.5, max_tokens: 128 }
}
