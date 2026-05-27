use crate::models::{RoleInfo, ModelConfig};
use crate::services::parser;
use serde_json::Value;

/// Parse Claude.md file at the given path and extract role definitions.
#[tauri::command]
pub fn parse_claude_md(file_path: String) -> Result<Vec<RoleInfo>, String> {
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("无法读取文件 {}：{}，请检查文件路径是否正确以及文件是否存在", file_path, e))?;
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
        .map_err(|e| format!("无法读取文件 {}：{}，请检查文件路径是否正确以及文件是否存在", file_path, e))?;

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

    let new_content = if content.contains(start_marker) {
        // Update existing config — find the block by markers, replace with string ops
        let block_start = content.rfind("---\nname: model-config")
            .or_else(|| content.rfind("---\r\nname: model-config"))
            .unwrap_or(0);

        // Find end of the block: next "---" or "## " after block_start
        let after_start = &content[block_start + 4..]; // skip the leading "---"
        let block_end = after_start
            .find("\n---")
            .map(|p| block_start + 4 + p + 1) // right before \n---
            .or_else(|| {
                after_start
                    .find("\n## ")
                    .map(|p| block_start + 4 + p)
            })
            .unwrap_or(content.len());

        // Replace old block with new one
        format!(
            "{}{}{}",
            &content[..block_start],
            config_block.trim(),
            &content[block_end..]
        )
    } else {
        // Append at end
        format!("{}\n\n{}", content, config_block)
    };

    // Backup before writing
    let backup_path = format!("{}.bak.before_modelroute", file_path);
    std::fs::write(&backup_path, &content)
        .map_err(|e| format!("无法创建备份文件 {}：{}，请检查磁盘空间和目录权限", backup_path, e))?;

    std::fs::write(&file_path, &new_content)
        .map_err(|e| format!("无法写入文件 {}：{}，请检查文件权限，确保文件未被其他程序占用", file_path, e))?;

    Ok(true)
}

/// Read file content as plain text for preview (bypasses fs plugin scope).
#[tauri::command]
pub fn read_file_content(file_path: String) -> Result<String, String> {
    std::fs::read_to_string(&file_path)
        .map_err(|e| format!("无法读取文件 {}：{}，请检查文件路径是否正确以及文件是否存在", file_path, e))
}

/// Write a new role definition to the Claude.md file.
#[tauri::command]
pub fn write_role(
    file_path: String,
    name: String,
    alias: String,
    description: String,
    skills: Vec<String>,
) -> Result<bool, String> {
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("无法读取文件 {}：{}，请检查文件路径是否正确以及文件是否存在", file_path, e))?;

    let skills_note = format!("<!-- modelrouter-skills: {} -->", skills.join(", "));
    let new_row = format!(
        "| {} | {} | {} | {}",
        name, alias, description, skills_note
    );

    // Find the role table and append the row
    let table_header = "| 角色 | 花名 | 寓意";
    let alt_header = "| 角色 | 角色名字 | 角色定义";

    let new_content = if let Some(pos) = content.find(table_header).or_else(|| content.find(alt_header)) {
        // Find the end of this table — next blank line or next ## / --- section
        let after_header = &content[pos..];
        // Find the last row of the table before a blank line or section break
        let re = regex::Regex::new(r"(?m)^(\|[^|]+\|[^|]+\|[^|]+\|.*)$")
            .map_err(|e| e.to_string())?;
        let mut last_match_end = 0;
        for cap in re.captures_iter(after_header) {
            last_match_end = pos + cap.get(0).unwrap().end();
        }

        if last_match_end > 0 {
            // Insert after the last table row
            let mut result = content.clone();
            result.insert_str(last_match_end, &format!("\n{}", new_row));
            result
        } else {
            // Table header exists but no rows? Insert after header
            let header_end = content[pos..].find('\n').map(|n| pos + n + 1).unwrap_or(pos + 30);
            let mut result = content.clone();
            result.insert_str(header_end, &format!("{}\n", new_row));
            result
        }
    } else {
        // No role table found — create one at the end
        let table_block = format!(
            "\n\n## 角色配置\n\n| 角色 | 角色名字 | 角色定义 |\n|------|------|------|\n{}",
            new_row
        );
        format!("{}{}", content, table_block)
    };

    // Backup before writing
    let backup_path = format!("{}.bak.before_modelroute", file_path);
    std::fs::write(&backup_path, &content)
        .map_err(|e| format!("无法创建备份文件 {}：{}，请检查磁盘空间和目录权限", backup_path, e))?;

    std::fs::write(&file_path, &new_content)
        .map_err(|e| format!("无法写入文件 {}：{}，请检查文件权限，确保文件未被其他程序占用", file_path, e))?;

    Ok(true)
}

/// Get the default Claude.md path for the current user.
#[tauri::command]
pub fn get_default_claude_md_path() -> Result<String, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "无法获取用户主目录，请检查系统环境变量 HOME 或 USERPROFILE 是否正确配置".to_string())?;

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

/// Read ~/.claude/settings.json content for preview.
#[tauri::command]
pub fn get_claude_settings_json() -> Result<String, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "无法获取用户主目录".to_string())?;
    let settings_path = home.join(".claude").join("settings.json");

    if !settings_path.exists() {
        return Ok("{\n  // 尚未创建 settings.json 文件\n  // 应用模型配置后将自动生成\n}".to_string());
    }

    let raw = std::fs::read_to_string(&settings_path)
        .map_err(|e| format!("无法读取设置文件：{}", e))?;

    // Re-parse and pretty-print for consistent formatting
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(v) => serde_json::to_string_pretty(&v)
            .map_err(|e| format!("无法格式化 JSON：{}", e)),
        Err(_) => Ok(raw),
    }
}

/// Write API key and model config to ~/.claude/settings.json so Claude Code can use it.
#[tauri::command]
pub fn write_claude_code_env(
    api_key: String,
    base_url: Option<String>,
    model_id: String,
    provider: String,
) -> Result<bool, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "无法获取用户主目录".to_string())?;
    let settings_path = home.join(".claude").join("settings.json");

    // Read existing settings or create default
    let mut settings: Value = if settings_path.exists() {
        let raw = std::fs::read_to_string(&settings_path)
            .map_err(|e| format!("无法读取 Claude Code 设置文件：{}", e))?;
        serde_json::from_str(&raw).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // Build the env section
    if !settings.as_object().map(|o| o.contains_key("env")).unwrap_or(false) {
        if let Some(obj) = settings.as_object_mut() {
            obj.insert("env".to_string(), serde_json::json!({}));
        }
    }
    let env = settings.get_mut("env")
        .ok_or_else(|| "设置文件格式异常".to_string())?;

    match provider.as_str() {
        "anthropic" => {
            env["ANTHROPIC_AUTH_TOKEN"] = serde_json::Value::String(api_key.clone());
            if let Some(ref url) = base_url {
                env["ANTHROPIC_BASE_URL"] = serde_json::Value::String(url.clone());
            }
            env["ANTHROPIC_MODEL"] = serde_json::Value::String(model_id.clone());
            // Auto-set tier-specific model vars
            if model_id.contains("haiku") {
                env["ANTHROPIC_DEFAULT_HAIKU_MODEL"] = serde_json::Value::String(model_id.clone());
            } else if model_id.contains("sonnet") {
                env["ANTHROPIC_DEFAULT_SONNET_MODEL"] = serde_json::Value::String(model_id.clone());
            } else if model_id.contains("opus") {
                env["ANTHROPIC_DEFAULT_OPUS_MODEL"] = serde_json::Value::String(model_id.clone());
            }
            env["API_TIMEOUT_MS"] = serde_json::json!("3000000");
            env["CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC"] = serde_json::json!("1");
        }
        "openai" => {
            env["OPENAI_API_KEY"] = serde_json::Value::String(api_key.clone());
            if let Some(ref url) = base_url {
                env["OPENAI_BASE_URL"] = serde_json::Value::String(url.clone());
            }
            env["OPENAI_MODEL"] = serde_json::Value::String(model_id.clone());
        }
        "gemini" => {
            env["GEMINI_API_KEY"] = serde_json::Value::String(api_key.clone());
            if let Some(ref url) = base_url {
                env["GEMINI_BASE_URL"] = serde_json::Value::String(url.clone());
            }
        }
        // All other providers use Anthropic-compatible API
        // (deepseek, minimax, kimi, zhipu, stepfun, bailian, longcat, xiaomimimo,
        //  baidu, openrouter, siliconflow, nvidia, modelscope, custom)
        _ => {
            env["ANTHROPIC_AUTH_TOKEN"] = serde_json::Value::String(api_key.clone());
            if let Some(ref url) = base_url {
                env["ANTHROPIC_BASE_URL"] = serde_json::Value::String(url.clone());
            }
            env["ANTHROPIC_MODEL"] = serde_json::Value::String(model_id.clone());
            // Set tier-specific model vars based on the model name
            let model_lower = model_id.to_lowercase();
            if model_lower.contains("haiku") || model_lower.contains("flash") || model_lower.contains("mini") {
                env["ANTHROPIC_DEFAULT_HAIKU_MODEL"] = serde_json::Value::String(model_id.clone());
            } else if model_lower.contains("sonnet") || model_lower.contains("pro") {
                env["ANTHROPIC_DEFAULT_SONNET_MODEL"] = serde_json::Value::String(model_id.clone());
            } else {
                env["ANTHROPIC_DEFAULT_OPUS_MODEL"] = serde_json::Value::String(model_id.clone());
            }
            env["API_TIMEOUT_MS"] = serde_json::json!("3000000");
            env["CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC"] = serde_json::json!("1");
        }
    }

    // Write back
    let new_content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("无法序列化设置：{}", e))?;

    // Backup
    let backup_path = format!("{}.bak.modelrouter", settings_path.display());
    if settings_path.exists() {
        let old = std::fs::read_to_string(&settings_path).unwrap_or_default();
        std::fs::write(&backup_path, old).ok();
    }

    std::fs::write(&settings_path, new_content)
        .map_err(|e| format!("无法写入 Claude Code 设置文件 {}：{}", settings_path.display(), e))?;

    Ok(true)
}
