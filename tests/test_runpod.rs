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


