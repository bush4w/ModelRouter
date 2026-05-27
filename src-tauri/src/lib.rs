mod commands;
mod models;
mod services;

use tauri::Manager;
use commands::{claude_md, config, model};
use commands::model::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let state = app.state::<AppState>();
            match config::load_from_store(app.handle()) {
                Ok((settings, profiles, active_id, custom_models)) => {
                    *state.settings.lock().unwrap() = settings;
                    *state.profiles.lock().unwrap() = profiles;
                    *state.active_profile_id.lock().unwrap() = active_id;
                    *state.custom_models.lock().unwrap() = custom_models;
                }
                Err(e) => {
                    eprintln!("[ModelRouter] Failed to load persisted state: {}", e);
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            claude_md::parse_claude_md,
            claude_md::write_model_config,
            claude_md::write_role,
            claude_md::read_file_content,
            claude_md::get_default_claude_md_path,
            claude_md::write_claude_code_env,
            claude_md::get_claude_settings_json,
            model::get_recommendation,
            model::get_model_list,
            model::fetch_provider_models,
            model::refresh_all_fetched_models,
            model::record_choice,
            model::add_custom_model,
            model::remove_custom_model,
            model::list_custom_models,
            config::set_api_key,
            config::get_api_keys,
            config::delete_api_key,
            config::create_profile,
            config::delete_profile,
            config::switch_profile,
            config::list_profiles,
            config::get_active_profile_id,
            config::get_settings,
            config::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
