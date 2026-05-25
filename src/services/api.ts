import { invoke } from "@tauri-apps/api/core";
import type {
  RoleInfo,
  ModelRecommendation,
  ModelConfig,
  ApiKeyConfig,
  UserChoice,
  ModelInfo,
  AppSettings,
  ModelProvider,
  TaskType,
} from "../types";

// 解析 Claude.md 文件，提取角色信息
export async function parseClaudeMd(filePath: string): Promise<RoleInfo[]> {
  return await invoke("parse_claude_md", { filePath });
}

// 获取模型推荐
export async function getRecommendation(
  role: string,
  taskType: TaskType
): Promise<ModelRecommendation> {
  return await invoke("get_recommendation", { role, taskType });
}

// 写入模型配置到 Claude.md
export async function writeModelConfig(
  filePath: string,
  config: ModelConfig
): Promise<boolean> {
  return await invoke("write_model_config", { filePath, config });
}

// 设置 API Key
export async function setApiKey(
  provider: ModelProvider,
  apiKey: string,
  baseUrl?: string
): Promise<boolean> {
  return await invoke("set_api_key", { provider, apiKey, baseUrl });
}

// 获取所有 API Key 配置
export async function getApiKeys(): Promise<ApiKeyConfig[]> {
  return await invoke("get_api_keys");
}

// 删除 API Key
export async function deleteApiKey(provider: ModelProvider): Promise<boolean> {
  return await invoke("delete_api_key", { provider });
}

// 记录用户选择
export async function recordChoice(choice: UserChoice): Promise<boolean> {
  return await invoke("record_choice", { choice });
}

// 获取可用模型列表
export async function getModelList(): Promise<ModelInfo[]> {
  return await invoke("get_model_list");
}

// 获取应用设置
export async function getSettings(): Promise<AppSettings> {
  return await invoke("get_settings");
}

// 更新应用设置
export async function updateSettings(settings: Partial<AppSettings>): Promise<boolean> {
  return await invoke("update_settings", { settings });
}

// 获取默认 Claude.md 路径
export async function getDefaultClaudeMdPath(): Promise<string> {
  return await invoke("get_default_claude_md_path");
}
