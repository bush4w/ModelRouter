use tauri::State;
use crate::commands::model::AppState;
use crate::models::{ApiKeyConfig, AppSettings};

/// Store an API key for a provider.
#[tauri::command]
pub fn set_api_key(
    provider: String,
    api_key: String,
    base_url: Option<String>,
    state: State<AppState>,
) -> Result<bool, String> {
    let mut keys = state.api_keys.lock()
        .map_err(|e| e.to_string())?;
    keys.insert(provider, (api_key, base_url));
    Ok(true)
}

/// Get all stored API key configurations (keys masked).
#[tauri::command]
pub fn get_api_keys(
    state: State<AppState>,
) -> Result<Vec<ApiKeyConfig>, String> {
    let keys = state.api_keys.lock()
        .map_err(|e| e.to_string())?;

    let configs: Vec<ApiKeyConfig> = keys.iter().map(|(provider, (key, base_url))| {
        ApiKeyConfig {
            provider: provider.clone(),
            api_key: key.clone(),
            base_url: base_url.clone(),
            enabled: true,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }).collect();

    Ok(configs)
}

/// Delete an API key for a provider.
#[tauri::command]
pub fn delete_api_key(
    provider: String,
    state: State<AppState>,
) -> Result<bool, String> {
    let mut keys = state.api_keys.lock()
        .map_err(|e| e.to_string())?;
    keys.remove(&provider);
    Ok(true)
}

/// Get current application settings.
#[tauri::command]
pub fn get_settings(
    state: State<AppState>,
) -> Result<AppSettings, String> {
    let settings = state.settings.lock()
        .map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

/// Update application settings.
#[tauri::command]
pub fn update_settings(
    new_settings: AppSettings,
    state: State<AppState>,
) -> Result<bool, String> {
    let mut settings = state.settings.lock()
        .map_err(|e| e.to_string())?;
    *settings = new_settings;
    Ok(true)
}
