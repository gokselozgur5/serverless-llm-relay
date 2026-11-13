use serverless_llm_relay::memory::MemoryStore;

#[test]
fn test_memory_store_roundtrip() {
    let store = MemoryStore::new_in_memory().expect("db init");
    store.add_message("user_1", "user", "Hello").expect("add");
    store.add_message("user_1", "assistant", "Hi love").expect("add");

    let history = store.get_history("user_1", 10).expect("get");
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], ("user".to_string(), "Hello".to_string()));
    assert_eq!(history[1], ("assistant".to_string(), "Hi love".to_string()));
}
