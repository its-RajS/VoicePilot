mod commands;
mod models;
mod services;
mod state;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![commands::health, commands::get_config])
        .run(tauri::generate_context!())
        .expect("error while running VoicePilot");
}
