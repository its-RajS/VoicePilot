use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkey: HotkeyConfig,
    pub audio: AudioConfig,
    pub models: ModelConfig,
    pub typing: TypingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig { pub modifiers: Vec<String>, pub key: String, pub mode: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig { pub device_id: Option<String>, pub vad_sensitivity: f32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig { pub stt_model: String, pub llm_model: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingConfig { pub mode: String, pub speed_cps: u32 }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkey: HotkeyConfig { modifiers: vec!["ctrl".into()], key: "Space".into(), mode: "push_to_talk".into() },
            audio: AudioConfig { device_id: None, vad_sensitivity: 0.5 },
            models: ModelConfig { stt_model: "nvidia/parakeet-tdt-0.6b".into(), llm_model: "qwen3:8b".into() },
            typing: TypingConfig { mode: "auto_type".into(), speed_cps: 50 },
        }
    }
}
