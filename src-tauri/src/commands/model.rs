use tauri::State;
use std::sync::Mutex;
use crate::models::{ModelRecommendation, ModelInfo, UserChoice};
use crate::services::router;

pub struct AppState {
    pub learning_choices: Mutex<Vec<UserChoice>>,
    pub api_keys: Mutex<std::collections::HashMap<String, (String, Option<String>)>>,
    pub settings: Mutex<crate::models::AppSettings>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            learning_choices: Mutex::new(Vec::new()),
            api_keys: Mutex::new(std::collections::HashMap::new()),
            settings: Mutex::new(crate::models::AppSettings::default()),
        }
    }
}

/// Get model recommendation for a role and task type.
#[tauri::command]
pub fn get_recommendation(
    role: String,
    task_type: String,
    state: State<AppState>,
) -> Result<ModelRecommendation, String> {
    let choices = state.learning_choices.lock()
        .map_err(|e| e.to_string())?;
    Ok(router::recommend(&role, &task_type, &choices))
}

/// Get the full list of known models.
#[tauri::command]
pub fn get_model_list() -> Result<Vec<ModelInfo>, String> {
    Ok(router::get_model_list())
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
