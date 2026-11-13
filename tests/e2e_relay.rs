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
