use tauri::State;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use std::collections::HashMap;
use crate::commands::model::AppState;
use crate::models::{ApiKeyConfig, AppSettings, ModelInfo, ApiProfile};

const STORE_PATH: &str = "model-router-settings.json";

fn save_settings_to_store(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let store = app.store(STORE_PATH)
        .map_err(|e| e.to_string())?;
    store.set("settings", serde_json::to_value(settings).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

fn save_profiles_to_store(app: &AppHandle, profiles: &[ApiProfile], active_id: &str) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    store.set("profiles", serde_json::to_value(profiles).map_err(|e| e.to_string())?);
    store.set("active_profile_id", serde_json::to_value(active_id).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_from_store(app: &AppHandle) -> Result<(AppSettings, Vec<ApiProfile>, String, Vec<ModelInfo>), String> {
    let store = app.store(STORE_PATH)
        .map_err(|e| e.to_string())?;

    let settings = store.get("settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let custom_models = store.get("custom_models")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    // Load profiles (new format) or migrate from old api_keys
    let profiles: Vec<ApiProfile> = store.get("profiles")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let (profiles, active_id) = if profiles.is_empty() {
        // Try migration from old api_keys format
        let old_keys: HashMap<String, (String, Option<String>)> = store.get("api_keys")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        if old_keys.is_empty() {
            // Fresh start: create default profile
            let default = ApiProfile {
                id: "default".into(),
                name: "默认".into(),
                keys: HashMap::new(),
                is_default: true,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            (vec![default], "default".to_string())
        } else {
            // Migrate old keys into a default profile
            let default = ApiProfile {
                id: "default".into(),
                name: "默认".into(),
                keys: old_keys,
                is_default: true,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            // Persist the migration immediately
            save_profiles_to_store(app, &[default.clone()], "default")?;
            (vec![default], "default".to_string())
        }
    } else {
        let active_id = store.get("active_profile_id")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_else(|| profiles.first().map(|p| p.id.clone()).unwrap_or_default());
        (profiles, active_id)
    };

    Ok((settings, profiles, active_id, custom_models))
}

/// Store an API key in the active profile.
#[tauri::command]
pub fn set_api_key(
    provider: String,
    api_key: String,
    base_url: Option<String>,
    state: State<AppState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let mut profiles = state.profiles.lock().map_err(|e| e.to_string())?;
    let active_id = state.active_profile_id.lock().map_err(|e| e.to_string())?.clone();

    if let Some(profile) = profiles.iter_mut().find(|p| p.id == active_id) {
        profile.keys.insert(provider, (api_key, base_url));
        save_profiles_to_store(&app_handle, &profiles, &active_id)?;
    }
    Ok(true)
}

/// Get API keys from the active profile.
#[tauri::command]
pub fn get_api_keys(
    state: State<AppState>,
) -> Result<Vec<ApiKeyConfig>, String> {
    let profiles = state.profiles.lock().map_err(|e| e.to_string())?;
    let active_id = state.active_profile_id.lock().map_err(|e| e.to_string())?;

    let keys = profiles.iter()
        .find(|p| p.id == *active_id)
        .map(|p| &p.keys)
        .cloned()
        .unwrap_or_default();

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

/// Delete an API key from the active profile.
#[tauri::command]
pub fn delete_api_key(
    provider: String,
    state: State<AppState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let mut profiles = state.profiles.lock().map_err(|e| e.to_string())?;
    let active_id = state.active_profile_id.lock().map_err(|e| e.to_string())?.clone();

    if let Some(profile) = profiles.iter_mut().find(|p| p.id == active_id) {
        profile.keys.remove(&provider);
        save_profiles_to_store(&app_handle, &profiles, &active_id)?;
    }
    Ok(true)
}

/// Create a new API profile.
#[tauri::command]
pub fn create_profile(
    name: String,
    state: State<AppState>,
    app_handle: AppHandle,
) -> Result<ApiProfile, String> {
    let mut profiles = state.profiles.lock().map_err(|e| e.to_string())?;
    let active_id = state.active_profile_id.lock().map_err(|e| e.to_string())?.clone();

    let id = format!("profile_{}", chrono::Utc::now().timestamp_millis());
    let profile = ApiProfile::new(id.clone(), name);
    profiles.push(profile.clone());
    save_profiles_to_store(&app_handle, &profiles, &active_id)?;
    Ok(profile)
}

/// Delete an API profile by id.
#[tauri::command]
pub fn delete_profile(
    id: String,
    state: State<AppState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let mut profiles = state.profiles.lock().map_err(|e| e.to_string())?;
    let mut active_id = state.active_profile_id.lock().map_err(|e| e.to_string())?;

    if profiles.len() <= 1 {
        return Err("至少需要保留一个配置模板".to_string());
    }

    profiles.retain(|p| p.id != id);

    // If the deleted profile was active, switch to the first remaining
    if *active_id == id {
        *active_id = profiles.first().map(|p| p.id.clone()).unwrap_or_default();
    }

    save_profiles_to_store(&app_handle, &profiles, &active_id)?;
    Ok(true)
}

/// Switch to a different profile.
#[tauri::command]
pub fn switch_profile(
    id: String,
    state: State<AppState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let profiles = state.profiles.lock().map_err(|e| e.to_string())?;
    let mut active_id = state.active_profile_id.lock().map_err(|e| e.to_string())?;

    if !profiles.iter().any(|p| p.id == id) {
        return Err(format!("配置模板 {} 不存在", id));
    }

    *active_id = id.clone();
    save_profiles_to_store(&app_handle, &profiles, &id)?;
    Ok(true)
}

/// List all API profiles.
#[tauri::command]
pub fn list_profiles(
    state: State<AppState>,
) -> Result<Vec<ApiProfile>, String> {
    let profiles = state.profiles.lock().map_err(|e| e.to_string())?;
    Ok(profiles.clone())
}

/// Get the active profile id.
#[tauri::command]
pub fn get_active_profile_id(
    state: State<AppState>,
) -> Result<String, String> {
    let id = state.active_profile_id.lock().map_err(|e| e.to_string())?;
    Ok(id.clone())
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
    app_handle: AppHandle,
) -> Result<bool, String> {
    let mut settings = state.settings.lock()
        .map_err(|e| e.to_string())?;
    *settings = new_settings;
    save_settings_to_store(&app_handle, &settings)?;
    Ok(true)
}
