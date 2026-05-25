import { useEffect, useState } from "react";
import { useAppStore } from "../store";
import * as api from "../services/api";
import type { TaskType, ModelConfig } from "../types";

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
  } = useAppStore();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (selectedRole) {
      fetchRecommendation();
    }
  }, [selectedRole, currentTaskType]);

  async function fetchRecommendation() {
    if (!selectedRole) return;
    setLoading(true);
    try {
      const rec = await api.getRecommendation(selectedRole.name, currentTaskType);
      setRecommendation(rec);
    } catch (err) {
      console.error("Failed to get recommendation:", err);
    } finally {
      setLoading(false);
    }
  }

  async function handleConfirm() {
    if (!currentRecommendation || !settings.claudeMdPath) return;
    const config: ModelConfig = {
      taskType: currentTaskType,
      recommendedModel: currentRecommendation.modelId,
      provider: currentRecommendation.provider,
      triggerCondition: `识别到 [${TASK_TYPES.find(t => t.value === currentTaskType)?.label}] 任务时自动切换`,
      updatedAt: new Date().toISOString(),
    };
    try {
      await api.writeModelConfig(settings.claudeMdPath, config);
      if (settings.learningMode) {
        await api.recordChoice({
          taskType: currentTaskType,
          role: selectedRole?.name,
          selectedModel: currentRecommendation.modelId,
          rejectedModels: [],
          timestamp: new Date().toISOString(),
        });
      }
      alert("配置已写入 Claude.md");
    } catch (err) {
      alert(`写入失败: ${err}`);
    }
  }

  if (!selectedRole) {
    return (
      <div className="recommendation-panel">
        <div className="panel-title">🤖 模型推荐</div>
        <div className="empty-state">请先选择左侧的角色</div>
      </div>
    );
  }

  return (
    <div className="recommendation-panel">
      <div className="panel-title">🤖 模型推荐 - {selectedRole.name}</div>

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
      ) : currentRecommendation ? (
        <div className="recommendation-card">
          <div className="recommendation-task">
            当前任务: {TASK_TYPES.find(t => t.value === currentTaskType)?.label}
          </div>
          <div className="recommendation-model">{currentRecommendation.modelId}</div>
          <div className="recommendation-meta">
            <span className="confidence-badge">
              置信度 {Math.round(currentRecommendation.confidence * 100)}%
            </span>
            <span style={{ color: "var(--color-text-muted)" }}>
              提供商: {currentRecommendation.provider}
            </span>
          </div>
          <div className="recommendation-reasoning">
            💡 {currentRecommendation.reasoning}
          </div>
          {currentRecommendation.alternatives.length > 0 && (
            <div style={{ marginBottom: 16, fontSize: 12, color: "var(--color-text-muted)" }}>
              备选: {currentRecommendation.alternatives.join(", ")}
            </div>
          )}
          <div className="recommendation-actions">
            <button className="btn btn-primary" onClick={handleConfirm}>
              ✓ 确认并应用
            </button>
            <button className="btn btn-secondary" onClick={fetchRecommendation}>
              🔄 重新推荐
            </button>
          </div>
        </div>
      ) : (
        <div className="empty-state">暂无推荐</div>
      )}
    </div>
  );
}
