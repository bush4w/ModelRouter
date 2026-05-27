import { useEffect, useState } from "react";
import { useAppStore } from "../store";
import * as api from "../services/api";
import type { TaskType, ModelConfig, ModelInfo } from "../types";
import { BotIcon, RefreshIcon, CheckIcon, LightbulbIcon } from "./Icons";

const TASK_TYPES: { value: TaskType; label: string }[] = [
  { value: "general", label: "通用任务" },
  { value: "code-generation", label: "代码生成" },
  { value: "code-review", label: "代码审查" },
  { value: "debugging", label: "调试排错" },
  { value: "documentation", label: "文档撰写" },
  { value: "architecture", label: "架构设计" },
  { value: "data-analysis", label: "数据分析" },
  { value: "ui-design", label: "UI 设计" },
  { value: "security-review", label: "安全审查" },
  { value: "performance", label: "性能优化" },
];

export default function ModelRecommendationPanel() {
  const {
    selectedRole,
    currentTaskType,
    setTaskType,
    currentRecommendation,
    setRecommendation,
    settings,
    apiKeys,
  } = useAppStore();
  const [loading, setLoading] = useState(false);
  const [selectedModelId, setSelectedModelId] = useState("");
  const [availableModels, setAvailableModels] = useState<ModelInfo[]>([]);

  const configuredProviders = [...new Set(apiKeys.filter(k => k.enabled).map(k => k.provider))];

  useEffect(() => {
    loadModels();
  }, [apiKeys]);

  useEffect(() => {
    if (selectedRole) {
      fetchRecommendation();
    }
  }, [selectedRole, currentTaskType, apiKeys]);

  useEffect(() => {
    if (currentRecommendation) {
      setSelectedModelId(currentRecommendation.modelId);
    }
  }, [currentRecommendation]);

  async function loadModels() {
    try {
      const all = await api.getModelList();
      setAvailableModels(all.filter(m => configuredProviders.includes(m.provider)));
    } catch (err) {
      console.error("Failed to load models:", err);
    }
  }

  async function fetchRecommendation() {
    if (!selectedRole) return;
    setLoading(true);
    try {
      const rec = await api.getRecommendation(selectedRole.name, currentTaskType, configuredProviders);
      setRecommendation(rec);
    } catch (err) {
      console.error("Failed to get recommendation:", err);
    } finally {
      setLoading(false);
    }
  }

  // Group available models by provider for the dropdown
  const modelsByProvider: Record<string, ModelInfo[]> = {};
  for (const m of availableModels) {
    if (!modelsByProvider[m.provider]) modelsByProvider[m.provider] = [];
    modelsByProvider[m.provider].push(m);
  }

  const selectedModel = availableModels.find(m => m.id === selectedModelId);

  async function handleConfirm() {
    if (!selectedModel) {
      alert("请先选择一个模型");
      return;
    }
    if (!settings.claudeMdPath) {
      alert("请先在设置中配置 Claude.md 文件路径（点击右上角 ⚙️ 图标）");
      return;
    }
    const keys = await api.getApiKeys();
    const providerKey = keys.find(
      (k) => k.provider === selectedModel.provider && k.enabled
    );
    if (!providerKey) {
      alert(
        `❌ 未配置 API Key\n\n` +
        `当前选择的模型 "${selectedModel.id}" 需要 "${selectedModel.provider}" 提供商的 API Key，但尚未配置。\n\n` +
        `请先前往设置（右上角 ⚙️ 图标）→ API Key 配置中添加对应的 API Key。`
      );
      return;
    }
    const config: ModelConfig = {
      taskType: currentTaskType,
      recommendedModel: selectedModel.id,
      provider: selectedModel.provider,
      triggerCondition: `识别到 [${TASK_TYPES.find(t => t.value === currentTaskType)?.label}] 任务时自动切换`,
      updatedAt: new Date().toISOString(),
    };
    try {
      await api.writeModelConfig(settings.claudeMdPath, config);
      await api.writeClaudeCodeEnv(
        providerKey.apiKey,
        providerKey.baseUrl || undefined,
        selectedModel.id,
        selectedModel.provider
      );
      if (settings.learningMode) {
        await api.recordChoice({
          taskType: currentTaskType,
          role: selectedRole?.name,
          selectedModel: selectedModel.id,
          rejectedModels: [],
          timestamp: new Date().toISOString(),
        });
      }
      alert(`✓ 已完成：\n\n1. 模型配置已写入 CLAUDE.md\n2. API Key + 模型 "${selectedModel.id}" 已写入 Claude Code 设置\n\n重启 Claude Code 会话后生效。`);
    } catch (err) {
      const errMsg = String(err);
      alert(`写入失败：${errMsg}`);
    }
  }

  if (!selectedRole) {
    return (
      <div className="recommendation-panel">
        <div className="panel-title">
          <BotIcon size={16} />
          模型推荐
        </div>
        <div className="empty-state">请先选择左侧的角色</div>
      </div>
    );
  }

  return (
    <div className="recommendation-panel">
      <div className="panel-title">
        <BotIcon size={16} />
        模型推荐 · {selectedRole.name}
      </div>

      {configuredProviders.length === 0 && (
        <div className="provider-warning">
          ⚠️ 尚未配置任何 API Key，请先前往设置页面配置 API Key 后再选择模型。
        </div>
      )}

      <div className="form-group" style={{ marginBottom: 16 }}>
        <label className="form-label">任务类型</label>
        <select
          className="form-input"
          value={currentTaskType}
          onChange={(e) => setTaskType(e.target.value as TaskType)}
        >
          {TASK_TYPES.map((t) => (
            <option key={t.value} value={t.value}>
              {t.label}
            </option>
          ))}
        </select>
      </div>

      {loading ? (
        <div className="empty-state">分析中...</div>
      ) : currentRecommendation || availableModels.length > 0 ? (
        <div className="recommendation-card">
          {/* Model Selector */}
          <div className="form-group">
            <label className="form-label">选择模型</label>
            <select
              className="form-input model-select"
              value={selectedModelId}
              onChange={(e) => setSelectedModelId(e.target.value)}
            >
              {Object.entries(modelsByProvider).map(([provider, models]) => (
                <optgroup key={provider} label={provider.toUpperCase()}>
                  {models.map((m) => (
                    <option key={m.id} value={m.id}>
                      {m.name} ({m.family})
                    </option>
                  ))}
                </optgroup>
              ))}
              {availableModels.length === 0 && (
                <option value="" disabled>暂无可用模型，请先配置 API Key</option>
              )}
            </select>
          </div>

          {currentRecommendation && (
            <>
              <div className="recommendation-meta">
                <span className="confidence-badge">
                  推荐置信度 {Math.round(currentRecommendation.confidence * 100)}%
                </span>
                <span style={{ color: "var(--color-text-muted)" }}>
                  提供商: {selectedModel?.provider || currentRecommendation.provider}
                </span>
              </div>
              <div className="recommendation-reasoning">
                <LightbulbIcon size={14} style={{ marginRight: 4, flexShrink: 0, marginTop: 2 }} />
                {currentRecommendation.reasoning}
              </div>
              {currentRecommendation.alternatives.length > 0 && (
                <div style={{ marginBottom: 16, fontSize: 12, color: "var(--color-text-muted)" }}>
                  备选: {currentRecommendation.alternatives.join(", ")}
                </div>
              )}
            </>
          )}
          <div className="recommendation-actions">
            <button className="btn btn-primary" onClick={handleConfirm}>
              <CheckIcon size={15} />
              确认并应用
            </button>
            <button className="btn btn-secondary" onClick={fetchRecommendation}>
              <RefreshIcon size={15} />
              重新推荐
            </button>
          </div>
        </div>
      ) : (
        <div className="empty-state">暂无推荐</div>
      )}
    </div>
  );
}
