use crate::models::config::AppConfig;

#[tauri::command]
pub async fn health() -> Result<&'static str, String> {
    Ok("ok")
}

#[tauri::command]
pub async fn get_config() -> Result<AppConfig, String> {
    Ok(AppConfig::default())
}
