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
