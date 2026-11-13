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
}
