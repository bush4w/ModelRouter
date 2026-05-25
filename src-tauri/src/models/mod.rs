use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub description: String,
    pub skills: Vec<String>,
    #[serde(rename = "filePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    #[serde(rename = "contextLength")]
    pub context_length: u32,
    pub capabilities: Vec<String>,
    #[serde(rename = "costPer1K", skip_serializing_if = "Option::is_none")]
    pub cost_per_1k: Option<ModelCost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCost {
    pub input: f64,
    pub output: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRecommendation {
    #[serde(rename = "modelId")]
    pub model_id: String,
    pub provider: String,
    pub reasoning: String,
    pub confidence: f32,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    #[serde(rename = "taskType")]
    pub task_type: String,
    #[serde(rename = "recommendedModel")]
    pub recommended_model: String,
    pub provider: String,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    pub provider: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "baseUrl", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    pub enabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "taskType")]
    pub task_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "selectedModel")]
    pub selected_model: String,
    #[serde(rename = "rejectedModels")]
    pub rejected_models: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<i32>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(rename = "autoWriteClaudeMd")]
    pub auto_write_claude_md: bool,
    #[serde(rename = "confirmBeforeSwitch")]
    pub confirm_before_switch: bool,
    #[serde(rename = "learningMode")]
    pub learning_mode: bool,
    #[serde(rename = "claudeMdPath")]
    pub claude_md_path: String,
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_write_claude_md: false,
            confirm_before_switch: true,
            learning_mode: false,
            claude_md_path: String::new(),
            language: "zh-CN".to_string(),
        }
    }
}
