use crate::models::config::AppConfig;
use crate::models::ollama::OllamaStatus;
use crate::services::config_service::ConfigService;
use crate::services::ollama_service::OllamaService;
use crate::state::app_state::VoicePilotState;
use tauri::State;

#[tauri::command]
pub async fn health(state: State<'_, VoicePilotState>) -> Result<String, String> {
    state
        .ipc_client
        .health_check()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_config() -> Result<AppConfig, String> {
    ConfigService::for_app()
        .and_then(|service| service.load())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn cleanup_transcript(
    transcript: String,
    state: State<'_, VoicePilotState>,
) -> Result<String, String> {
    state
        .ipc_client
        .cleanup_transcript(&transcript, "engineering")
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_ollama_status(state: State<'_, VoicePilotState>) -> Result<OllamaStatus, String> {
    state
        .ollama_status
        .lock()
        .map(|status| status.clone())
        .map_err(|_| "ollama status lock poisoned".to_string())
}

#[tauri::command]
pub async fn refresh_ollama_status(state: State<'_, VoicePilotState>) -> Result<OllamaStatus, String> {
    let latest = OllamaService::default().discover();
    let mut status = state
        .ollama_status
        .lock()
        .map_err(|_| "ollama status lock poisoned".to_string())?;
    *status = latest.clone();
    Ok(latest)
}
