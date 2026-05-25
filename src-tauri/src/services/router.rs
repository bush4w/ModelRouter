use crate::models::{ModelRecommendation, UserChoice, ModelInfo};
use std::collections::HashMap;

/// Static role-to-model mapping rules.
fn role_model_map() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("项目经理", vec!["claude-opus-4-7", "gpt-4o"]),
        ("产品工程师", vec!["claude-sonnet-4-7", "gpt-4o"]),
        ("产品架构师", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("UI设计师", vec!["claude-sonnet-4-7", "gpt-4o"]),
        ("前端工程师", vec!["claude-sonnet-4-7", "gpt-4o-mini"]),
        ("后端工程师", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("数据库工程师", vec!["claude-opus-4-7", "gpt-4o"]),
        ("外部系统集成工程师", vec!["claude-sonnet-4-7", "gpt-4o"]),
        ("性能优化工程师", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("安全测试工程师", vec!["claude-opus-4-7", "gpt-4o"]),
        ("运维工程师", vec!["claude-sonnet-4-7", "gpt-4o-mini"]),
        ("售前工程师", vec!["gpt-4o", "claude-sonnet-4-7"]),
        ("商务经理", vec!["gpt-4o", "claude-sonnet-4-7"]),
    ])
}

/// Task-to-model weighting overrides.
fn task_model_weights() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("code-generation", vec!["claude-sonnet-4-7", "gpt-4o"]),
        ("code-review", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("debugging", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("documentation", vec!["gpt-4o", "claude-sonnet-4-7"]),
        ("architecture", vec!["claude-opus-4-7", "gpt-4o"]),
        ("data-analysis", vec!["gpt-4o", "claude-sonnet-4-7"]),
        ("ui-design", vec!["claude-sonnet-4-7", "gpt-4o"]),
        ("security-review", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("performance", vec!["claude-opus-4-7", "claude-sonnet-4-7"]),
        ("general", vec!["claude-sonnet-4-7", "gpt-4o"]),
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
    } else if model_id.contains("gpt") {
        "具备强大的通用理解和生成能力"
    } else {
        "匹配当前任务需求"
    };

    format!(
        "角色 [{}] 正在处理 [{}] 任务。推荐 {} 因为 {}。",
        role, task_label, model_id, model_reason
    )
}

/// Get model recommendation for a given role and task type.
/// Phase 1: static rule-based routing.
/// Phase 2+: will incorporate learning data from user_choices.
pub fn recommend(role: &str, task_type: &str, _learning_data: &[UserChoice]) -> ModelRecommendation {
    let role_map = role_model_map();
    let task_map = task_model_weights();

    // Start with task-specific preferences, fall back to role preferences
    let candidates = task_map
        .get(task_type)
        .or_else(|| role_map.get(role))
        .or_else(|| {
            // Partial match on role name
            role_map.iter().find(|(k, _)| role.contains(*k)).map(|(_, v)| v)
        });

    let (primary, alternatives) = match candidates {
        Some(list) if !list.is_empty() => (
            list[0].to_string(),
            list.iter().skip(1).map(|s| s.to_string()).collect::<Vec<_>>(),
        ),
        _ => (
            "claude-sonnet-4-7".to_string(),
            vec!["gpt-4o".to_string()],
        ),
    };

    let confidence = if role_map.contains_key(role) { 0.92 } else { 0.65 };

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
    } else if model_id.contains("gpt") || model_id.contains("o1") || model_id.contains("o3") {
        "openai".to_string()
    } else if model_id.contains("gemini") {
        "gemini".to_string()
    } else if model_id.contains("deepseek") {
        "deepseek".to_string()
    } else if model_id.contains("qwen") {
        "qwen".to_string()
    } else {
        "custom".to_string()
    }
}

/// Returns the known list of models with metadata.
pub fn get_model_list() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "claude-opus-4-7".into(),
            name: "Claude Opus 4.7".into(),
            provider: "anthropic".into(),
            context_length: 200000,
            capabilities: vec!["推理".into(), "代码审查".into(), "架构设计".into(), "安全分析".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "claude-sonnet-4-7".into(),
            name: "Claude Sonnet 4.7".into(),
            provider: "anthropic".into(),
            context_length: 200000,
            capabilities: vec!["代码生成".into(), "日常开发".into(), "文档撰写".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "claude-haiku-4-5".into(),
            name: "Claude Haiku 4.5".into(),
            provider: "anthropic".into(),
            context_length: 200000,
            capabilities: vec!["快速响应".into(), "简单任务".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "gpt-4o".into(),
            name: "GPT-4o".into(),
            provider: "openai".into(),
            context_length: 128000,
            capabilities: vec!["通用".into(), "创意写作".into(), "数据分析".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "gpt-4o-mini".into(),
            name: "GPT-4o Mini".into(),
            provider: "openai".into(),
            context_length: 128000,
            capabilities: vec!["轻量任务".into(), "快速响应".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "gemini-2.0-pro".into(),
            name: "Gemini 2.0 Pro".into(),
            provider: "gemini".into(),
            context_length: 2000000,
            capabilities: vec!["超长上下文".into(), "多模态".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "deepseek-v3".into(),
            name: "DeepSeek V3".into(),
            provider: "deepseek".into(),
            context_length: 128000,
            capabilities: vec!["高性价比".into(), "代码生成".into()],
            cost_per_1k: None,
        },
        ModelInfo {
            id: "qwen-max".into(),
            name: "通义千问 Max".into(),
            provider: "qwen".into(),
            context_length: 32768,
            capabilities: vec!["中文优化".into(), "通用任务".into()],
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
        let rec = recommend("前端工程师", "code-generation", &[]);
        assert_eq!(rec.model_id, "claude-sonnet-4-7");
        assert!(rec.confidence > 0.9);
        assert!(!rec.alternatives.is_empty());
    }

    #[test]
    fn test_recommend_for_unknown_role() {
        let rec = recommend("未知角色", "general", &[]);
        assert!(rec.confidence < 1.0);
    }
}
