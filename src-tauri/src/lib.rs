mod commands;
mod models;
mod services;
mod state;
mod utils;

use services::ipc_client::IpcClient;
use services::ollama_service::OllamaService;
use state::app_state::VoicePilotState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let ipc_client = IpcClient::spawn().map_err(|error| error.to_string())?;
            let ollama_status = OllamaService::default().discover();
            app.manage(VoicePilotState {
                ipc_client,
                ollama_status: std::sync::Mutex::new(ollama_status),
            });
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::health,
            commands::get_config,
            commands::cleanup_transcript,
            commands::get_ollama_status,
            commands::refresh_ollama_status,
            commands::set_llm_model,
            commands::pull_ollama_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running VoicePilot");
}
