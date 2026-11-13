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
