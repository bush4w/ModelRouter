mod commands;
mod models;
mod services;

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
        .invoke_handler(tauri::generate_handler![
            claude_md::parse_claude_md,
            claude_md::write_model_config,
            claude_md::get_default_claude_md_path,
            model::get_recommendation,
            model::get_model_list,
            model::record_choice,
            config::set_api_key,
            config::get_api_keys,
            config::delete_api_key,
            config::get_settings,
            config::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
