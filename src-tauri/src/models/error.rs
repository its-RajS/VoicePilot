use thiserror::Error;

#[derive(Debug, Error)]
pub enum VoicePilotError {
    #[error("audio device error: {0}")]
    AudioDevice(String),
    #[error("inference service unavailable: {0}")]
    InferenceUnavailable(String),
    #[error("typing failed: {0}")]
    Typing(String),
    #[error("configuration error: {0}")]
    Config(String),
}
