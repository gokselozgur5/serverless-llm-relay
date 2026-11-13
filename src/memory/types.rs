use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmotionalArc {
    pub start_tone: String,
    pub current_tone: String,
    pub intimacy_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyMoment {
    pub timestamp_epoch_s: u64,
    pub summary: String,
    pub emotional_significance: f32,
}
