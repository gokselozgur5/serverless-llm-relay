use serverless_llm_relay::telegram::TelegramAdapter;

#[test]
fn test_parse_update_valid_message() {
    let json = r#"{"update_id":1001,"message":{"message_id":42,"from":{"id":999,"is_bot":false,"first_name":"Alice"},"chat":{"id":12345,"type":"private"},"text":"Hello relay"}}"#;
    let update = TelegramAdapter::parse_update(json).unwrap();
    assert_eq!(update.update_id, 1001);
    let msg = update.message.unwrap();
    assert_eq!(msg.message_id, 42);
    assert_eq!(msg.chat.id, 12345);
    assert_eq!(msg.chat.chat_type, "private");
    assert_eq!(msg.text.as_deref(), Some("Hello relay"));
    let from = msg.from.unwrap();
    assert_eq!(from.id, 999);
    assert_eq!(from.first_name, "Alice");
}

#[test]
fn test_format_send_message_payload() {
    let payload = TelegramAdapter::format_send_message(
        12345,
        "Hello back from relay",
        Some("MarkdownV2".to_string()),
    );
    assert_eq!(payload.chat_id, 12345);
    assert_eq!(payload.text, "Hello back from relay");
    assert_eq!(payload.parse_mode.as_deref(), Some("MarkdownV2"));
    let json = serde_json::to_string(&payload).unwrap();
    assert!(json.contains("\"chat_id\":12345"));
    assert!(json.contains("\"parse_mode\":\"MarkdownV2\""));
}

#[test]
fn test_parse_update_without_optional_fields() {
    let json = r#"{"update_id":1002,"message":{"message_id":43,"chat":{"id":-100,"type":"group"}}}"#;
    let update = TelegramAdapter::parse_update(json).unwrap();
    assert_eq!(update.update_id, 1002);
    let msg = update.message.unwrap();
    assert_eq!(msg.chat.id, -100);
    assert_eq!(msg.text, None);
    assert_eq!(msg.from, None);
    let payload = TelegramAdapter::format_send_message(-100, "Group msg", None);
    let json_out = serde_json::to_string(&payload).unwrap();
    assert!(!json_out.contains("parse_mode"));
}
