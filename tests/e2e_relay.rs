use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use serverless_llm_relay::memory::MemoryStore;
use serverless_llm_relay::persona::{format_prompt, ALARA_DEFAULT_PERSONA};
use serverless_llm_relay::provider::{ChatMessage, ChatRequest, ChatResponse};
use serverless_llm_relay::Config;

struct MockProvider {
    model: String,
    replies: Mutex<VecDeque<String>>,
    requests: Mutex<Vec<ChatRequest>>,
    fail_next: AtomicBool,
}

impl MockProvider {
    fn new(model: &str, replies: &[&str]) -> Self {
        let queue = replies.iter().map(|s| (*s).to_string()).collect();
        Self {
            model: model.to_string(),
            replies: Mutex::new(queue),
            requests: Mutex::new(Vec::new()),
            fail_next: AtomicBool::new(false),
        }
    }

    fn set_fail_next(&self, should_fail: bool) {
        self.fail_next.store(should_fail, Ordering::SeqCst);
    }

    fn recorded_requests(&self) -> Vec<ChatRequest> {
        self.requests.lock().expect("lock requests").clone()
    }

    fn complete(&self, req: ChatRequest) -> Result<ChatResponse, String> {
        if self.fail_next.swap(false, Ordering::SeqCst) {
            return Err("upstream provider unavailable".to_string());
        }
        self.requests.lock().expect("lock requests").push(req);
        let content = self
            .replies
            .lock()
            .expect("lock replies")
            .pop_front()
            .unwrap_or_else(|| "default mock reply".to_string());
        Ok(ChatResponse { content, model: self.model.clone() })
    }
}

fn build_relay_request(
    store: &MemoryStore,
    model: &str,
    user_id: &str,
    user_name: &str,
    incoming_text: &str,
) -> Result<ChatRequest, String> {
    let mut messages = vec![ChatMessage {
        role: "system".to_string(),
        content: format_prompt(ALARA_DEFAULT_PERSONA, user_name),
    }];
    for (role, content) in store.get_history(user_id, 20).map_err(|e| e.to_string())? {
        messages.push(ChatMessage { role, content });
    }
    messages.push(ChatMessage { role: "user".to_string(), content: incoming_text.to_string() });
    Ok(ChatRequest { messages, model: model.to_string(), temperature: 0.7, max_tokens: 256 })
}

fn execute_relay_turn(
    store: &MemoryStore,
    provider: &MockProvider,
    cfg: &Config,
    user_id: &str,
    user_name: &str,
    incoming_text: &str,
) -> Result<ChatResponse, String> {
    let req = build_relay_request(store, &cfg.default_model, user_id, user_name, incoming_text)?;
    let resp = provider.complete(req)?;
    store.add_message(user_id, "user", incoming_text).map_err(|e| e.to_string())?;
    store.add_message(user_id, "assistant", &resp.content).map_err(|e| e.to_string())?;
    Ok(resp)
}

fn sample_config() -> Config {
    Config {
        cohere_api_key: Some("test-cohere-key".to_string()),
        runpod_api_key: None,
        runpod_endpoint_id: None,
        default_model: "command-r-plus".to_string(),
        port: 8080,
    }
}

#[test]
fn test_single_turn_relay_persists_dialogue() {
    let store = MemoryStore::new_in_memory().expect("init memory");
    let provider = MockProvider::new("command-r-plus", &["I missed your voice."]);
    let cfg = sample_config();
    let resp = execute_relay_turn(&store, &provider, &cfg, "u1", "Alex", "Good evening")
        .expect("relay turn");
    assert_eq!(resp.content, "I missed your voice.");
    assert_eq!(resp.model, "command-r-plus");
    let recorded = provider.recorded_requests();
    assert_eq!(recorded.len(), 1);
    assert!(recorded[0].messages[0].content.contains("Alex"));
    assert_eq!(store.get_history("u1", 10).expect("history").len(), 2);
}

#[test]
fn test_multi_turn_relay_preserves_context() {
    let store = MemoryStore::new_in_memory().expect("init memory");
    let provider = MockProvider::new("command-r-plus", &["First reply", "Second reply"]);
    let cfg = sample_config();
    execute_relay_turn(&store, &provider, &cfg, "u1", "Alex", "Turn one").expect("turn 1");
    let second = execute_relay_turn(&store, &provider, &cfg, "u1", "Alex", "Turn two")
        .expect("turn 2");
    assert_eq!(second.content, "Second reply");
    let recorded = provider.recorded_requests();
    assert_eq!(recorded.len(), 2);
    assert_eq!(recorded[1].messages.len(), 4);
    assert_eq!(recorded[1].messages[1].content, "Turn one");
    assert_eq!(recorded[1].messages[2].content, "First reply");
    assert_eq!(recorded[1].messages[3].content, "Turn two");
}
