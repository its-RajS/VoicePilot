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
    let config = ConfigService::for_app()
        .and_then(|service| service.load())
        .map_err(|error| error.to_string())?;

    state
        .ipc_client
        .cleanup_transcript(&transcript, "engineering", &config.models.llm_model)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn set_llm_model(model: String) -> Result<(), String> {
    ConfigService::for_app()
        .and_then(|service| service.set_llm_model(&model))
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn pull_ollama_model(model: String) -> Result<(), String> {
    // ponytail: reqwest::blocking::Client spins up its own tokio runtime; dropping it
    // inside an async worker panics ("Cannot drop a runtime in a context where blocking
    // is not allowed"). Run the whole construct+use+drop on a blocking thread.
    tauri::async_runtime::spawn_blocking(move || OllamaService::default().pull_model(&model))
        .await
        .map_err(|join_err| join_err.to_string())?
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
    // ponytail: see pull_ollama_model — keep the blocking reqwest client off the async runtime.
    let latest = tauri::async_runtime::spawn_blocking(|| OllamaService::default().discover())
        .await
        .map_err(|join_err| join_err.to_string())?;
    let mut status = state
        .ollama_status
        .lock()
        .map_err(|_| "ollama status lock poisoned".to_string())?;
    *status = latest.clone();
    Ok(latest)
}
