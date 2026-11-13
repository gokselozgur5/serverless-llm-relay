use serde_json::json;
use serverless_llm_relay::runpod::extract_content;

#[test]
fn test_extract_content_from_plain_string() {
    let val = json!("response text directly");
    assert_eq!(extract_content(&val), Some("response text directly".to_string()));
}

#[test]
fn test_extract_content_from_nested_array() {
    let val = json!([["nested string content"]]);
    assert_eq!(extract_content(&val), Some("nested string content".to_string()));
}

#[test]
fn test_extract_content_openai_choices() {
    let val = json!({
        "choices": [{"message": {"role": "assistant", "content": "hello from vllm"}}]
    });
    assert_eq!(extract_content(&val), Some("hello from vllm".to_string()));
}

#[test]
fn test_extract_content_text_field() {
    let val = json!({"text": "plain object text"});
    assert_eq!(extract_content(&val), Some("plain object text".to_string()));
}

#[test]
fn test_runpod_from_config_missing_keys() {
    let cfg = serverless_llm_relay::Config {
        cohere_api_key: None,
        runpod_api_key: None,
        runpod_endpoint_id: None,
        default_model: "vllm-model".to_string(),
        port: 8080,
    };
    assert!(serverless_llm_relay::RunpodProvider::from_config(&cfg).is_err());
}

#[test]
fn test_runpod_from_config_success() {
    let cfg = serverless_llm_relay::Config {
        cohere_api_key: None,
        runpod_api_key: Some("key123".to_string()),
        runpod_endpoint_id: Some("ep456".to_string()),
        default_model: "vllm-model".to_string(),
        port: 8080,
    };
    let provider = serverless_llm_relay::RunpodProvider::from_config(&cfg).unwrap();
    assert_eq!(provider.api_key, "key123");
    assert_eq!(provider.endpoint_id, "ep456");
}

async fn start_mock_server(body: serde_json::Value) -> (String, tokio::task::JoinHandle<()>) {
    let app = axum::Router::new().route(
        "/v2/test-ep/runsync",
        axum::routing::post(move || {
            let b = body.clone();
            async move { axum::Json(b) }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });
    (format!("http://{addr}/v2"), handle)
}

#[tokio::test]
async fn test_runpod_complete_success() {
    let mock = json!({
        "status": "COMPLETED",
        "output": {"choices": [{"message": {"role": "assistant", "content": "vllm answer"}}]}
    });
    let (base_url, handle) = start_mock_server(mock).await;
    let provider = serverless_llm_relay::RunpodProvider::new("test_key", "test-ep")
        .with_base_url(base_url);
    let req = serverless_llm_relay::provider::ChatRequest {
        messages: vec![serverless_llm_relay::provider::ChatMessage {
            role: "user".to_string(),
            content: "hello".to_string(),
        }],
        model: "vllm-mistral".to_string(),
        temperature: 0.7,
        max_tokens: 128,
    };
    let res = provider.complete(&req).await.unwrap();
    assert_eq!(res.content, "vllm answer");
    assert_eq!(res.model, "vllm-mistral");
    handle.abort();
}





