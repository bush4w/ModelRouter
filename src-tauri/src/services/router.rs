use crate::models::{ModelRecommendation, UserChoice, ModelInfo};
use std::collections::HashMap;

/// Static role-to-model mapping rules.
fn role_model_map() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("项目经理", vec!["claude-opus-4-7", "gpt-5.4"]),
        ("产品工程师", vec!["claude-sonnet-4-7", "gpt-5.4"]),
        ("产品架构师", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("UI设计师", vec!["claude-sonnet-4-7", "gemini-3.1-pro"]),
        ("前端工程师", vec!["claude-sonnet-4-7", "deepseek-v4-pro"]),
        ("后端工程师", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("数据库工程师", vec!["claude-opus-4-7", "gpt-5.4"]),
        ("外部系统集成工程师", vec!["claude-sonnet-4-7", "gpt-5.4"]),
        ("性能优化工程师", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("安全测试工程师", vec!["claude-opus-4-7", "gpt-5.4"]),
        ("运维工程师", vec!["claude-sonnet-4-7", "deepseek-v4-pro"]),
        ("售前工程师", vec!["gpt-5.4", "claude-sonnet-4-7"]),
        ("商务经理", vec!["gpt-5.4", "claude-sonnet-4-7"]),
    ])
}

/// Task-to-model weighting overrides.
fn task_model_weights() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("code-generation", vec!["claude-sonnet-4-7", "deepseek-v4-pro", "MiniMax-M2.7"]),
        ("code-review", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("debugging", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("documentation", vec!["gpt-5.4", "claude-sonnet-4-7"]),
        ("architecture", vec!["claude-opus-4-7", "gpt-5.4"]),
        ("data-analysis", vec!["gpt-5.4", "claude-sonnet-4-7"]),
        ("ui-design", vec!["claude-sonnet-4-7", "gemini-3.1-pro"]),
        ("security-review", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("performance", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("general", vec!["claude-sonnet-4-7", "gpt-5.4"]),
    ])
}

fn get_reasoning(role: &str, task_type: &str, model_id: &str) -> String {
    let task_label = match task_type {
        "code-generation" => "代码生成",
        "code-review" => "代码审查",
        "debugging" => "调试排错",
        "documentation" => "文档撰写",
        "architecture" => "架构设计",
        "data-analysis" => "数据分析",
        "ui-design" => "UI设计",
        "security-review" => "安全审查",
        "performance" => "性能优化",
        _ => "通用任务",
    };

    let model_reason = if model_id.contains("opus") {
        "需要最强推理能力，处理复杂逻辑"
    } else if model_id.contains("sonnet") {
        "平衡速度与质量，适合日常开发"
    } else if model_id.contains("gpt") || model_id.contains("o4") {
        "具备强大的通用理解和生成能力"
    } else if model_id.contains("deepseek") {
        "高性价比的代码生成模型，中文表现优异"
    } else if model_id.contains("MiniMax") {
        "出色的编程和复杂 Agent 任务处理能力"
    } else if model_id.contains("kimi") {
        "优秀的推理和中文生成能力"
    } else if model_id.contains("glm") {
        "中文场景表现突出，推理能力强"
    } else if model_id.contains("gemini") {
        "超长上下文窗口，支持多模态输入"
    } else if model_id.contains("qwen") {
        "中文优化，性价比高"
    } else {
        "匹配当前任务需求"
    };

    format!(
        "角色 [{}] 正在处理 [{}] 任务。推荐 {} 因为 {}。",
        role, task_label, model_id, model_reason
    )
}

/// Check if a model's provider is in the available list.
fn is_model_available(model_id: &str, available_providers: &[String]) -> bool {
    let provider = provider_from_model(model_id);
    available_providers.iter().any(|p| p == &provider)
}

/// Get model recommendation for a given role and task type.
/// Only recommends models from providers with configured API keys.
pub fn recommend(role: &str, task_type: &str, _learning_data: &[UserChoice], available_providers: &[String]) -> ModelRecommendation {
    let role_map = role_model_map();
    let task_map = task_model_weights();

    // Start with task-specific preferences, fall back to role preferences
    let candidates = task_map
        .get(task_type)
        .or_else(|| role_map.get(role))
        .or_else(|| {
            role_map.iter().find(|(k, _)| role.contains(*k)).map(|(_, v)| v)
        });

    // Filter candidates by available providers
    let available_candidates: Vec<&str> = match candidates {
        Some(list) => list.iter().filter(|m| is_model_available(m, available_providers)).copied().collect(),
        None => vec![],
    };

    let (primary, alternatives): (String, Vec<String>) = if !available_candidates.is_empty() {
        (
            available_candidates[0].to_string(),
            available_candidates.iter().skip(1).map(|s| s.to_string()).collect(),
        )
    } else if !available_providers.is_empty() {
        // No role/task match — pick the first available model from known models
        let builtin = get_builtin_models();
        let fallback = builtin.iter()
            .find(|m| available_providers.contains(&m.provider))
            .map(|m| m.id.clone())
            .unwrap_or_else(|| "claude-sonnet-4-7".to_string());
        (fallback, vec![])
    } else {
        // No API keys configured at all — return a generic suggestion
        ("claude-sonnet-4-7".to_string(), vec!["gpt-4o".to_string()])
    };

    let confidence = if role_map.contains_key(role) && !alternatives.is_empty() { 0.92 } else { 0.55 };

    ModelRecommendation {
        model_id: primary.clone(),
        provider: provider_from_model(&primary),
        reasoning: get_reasoning(role, task_type, &primary),
        confidence,
        alternatives,
    }
}

fn provider_from_model(model_id: &str) -> String {
    if model_id.contains("claude") {
        "anthropic".to_string()
    } else if model_id.contains("gpt") || model_id.contains("o1") || model_id.contains("o3") || model_id.contains("o4") {
        "openai".to_string()
    } else if model_id.contains("gemini") {
        "gemini".to_string()
    } else if model_id.contains("deepseek") || model_id.contains("DeepSeek") {
        "deepseek".to_string()
    } else if model_id.contains("MiniMax") || model_id.contains("minimax") {
        "minimax".to_string()
    } else if model_id.contains("kimi") || model_id.contains("Kimi") {
        "kimi".to_string()
    } else if model_id.contains("glm") || model_id.contains("GLM") {
        "zhipu".to_string()
    } else if model_id.contains("step-") || model_id.contains("Step") {
        "stepfun".to_string()
    } else if model_id.contains("qwen") || model_id.contains("Qwen") {
        "bailian".to_string()
    } else if model_id.contains("LongCat") || model_id.contains("longcat") {
        "longcat".to_string()
    } else if model_id.contains("mimo") || model_id.contains("MiMo") {
        "xiaomimimo".to_string()
    } else if model_id.contains("qianfan") || model_id.contains("Qianfan") {
        "baidu".to_string()
    } else if model_id.contains("ZhipuAI") || model_id.contains("moonshotai") {
        "siliconflow".to_string()
    } else {
        "custom".to_string()
    }
}

/// Returns the built-in list of models with metadata.
pub fn get_builtin_models() -> Vec<ModelInfo> {
    vec![
        // ===== Anthropic — Claude =====
        ModelInfo {
            id: "claude-opus-4-7".into(),
            name: "Claude Opus 4.7".into(),
            provider: "anthropic".into(),
            family: "claude".into(),
            context_length: 200000,
            capabilities: vec!["推理".into(), "代码审查".into(), "架构设计".into(), "安全分析".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "claude-sonnet-4-7".into(),
            name: "Claude Sonnet 4.7".into(),
            provider: "anthropic".into(),
            family: "claude".into(),
            context_length: 200000,
            capabilities: vec!["代码生成".into(), "日常开发".into(), "文档撰写".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "claude-haiku-4-5".into(),
            name: "Claude Haiku 4.5".into(),
            provider: "anthropic".into(),
            family: "claude".into(),
            context_length: 200000,
            capabilities: vec!["快速响应".into(), "简单任务".into()],
            cost_per_1k: None,
        },
        // ===== OpenAI — GPT =====
        ModelInfo {
            id: "gpt-5.4".into(),
            name: "GPT-5.4".into(),
            provider: "openai".into(),
            family: "gpt".into(),
            context_length: 128000,
            capabilities: vec!["推理".into(), "代码生成".into(), "通用".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "gpt-5.4-mini".into(),
            name: "GPT-5.4 Mini".into(),
            provider: "openai".into(),
            family: "gpt".into(),
            context_length: 128000,
            capabilities: vec!["快速响应".into(), "轻量任务".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "gpt-4o".into(),
            name: "GPT-4o".into(),
            provider: "openai".into(),
            family: "gpt".into(),
            context_length: 128000,
            capabilities: vec!["通用".into(), "创意写作".into(), "数据分析".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "gpt-4o-mini".into(),
            name: "GPT-4o Mini".into(),
            provider: "openai".into(),
            family: "gpt".into(),
            context_length: 128000,
            capabilities: vec!["轻量任务".into(), "快速响应".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "o4".into(),
            name: "o4".into(),
            provider: "openai".into(),
            family: "o-series".into(),
            context_length: 200000,
            capabilities: vec!["高级推理".into(), "数学".into(), "科学分析".into()],
            cost_per_1k: None,
        },
        // ===== Google — Gemini =====
        ModelInfo {
            id: "gemini-3.1-pro".into(),
            name: "Gemini 3.1 Pro".into(),
            provider: "gemini".into(),
            family: "gemini".into(),
            context_length: 2000000,
            capabilities: vec!["超长上下文".into(), "多模态".into(), "推理".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "gemini-3-flash".into(),
            name: "Gemini 3 Flash".into(),
            provider: "gemini".into(),
            family: "gemini".into(),
            context_length: 1000000,
            capabilities: vec!["快速响应".into(), "多模态".into(), "高性价比".into()],
            cost_per_1k: None,
        },
        // ===== DeepSeek =====
        ModelInfo {
            id: "deepseek-v4-pro".into(),
            name: "DeepSeek V4 Pro".into(),
            provider: "deepseek".into(),
            family: "deepseek".into(),
            context_length: 128000,
            capabilities: vec!["推理".into(), "代码生成".into(), "中文优化".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "deepseek-v4-flash".into(),
            name: "DeepSeek V4 Flash".into(),
            provider: "deepseek".into(),
            family: "deepseek".into(),
            context_length: 128000,
            capabilities: vec!["快速响应".into(), "高性价比".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "deepseek-v3".into(),
            name: "DeepSeek V3".into(),
            provider: "deepseek".into(),
            family: "deepseek".into(),
            context_length: 128000,
            capabilities: vec!["高性价比".into(), "代码生成".into()],
            cost_per_1k: None,
        },
        // ===== MiniMax =====
        ModelInfo {
            id: "MiniMax-M2.7".into(),
            name: "MiniMax M2.7".into(),
            provider: "minimax".into(),
            family: "minimax".into(),
            context_length: 128000,
            capabilities: vec!["编程".into(), "Agent".into(), "复杂任务".into()],
            cost_per_1k: None,
        },
        // ===== Kimi (月之暗面) =====
        ModelInfo {
            id: "kimi-k2.6".into(),
            name: "Kimi K2.6".into(),
            provider: "kimi".into(),
            family: "kimi".into(),
            context_length: 128000,
            capabilities: vec!["推理".into(), "代码生成".into(), "中文优化".into()],
            cost_per_1k: None,
        },
        // ===== 智谱 GLM =====
        ModelInfo {
            id: "glm-5".into(),
            name: "GLM-5".into(),
            provider: "zhipu".into(),
            family: "glm".into(),
            context_length: 128000,
            capabilities: vec!["推理".into(), "中文优化".into(), "多模态".into()],
            cost_per_1k: None,
        },
        // ===== 阶跃星辰 StepFun =====
        ModelInfo {
            id: "step-3.5-flash-2603".into(),
            name: "Step 3.5 Flash".into(),
            provider: "stepfun".into(),
            family: "step".into(),
            context_length: 128000,
            capabilities: vec!["快速推理".into(), "中文优化".into()],
            cost_per_1k: None,
        },
        // ===== 阿里百炼 =====
        ModelInfo {
            id: "qwen-max".into(),
            name: "通义千问 Max".into(),
            provider: "bailian".into(),
            family: "qwen".into(),
            context_length: 32768,
            capabilities: vec!["中文优化".into(), "通用任务".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "qwen-plus".into(),
            name: "通义千问 Plus".into(),
            provider: "bailian".into(),
            family: "qwen".into(),
            context_length: 131072,
            capabilities: vec!["中文优化".into(), "高性价比".into()],
            cost_per_1k: None,
        },
        // ===== LongCat =====
        ModelInfo {
            id: "LongCat-Flash-Chat".into(),
            name: "LongCat Flash Chat".into(),
            provider: "longcat".into(),
            family: "longcat".into(),
            context_length: 128000,
            capabilities: vec!["快速响应".into(), "代码生成".into()],
            cost_per_1k: None,
        },
        // ===== 小米 MiMo =====
        ModelInfo {
            id: "mimo-v2.5-pro".into(),
            name: "MiMo V2.5 Pro".into(),
            provider: "xiaomimimo".into(),
            family: "mimo".into(),
            context_length: 128000,
            capabilities: vec!["推理".into(), "中文优化".into()],
            cost_per_1k: None,
        },
        // ===== 百度千帆 =====
        ModelInfo {
            id: "qianfan-code-latest".into(),
            name: "千帆 Code".into(),
            provider: "baidu".into(),
            family: "qianfan".into(),
            context_length: 128000,
            capabilities: vec!["代码生成".into(), "中文优化".into(), "企业级".into()],
            cost_per_1k: None,
        },
        // ===== OpenRouter (聚合) =====
        ModelInfo {
            id: "anthropic/claude-opus-4-7".into(),
            name: "Claude Opus 4.7 (OR)".into(),
            provider: "openrouter".into(),
            family: "claude".into(),
            context_length: 200000,
            capabilities: vec!["推理".into(), "代码审查".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "anthropic/claude-sonnet-4-7".into(),
            name: "Claude Sonnet 4.7 (OR)".into(),
            provider: "openrouter".into(),
            family: "claude".into(),
            context_length: 200000,
            capabilities: vec!["代码生成".into(), "日常开发".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "anthropic/claude-haiku-4-5".into(),
            name: "Claude Haiku 4.5 (OR)".into(),
            provider: "openrouter".into(),
            family: "claude".into(),
            context_length: 200000,
            capabilities: vec!["快速响应".into(), "轻量任务".into()],
            cost_per_1k: None,
        },
        // ===== 硅基流动 (聚合) =====
        ModelInfo {
            id: "Pro/MiniMaxAI/MiniMax-M2.7".into(),
            name: "MiniMax M2.7 (SF)".into(),
            provider: "siliconflow".into(),
            family: "minimax".into(),
            context_length: 128000,
            capabilities: vec!["编程".into(), "Agent".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "deepseek-ai/DeepSeek-V4-Pro".into(),
            name: "DeepSeek V4 Pro (SF)".into(),
            provider: "siliconflow".into(),
            family: "deepseek".into(),
            context_length: 128000,
            capabilities: vec!["推理".into(), "代码生成".into()],
            cost_per_1k: None,
        },
    ]
}

/// Record a user choice for learning optimization.
pub fn record_choice(choice: &UserChoice, _storage: &mut dyn UserChoiceStorage) -> anyhow::Result<()> {
    _storage.save(choice)
}

pub trait UserChoiceStorage {
    fn save(&mut self, choice: &UserChoice) -> anyhow::Result<()>;
}

/// In-memory storage for user choices (to be replaced with SQLite in Phase 3).
pub struct InMemoryStorage {
    pub choices: Vec<UserChoice>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self { choices: vec![] }
    }
}

impl UserChoiceStorage for InMemoryStorage {
    fn save(&mut self, choice: &UserChoice) -> anyhow::Result<()> {
        self.choices.push(choice.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recommend_for_known_role() {
        let available = vec!["anthropic".to_string(), "openai".to_string()];
        let rec = recommend("前端工程师", "code-generation", &[], &available);
        assert_eq!(rec.model_id, "claude-sonnet-4-7");
        assert!(rec.confidence > 0.9);
        assert!(!rec.alternatives.is_empty());
    }

    #[test]
    fn test_recommend_for_unknown_role() {
        let available = vec!["anthropic".to_string()];
        let rec = recommend("未知角色", "general", &[], &available);
        assert!(rec.confidence < 1.0);
    }

    #[test]
    fn test_recommend_filters_by_provider() {
        // Only OpenAI configured — should not recommend Claude
        let available = vec!["openai".to_string()];
        let rec = recommend("前端工程师", "code-generation", &[], &available);
        // Should pick GPT-4o since Anthropic is not available
        assert_eq!(rec.model_id, "gpt-4o");
    }
}
