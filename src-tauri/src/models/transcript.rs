use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptEvent {
    pub text: String,
    pub is_final: bool,
    pub confidence: Option<f32>,
}
