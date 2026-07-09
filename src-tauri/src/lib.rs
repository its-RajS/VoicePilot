mod commands;
mod models;
mod services;
mod state;
mod utils;

use services::config_service::ConfigService;
use services::hotkey_service::HotkeyService;
use services::ipc_client::IpcClient;
use services::ollama_service::OllamaService;
use state::app_state::VoicePilotState;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let ipc_client = Arc::new(IpcClient::spawn().map_err(|error| error.to_string())?);
            let ollama_status = OllamaService::default().discover();
            app.manage(VoicePilotState {
                ipc_client: ipc_client.clone(),
                ollama_status: std::sync::Mutex::new(ollama_status),
            });

            // Bind the configured push-to-talk hotkey. If config can't load we
            // log and skip it — the rest of the app still works.
            match ConfigService::for_app().and_then(|service| service.load()) {
                Ok(config) => HotkeyService::start(app.handle().clone(), ipc_client.clone(), config),
                Err(error) => eprintln!(
                    "config load failed, hotkey service disabled: {error}"
                ),
            }
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                // ponytail: default is Trace, which floods the console with reqwest
                // noise and hides real signal. Info keeps app logs, drops reqwest DEBUG/TRACE.
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
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
