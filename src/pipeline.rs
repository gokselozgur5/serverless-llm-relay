use std::sync::Mutex;

use thiserror::Error;

use crate::memory::MemoryStore;
use crate::persona::{format_prompt, ALARA_DEFAULT_PERSONA};
use crate::provider::{ChatMessage, ChatRequest, ChatResponse};
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

impl ConversationMemory for Mutex<MemoryStore> {
    fn load_history(&self, user_id: &str, limit: usize) -> Result<Vec<(String, String)>, String> {
        let guard = self.lock().map_err(|e| e.to_string())?;
        guard.get_history(user_id, limit).map_err(|e| e.to_string())
    }

    fn save_message(&self, user_id: &str, role: &str, content: &str) -> Result<(), String> {
        let guard = self.lock().map_err(|e| e.to_string())?;
        guard.add_message(user_id, role, content).map_err(|e| e.to_string())
    }
}

pub struct RelayPipeline<P, M = MemoryStore> {
    memory: M,
    provider: P,
    persona: String,
    model: String,
    history_limit: usize,
    temperature: f32,
    max_tokens: u32,
}

impl<P, M> RelayPipeline<P, M> {
    pub fn new(memory: M, provider: P) -> Self {
        Self {
            memory,
            provider,
            persona: ALARA_DEFAULT_PERSONA.to_string(),
            model: "command-r".to_string(),
            history_limit: 20,
            temperature: 0.7,
            max_tokens: 512,
        }
    }
}
