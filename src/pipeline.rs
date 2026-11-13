use std::sync::Mutex;

use thiserror::Error;

use crate::memory::MemoryStore;
pub use crate::provider::LLMProvider;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PipelineError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("memory error: {0}")]
    Memory(String),
    #[error("provider error: {0}")]
    Provider(String),
}

pub trait ConversationMemory {
    fn load_history(&self, user_id: &str, limit: usize) -> Result<Vec<(String, String)>, String>;
    fn save_message(&self, user_id: &str, role: &str, content: &str) -> Result<(), String>;
}

impl ConversationMemory for MemoryStore {
    fn load_history(&self, user_id: &str, limit: usize) -> Result<Vec<(String, String)>, String> {
        self.get_history(user_id, limit).map_err(|e| e.to_string())
    }

    fn save_message(&self, user_id: &str, role: &str, content: &str) -> Result<(), String> {
        self.add_message(user_id, role, content).map_err(|e| e.to_string())
    }
}
