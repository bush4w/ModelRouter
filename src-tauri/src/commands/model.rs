use tauri::State;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::models::{ModelRecommendation, ModelInfo, UserChoice, ApiProfile};
use crate::services::router;

const STORE_PATH: &str = "model-router-settings.json";

pub struct AppState {
    pub learning_choices: Mutex<Vec<UserChoice>>,
    pub profiles: Mutex<Vec<ApiProfile>>,
    pub active_profile_id: Mutex<String>,
    pub settings: Mutex<crate::models::AppSettings>,
    pub custom_models: Mutex<Vec<ModelInfo>>,
    pub fetched_models: Mutex<HashMap<String, Vec<ModelInfo>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            learning_choices: Mutex::new(Vec::new()),
            profiles: Mutex::new(Vec::new()),
            active_profile_id: Mutex::new(String::new()),
            settings: Mutex::new(crate::models::AppSettings::default()),
            custom_models: Mutex::new(Vec::new()),
            fetched_models: Mutex::new(HashMap::new()),
        }
    }
}

fn save_custom_models_to_store(app: &AppHandle, models: &[ModelInfo]) -> Result<(), String> {
    let store = app.store(STORE_PATH)
        .map_err(|e| e.to_string())?;
    store.set("custom_models", serde_json::to_value(models).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

/// Get model recommendation for a role and task type.
#[tauri::command]
pub fn get_recommendation(
    role: String,
    task_type: String,
    available_providers: Vec<String>,
    state: State<AppState>,
) -> Result<ModelRecommendation, String> {
    let choices = state.learning_choices.lock()
        .map_err(|e| e.to_string())?;
    Ok(router::recommend(&role, &task_type, &choices, &available_providers))
}

/// Get the full list of known models (built-in + custom + dynamically fetched).
#[tauri::command]
pub fn get_model_list(
    state: State<AppState>,
) -> Result<Vec<ModelInfo>, String> {
    let custom = state.custom_models.lock()
        .map_err(|e| e.to_string())?;
    let fetched = state.fetched_models.lock()
        .map_err(|e| e.to_string())?;
    let mut all = router::get_builtin_models();

    // Merge fetched models: replace built-in with same id, add new ones
    for (_, models) in fetched.iter() {
        for fm in models {
            if let Some(existing) = all.iter_mut().find(|m| m.id == fm.id) {
                *existing = fm.clone();
            } else {
                all.push(fm.clone());
            }
        }
    }

    // Merge custom models
    for cm in custom.iter() {
        if !all.iter().any(|m| m.id == cm.id) {
            all.push(cm.clone());
        }
    }

    Ok(all)
}

/// Fetch models from a provider's API and cache them in state.
#[tauri::command]
pub async fn fetch_provider_models(
    provider: String,
    api_key: String,
    base_url: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ModelInfo>, String> {
    let models = crate::services::fetcher::fetch_provider_models(
        &provider,
        &api_key,
        base_url.as_deref(),
    )
    .await?;

    let mut fetched = state.fetched_models.lock()
        .map_err(|e| e.to_string())?;
    fetched.insert(provider, models.clone());

    Ok(models)
}

/// Refresh fetched models for all configured providers in the active profile.
#[tauri::command]
pub async fn refresh_all_fetched_models(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    // Extract keys in a block scope — MutexGuard must NOT cross await
    let keys: Vec<(String, String, Option<String>)> = {
        let profiles = state.profiles.lock().map_err(|e| e.to_string())?;
        let active_id = state.active_profile_id.lock().map_err(|e| e.to_string())?;
        profiles.iter()
            .find(|p| p.id == *active_id)
            .map(|p| p.keys.iter().map(|(provider, (key, base_url))| {
                (provider.clone(), key.clone(), base_url.clone())
            }).collect())
            .unwrap_or_default()
    };

    let mut results = vec![];
    for (provider, key, base_url) in keys {
        match crate::services::fetcher::fetch_provider_models(
            &provider, &key, base_url.as_deref()
        ).await {
            Ok(models) => {
                let count = models.len();
                let mut fetched = state.fetched_models.lock().map_err(|e| e.to_string())?;
                fetched.insert(provider.clone(), models);
                drop(fetched);
                results.push(format!("{}: {} 个模型", provider, count));
            }
            Err(e) => {
                results.push(format!("{}: 获取失败 — {}", provider, e));
            }
        }
    }

    Ok(results)
}

/// Record a user choice to the learning database.
#[tauri::command]
pub fn record_choice(
    choice: UserChoice,
    state: State<AppState>,
) -> Result<bool, String> {
    let mut choices = state.learning_choices.lock()
        .map_err(|e| e.to_string())?;
    let mut storage = router::InMemoryStorage::new();
    router::record_choice(&choice, &mut storage)
        .map_err(|e| e.to_string())?;
    choices.push(choice);
    Ok(true)
}

/// Add a custom model.
#[tauri::command]
pub fn add_custom_model(
    model: ModelInfo,
    state: State<AppState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let mut models = state.custom_models.lock()
        .map_err(|e| e.to_string())?;
    models.push(model);
    save_custom_models_to_store(&app_handle, &models)?;
    Ok(true)
}

/// Remove a custom model by id.
#[tauri::command]
pub fn remove_custom_model(
    model_id: String,
    state: State<AppState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let mut models = state.custom_models.lock()
        .map_err(|e| e.to_string())?;
    models.retain(|m| m.id != model_id);
    save_custom_models_to_store(&app_handle, &models)?;
    Ok(true)
}

/// List custom models only.
#[tauri::command]
pub fn list_custom_models(
    state: State<AppState>,
) -> Result<Vec<ModelInfo>, String> {
    let models = state.custom_models.lock()
        .map_err(|e| e.to_string())?;
    Ok(models.clone())
}
