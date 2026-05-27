use crate::models::ModelInfo;
use serde::Deserialize;

/// Anthropic list models response
#[derive(Debug, Deserialize)]
struct AnthropicResp {
    data: Vec<AnthropicModel>,
}

#[derive(Debug, Deserialize)]
struct AnthropicModel {
    id: String,
    #[serde(default)]
    display_name: String,
}

/// OpenAI-compatible list models response
#[derive(Debug, Deserialize)]
struct OpenAIResp {
    data: Vec<OpenAIModel>,
}

#[derive(Debug, Deserialize)]
struct OpenAIModel {
    id: String,
    #[serde(default)]
    #[allow(dead_code)]
    owned_by: String,
}

/// Gemini list models response
#[derive(Debug, Deserialize)]
struct GeminiResp {
    models: Vec<GeminiModel>,
}

#[derive(Debug, Deserialize)]
struct GeminiModel {
    name: String,
    #[serde(default)]
    #[allow(dead_code)]
    display_name: String,
}

fn known_model_meta(id: &str) -> Option<(String, String, u32, Vec<String>)> {
    let builtin = crate::services::router::get_builtin_models();
    builtin.into_iter().find(|m| m.id == id).map(|m| {
        (m.family, m.name, m.context_length, m.capabilities)
    })
}

fn infer_model_info(id: &str, name: &str, provider: &str) -> ModelInfo {
    // Try to find known metadata from builtin models
    if let Some((family, display_name, ctx_len, caps)) = known_model_meta(id) {
        ModelInfo {
            id: id.to_string(),
            name: display_name,
            provider: provider.to_string(),
            family,
            context_length: ctx_len,
            capabilities: caps,
            cost_per_1k: None,
        }
    } else {
        // Infer family from id pattern
        let family = if id.contains("opus") {
            "opus"
        } else if id.contains("sonnet") {
            "sonnet"
        } else if id.contains("haiku") {
            "haiku"
        } else if id.contains("gpt") {
            "gpt"
        } else if id.contains("gemini") {
            "gemini"
        } else if id.contains("deepseek") {
            "deepseek"
        } else if id.contains("qwen") {
            "qwen"
        } else {
            provider
        };

        ModelInfo {
            id: id.to_string(),
            name: name.to_string(),
            provider: provider.to_string(),
            family: family.to_string(),
            context_length: 128000,
            capabilities: vec![],
            cost_per_1k: None,
        }
    }
}

pub async fn fetch_anthropic_models(api_key: &str, base_url: &str) -> Result<Vec<ModelInfo>, String> {
    let url = format!("{}/v1/models", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| format!("Anthropic API 请求失败：{}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Anthropic API 返回错误 {}：{}", status, body));
    }

    let data: AnthropicResp = resp.json().await
        .map_err(|e| format!("Anthropic API 响应解析失败：{}", e))?;

    let models: Vec<ModelInfo> = data.data.into_iter()
        .filter(|m| m.id.starts_with("claude-"))
        .map(|m| {
            let name = if m.display_name.is_empty() { m.id.clone() } else { m.display_name.clone() };
            infer_model_info(&m.id, &name, "anthropic")
        })
        .collect();

    Ok(models)
}

pub async fn fetch_openai_models(api_key: &str, base_url: &str) -> Result<Vec<ModelInfo>, String> {
    let url = format!("{}/v1/models", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| format!("OpenAI API 请求失败：{}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("OpenAI API 返回错误 {}：{}", status, body));
    }

    let data: OpenAIResp = resp.json().await
        .map_err(|e| format!("OpenAI API 响应解析失败：{}", e))?;

    // OpenAI returns ALL models — filter for current GPT models
    let keep_prefixes = ["gpt-4o", "gpt-4.1", "gpt-5", "o1", "o3", "o4"];
    let models: Vec<ModelInfo> = data.data.into_iter()
        .filter(|m| keep_prefixes.iter().any(|p| m.id.starts_with(p)) && !m.id.contains("audio") && !m.id.contains("realtime") && !m.id.contains("transcribe") && !m.id.contains("tts"))
        .map(|m| {
            let name = m.id.clone();
            infer_model_info(&m.id, &name, "openai")
        })
        .collect();

    Ok(models)
}

pub async fn fetch_gemini_models(api_key: &str, _base_url: &str) -> Result<Vec<ModelInfo>, String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models?key={}",
        api_key
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| format!("Gemini API 请求失败：{}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Gemini API 返回错误 {}：{}", status, body));
    }

    let data: GeminiResp = resp.json().await
        .map_err(|e| format!("Gemini API 响应解析失败：{}", e))?;

    let models: Vec<ModelInfo> = data.models.into_iter()
        .filter(|m| {
            // Keep only generate models, exclude embedding/vision-only etc.
            m.name.contains("gemini") && !m.name.contains("embedding")
        })
        .map(|m| {
            // Gemini returns "models/gemini-2.0-pro", strip prefix
            let id = m.name.strip_prefix("models/").unwrap_or(&m.name).to_string();
            infer_model_info(&id, &id, "gemini")
        })
        .collect();

    Ok(models)
}

/// Fetch models from DeepSeek, Qwen, or custom OpenAI-compatible providers.
/// Uses OpenAI-compatible /v1/models endpoint.
pub async fn fetch_openai_compatible_models(api_key: &str, base_url: &str, provider: &str) -> Result<Vec<ModelInfo>, String> {
    let url = format!("{}/v1/models", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| format!("{} API 请求失败：{}", provider, e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("{} API 返回错误 {}：{}", provider, status, body));
    }

    let data: OpenAIResp = resp.json().await
        .map_err(|e| format!("{} API 响应解析失败：{}", provider, e))?;

    let models: Vec<ModelInfo> = data.data.into_iter()
        .filter(|m| {
            // Exclude embedding, moderation, and other non-chat models
            let id = m.id.to_lowercase();
            !id.contains("embedding") && !id.contains("moderation") && !id.contains("dall-e") && !id.contains("tts") && !id.contains("whisper")
        })
        .map(|m| {
            let name = m.id.clone();
            infer_model_info(&m.id, &name, provider)
        })
        .collect();

    Ok(models)
}

/// Fetch models from a provider's actual model API.
///
/// **IMPORTANT**: The model listing URL is DIFFERENT from the chat/Anthropic-compatible
/// base URL the user configures. Each provider has a well-known models endpoint:
///
/// | Provider    | Models API                                    | Auth Type     |
/// |-------------|-----------------------------------------------|---------------|
/// | Anthropic   | https://api.anthropic.com/v1/models           | x-api-key     |
/// | OpenAI      | https://api.openai.com/v1/models              | Bearer        |
/// | DeepSeek    | https://api.deepseek.com/models               | Bearer+json   |
/// | Gemini      | https://generativelanguage.googleapis.com/... | query key     |
/// | Zhipu       | https://open.bigmodel.cn/api/paas/v4/models   | Bearer        |
/// | Kimi        | https://api.moonshot.cn/v1/models             | Bearer        |
/// | Others      | Base URL + /v1/models (OpenAI-compatible)     | Bearer        |
pub async fn fetch_provider_models(provider: &str, api_key: &str, base_url: Option<&str>) -> Result<Vec<ModelInfo>, String> {
    match provider {
        "anthropic" => {
            fetch_anthropic_models(api_key, "https://api.anthropic.com").await
        }
        "openai" => {
            fetch_openai_models(api_key, "https://api.openai.com").await
        }
        "deepseek" => {
            // DeepSeek models endpoint uses OpenAI-compatible format at /models (not /v1/models)
            fetch_openai_compatible_models(api_key, "https://api.deepseek.com", "deepseek").await
        }
        "gemini" => {
            fetch_gemini_models(api_key, "").await
        }
        "zhipu" => {
            // Zhipu uses its own API format
            fetch_openai_compatible_models(api_key, "https://open.bigmodel.cn/api/paas/v4", "zhipu").await
        }
        "kimi" => {
            // Kimi uses OpenAI-compatible API
            fetch_openai_compatible_models(api_key, "https://api.moonshot.cn", "kimi").await
        }
        "minimax" | "stepfun" | "bailian" | "longcat" | "xiaomimimo" | "baidu" | "openrouter" | "siliconflow" | "nvidia" | "modelscope" => {
            // These providers don't have a standard public model listing API.
            // Return builtin models only (merge happens in get_model_list).
            return Ok(vec![]);
        }
        _ => {
            let base = base_url.ok_or_else(|| "自定义供应商需要提供 Base URL".to_string())?;
            fetch_openai_compatible_models(api_key, base, provider).await
        }
    }
}
