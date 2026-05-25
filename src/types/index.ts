// 角色信息
export interface RoleInfo {
  name: string;          // 角色名称（如 "项目经理"）
  alias?: string;        // 花名（如 "周明"）
  description: string;   // 角色描述
  skills: string[];      // 技能列表
  filePath?: string;     // 来源文件路径
}

// AI 模型提供商
export type ModelProvider = "anthropic" | "openai" | "gemini" | "deepseek" | "qwen" | "custom";

// 模型信息
export interface ModelInfo {
  id: string;            // 模型 ID（如 "claude-opus-4-7"）
  name: string;          // 显示名称
  provider: ModelProvider;
  contextLength: number; // 上下文长度
  capabilities: string[]; // 能力标签
  costPer1K?: {
    input: number;
    output: number;
  };
}

// API Key 配置
export interface ApiKeyConfig {
  provider: ModelProvider;
  apiKey: string;
  baseUrl?: string;      // 自定义端点
  enabled: boolean;
  createdAt: string;
}

// 模型推荐
export interface ModelRecommendation {
  modelId: string;
  provider: ModelProvider;
  reasoning: string;     // 推荐理由
  confidence: number;    // 置信度 0-1
  alternatives: string[]; // 备选模型
}

// 任务类型
export type TaskType =
  | "code-generation"   // 代码生成
  | "code-review"        // 代码审查
  | "debugging"         // 调试
  | "documentation"     // 文档撰写
  | "architecture"       // 架构设计
  | "data-analysis"      // 数据分析
  | "ui-design"          // UI 设计
  | "security-review"    // 安全审查
  | "performance"        // 性能优化
  | "general";           // 通用

// 模型配置（写入 Claude.md）
export interface ModelConfig {
  taskType: TaskType;
  recommendedModel: string;
  provider: ModelProvider;
  triggerCondition: string;
  updatedAt: string;
}

// 用户选择记录（学习数据）
export interface UserChoice {
  id?: number;
  taskType: TaskType;
  role?: string;
  selectedModel: string;
  rejectedModels: string[];
  feedback?: number;     // 1-5 评分
  timestamp: string;
}

// 应用设置
export interface AppSettings {
  autoWriteClaudeMd: boolean;      // 自动写入 Claude.md
  confirmBeforeSwitch: boolean;    // 切换前确认
  learningMode: boolean;            // 学习模式
  claudeMdPath: string;             // Claude.md 路径
  language: "zh-CN" | "en-US";
}
