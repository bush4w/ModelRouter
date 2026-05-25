use tauri::State;
use std::sync::Mutex;
use crate::models::{RoleInfo, ModelConfig, AppSettings};
use crate::services::parser;

/// Parse Claude.md file at the given path and extract role definitions.
#[tauri::command]
pub fn parse_claude_md(file_path: String) -> Result<Vec<RoleInfo>, String> {
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;
    let mut roles = parser::parse_claude_md(&content);
    for role in &mut roles {
        role.file_path = Some(file_path.clone());
    }
    Ok(roles)
}

/// Write model configuration block into Claude.md.
#[tauri::command]
pub fn write_model_config(
    file_path: String,
    config: ModelConfig,
) -> Result<bool, String> {
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    let config_block = format!(
        "---\nname: model-config\ndescription: ModelRoute Auto Config\n---\n\n## 模型配置\n\n| 任务类型 | 推荐模型 | 提供商 | 切换时机 | 更新时间 |\n|---------|---------|--------|---------|----------|\n| {} | {} | {} | {} | {} |\n",
        config.task_type,
        config.recommended_model,
        config.provider,
        config.trigger_condition,
        config.updated_at,
    );

    // Replace existing config block or append
    let start_marker = "name: model-config";
    let end_marker = "## 模型配置";

    let new_content = if content.contains(start_marker) {
        // Update existing config
        let re = regex::Regex::new(
            r"(?s)---\nname: model-config\n.*?(\n## |\Z)"
        ).map_err(|e| e.to_string())?;
        re.replace(&content, |_caps: &regex::Captures| {
            format!("{}\n", config_block.trim())
        }).to_string()
    } else {
        // Append at end
        format!("{}\n\n{}", content, config_block)
    };

    // Backup before writing
    let backup_path = format!("{}.bak.before_modelroute", file_path);
    std::fs::write(&backup_path, &content)
        .map_err(|e| format!("Failed to create backup: {}", e))?;

    std::fs::write(&file_path, &new_content)
        .map_err(|e| format!("Failed to write file {}: {}", file_path, e))?;

    Ok(true)
}

/// Get the default Claude.md path for the current user.
#[tauri::command]
pub fn get_default_claude_md_path() -> Result<String, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "Cannot find home directory".to_string())?;

    let candidates = vec![
        home.join(".claude").join("CLAUDE.md"),
        home.join(".claude").join("projects"),
    ];

    for candidate in &candidates {
        if candidate.exists() && candidate.is_file() {
            return Ok(candidate.to_string_lossy().to_string());
        }
        if candidate.is_dir() {
            // Check for any CLAUDE.md in project subdirs
            if let Ok(entries) = std::fs::read_dir(candidate) {
                for entry in entries.flatten() {
                    let path = entry.path().join("CLAUDE.md");
                    if path.exists() {
                        return Ok(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    // Fallback: return expected path even if it doesn't exist yet
    Ok(home.join(".claude").join("CLAUDE.md")
        .to_string_lossy()
        .to_string())
}
