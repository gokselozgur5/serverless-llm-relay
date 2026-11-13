use axum::{routing::post, Json, Router};
use serde_json::json;
use serverless_llm_relay::CohereProvider;
use serverless_llm_relay::provider::{ChatMessage, ChatRequest};

async fn spawn_mock_server() -> String {
    let app = Router::new().route("/chat", post(|| async {
        Json(json!({"message": {"content": [{"text": "Hello"}]}}))
    }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });
    format!("http://{}", addr)
}
