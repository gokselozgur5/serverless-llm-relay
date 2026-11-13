use thiserror::Error;

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
