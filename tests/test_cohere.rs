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

#[tokio::test]
async fn test_cohere_complete_success() {
    let base_url = spawn_mock_server().await;
    let provider = CohereProvider::new("test-key").with_base_url(base_url);
    let req = ChatRequest {
        messages: vec![ChatMessage {
            role: "user".into(),
            content: "Hi".into(),
        }],
        model: "command-r".into(),
        temperature: 0.3,
        max_tokens: 50,
    };
    let res = provider.complete(&req).await.unwrap();
    assert_eq!(res.content, "Hello");
    assert_eq!(res.model, "command-r");
}
