import { invoke } from "@tauri-apps/api/core";
import type {
  RoleInfo,
  ModelRecommendation,
  ModelConfig,
  ApiKeyConfig,
  UserChoice,
  ModelInfo,
  AppSettings,
  ApiProfile,
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
  taskType: TaskType,
  availableProviders: string[]
): Promise<ModelRecommendation> {
  return await invoke("get_recommendation", { role, taskType, availableProviders });
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

// API 配置模板管理
export async function createProfile(name: string): Promise<ApiProfile> {
  return await invoke("create_profile", { name });
}

export async function deleteProfile(id: string): Promise<boolean> {
  return await invoke("delete_profile", { id });
}

export async function switchProfile(id: string): Promise<boolean> {
  return await invoke("switch_profile", { id });
}

export async function listProfiles(): Promise<ApiProfile[]> {
  return await invoke("list_profiles");
}

export async function getActiveProfileId(): Promise<string> {
  return await invoke("get_active_profile_id");
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
  return await invoke("update_settings", { newSettings: settings });
}

// 获取默认 Claude.md 路径
export async function getDefaultClaudeMdPath(): Promise<string> {
  return await invoke("get_default_claude_md_path");
}

// 自定义模型管理
export async function addCustomModel(model: ModelInfo): Promise<boolean> {
  return await invoke("add_custom_model", { model });
}

export async function removeCustomModel(modelId: string): Promise<boolean> {
  return await invoke("remove_custom_model", { modelId });
}

export async function listCustomModels(): Promise<ModelInfo[]> {
  return await invoke("list_custom_models");
}

// 写入新角色到 Claude.md
export async function writeRole(
  filePath: string,
  name: string,
  alias: string,
  description: string,
  skills: string[]
): Promise<boolean> {
  return await invoke("write_role", { filePath, name, alias, description, skills });
}

// 从供应商 API 动态获取可用模型列表
export async function fetchProviderModels(
  provider: string,
  apiKey: string,
  baseUrl?: string
): Promise<ModelInfo[]> {
  return await invoke("fetch_provider_models", { provider, apiKey, baseUrl });
}

// 刷新所有已配置供应商的模型列表
export async function refreshAllFetchedModels(): Promise<string[]> {
  return await invoke("refresh_all_fetched_models");
}

// 写入 API Key 和模型配置到 Claude Code settings.json
export async function writeClaudeCodeEnv(
  apiKey: string,
  baseUrl: string | undefined,
  modelId: string,
  provider: string
): Promise<boolean> {
  return await invoke("write_claude_code_env", { apiKey, baseUrl, modelId, provider });
}

// 获取 Claude Code settings.json 内容
export async function getClaudeSettingsJson(): Promise<string> {
  return await invoke("get_claude_settings_json");
}

// 读取文件内容用于预览（绕过 fs plugin scope）
export async function readFileContent(filePath: string): Promise<string> {
  return await invoke("read_file_content", { filePath });
}
