use std::collections::VecDeque;
use std::future::{ready, Ready};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use serverless_llm_relay::memory::MemoryStore;
use serverless_llm_relay::pipeline::{ConversationMemory, LLMProvider, PipelineError, RelayPipeline};
use serverless_llm_relay::provider::{ChatRequest, ChatResponse};

struct MockProvider {
    replies: Mutex<VecDeque<String>>,
    requests: Mutex<Vec<ChatRequest>>,
    fail_next: AtomicBool,
}

impl MockProvider {
    fn new(replies: &[&str]) -> Self {
        Self {
            replies: Mutex::new(replies.iter().map(|s| (*s).to_string()).collect()),
            requests: Mutex::new(Vec::new()),
            fail_next: AtomicBool::new(false),
        }
    }
}

impl LLMProvider for MockProvider {
    type Error = String;

    fn complete(&self, request: &ChatRequest) -> Ready<Result<ChatResponse, Self::Error>> {
        if self.fail_next.swap(false, Ordering::SeqCst) {
            return ready(Err("upstream failure".to_string()));
        }
        self.requests.lock().expect("lock").push(request.clone());
        let text = self
            .replies
            .lock()
            .expect("lock")
            .pop_front()
            .unwrap_or_else(|| "fallback".to_string());
        ready(Ok(ChatResponse { content: text, model: request.model.clone() }))
    }
}

#[derive(Default)]
struct MockMemory {
    entries: Mutex<Vec<(String, String, String)>>,
}

impl ConversationMemory for MockMemory {
    fn load_history(&self, user_id: &str, limit: usize) -> Result<Vec<(String, String)>, String> {
        let all: Vec<_> = self
            .entries
            .lock()
            .expect("lock")
            .iter()
            .filter(|(u, _, _)| u == user_id)
            .map(|(_, r, c)| (r.clone(), c.clone()))
            .collect();
        let start = all.len().saturating_sub(limit);
        Ok(all[start..].to_vec())
    }

    fn save_message(&self, user_id: &str, role: &str, content: &str) -> Result<(), String> {
        self.entries.lock().expect("lock").push((user_id.into(), role.into(), content.into()));
        Ok(())
    }
}
