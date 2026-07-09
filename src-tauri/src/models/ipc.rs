use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IpcRequest {
    HealthCheck {
        request_id: u64,
    },
    StartRecording {
        request_id: u64,
        sample_rate_hz: u32,
    },
    AudioChunk {
        request_id: u64,
        pcm_s16le: Vec<u8>,
    },
    CleanupRequest {
        request_id: u64,
        transcript: String,
        mode: String,
        model: String,
    },
}

impl IpcRequest {
    pub fn request_id(&self) -> u64 {
        match self {
            Self::HealthCheck { request_id }
            | Self::StartRecording { request_id, .. }
            | Self::AudioChunk { request_id, .. }
            | Self::CleanupRequest { request_id, .. } => *request_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IpcResponse {
    HealthStatus {
        request_id: u64,
        status: String,
        #[serde(default)]
        protocol_version: u32,
    },
    RecordingStarted {
        request_id: u64,
    },
    PartialTranscript {
        request_id: u64,
        text: String,
        confidence: Option<f32>,
    },
    FinalTranscript {
        request_id: u64,
        text: String,
        confidence: Option<f32>,
    },
    CleanupResponse {
        request_id: u64,
        raw_transcript: String,
        cleaned_prompt: String,
    },
    Error {
        request_id: u64,
        message: String,
    },
}

impl IpcResponse {
    pub fn request_id(&self) -> u64 {
        match self {
            Self::HealthStatus { request_id, .. }
            | Self::RecordingStarted { request_id }
            | Self::PartialTranscript { request_id, .. }
            | Self::FinalTranscript { request_id, .. }
            | Self::CleanupResponse { request_id, .. }
            | Self::Error { request_id, .. } => *request_id,
        }
    }
}
