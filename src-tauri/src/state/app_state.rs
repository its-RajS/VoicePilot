use crate::models::ollama::OllamaStatus;
use crate::services::ipc_client::IpcClient;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Idle,
    Listening,
    Processing,
    Typing,
    Error,
}

pub struct VoicePilotState {
    pub ipc_client: Arc<IpcClient>,
    pub ollama_status: Mutex<OllamaStatus>,
}
