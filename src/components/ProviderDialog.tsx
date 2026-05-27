import { useState, useEffect } from "react";
import type { ModelProvider } from "../types";
import { CloseIcon, KeyIcon, EyeIcon, EyeOffIcon, ServerIcon, CheckIcon } from "./Icons";

interface ProviderDialogProps {
  provider?: ModelProvider;
  existingKey?: string;
  existingUrl?: string;
  onSave: (provider: ModelProvider, apiKey: string, baseUrl: string) => Promise<void>;
  onClose: () => void;
}

const PROVIDERS: { value: ModelProvider; label: string; color: string; icon: string; placeholder: string; defaultUrl: string; models: { id: string; name: string }[] }[] = [
  {
    value: "anthropic", label: "Anthropic (官方)", color: "#F97316", icon: "🧠",
    placeholder: "sk-ant-api03-...", defaultUrl: "https://api.anthropic.com",
    models: [
      { id: "claude-opus-4-7", name: "Claude Opus 4.7 — 最强推理" },
      { id: "claude-sonnet-4-7", name: "Claude Sonnet 4.7 — 日常开发" },
      { id: "claude-haiku-4-5", name: "Claude Haiku 4.5 — 快速响应" },
    ],
  },
  {
    value: "openai", label: "OpenAI (官方)", color: "#10A37F", icon: "⚡",
    placeholder: "sk-proj-...", defaultUrl: "https://api.openai.com/v1",
    models: [
      { id: "gpt-4o", name: "GPT-4o — 通用全能" },
      { id: "gpt-4o-mini", name: "GPT-4o Mini — 轻量快速" },
      { id: "gpt-5.4", name: "GPT-5.4 — 最新旗舰" },
      { id: "o4", name: "o4 — 高级推理" },
    ],
  },
  {
    value: "gemini", label: "Google Gemini (官方)", color: "#4285F4", icon: "🌐",
    placeholder: "AIza...", defaultUrl: "https://generativelanguage.googleapis.com",
    models: [
      { id: "gemini-3.1-pro", name: "Gemini 3.1 Pro — 旗舰模型" },
      { id: "gemini-3-flash", name: "Gemini 3 Flash — 轻量快速" },
    ],
  },
  {
    value: "deepseek", label: "DeepSeek", color: "#6366F1", icon: "🔍",
    placeholder: "sk-...", defaultUrl: "https://api.deepseek.com/anthropic",
    models: [
      { id: "deepseek-v4-pro", name: "DeepSeek V4 Pro — 旗舰推理" },
      { id: "deepseek-v4-flash", name: "DeepSeek V4 Flash — 快速响应" },
      { id: "deepseek-v3", name: "DeepSeek V3 — 高性价比" },
    ],
  },
  {
    value: "minimax", label: "MiniMax", color: "#F64551", icon: "💎",
    placeholder: "sk-...", defaultUrl: "https://api.minimaxi.com/anthropic",
    models: [
      { id: "MiniMax-M2.7", name: "MiniMax M2.7 — 旗舰编程" },
    ],
  },
  {
    value: "kimi", label: "Kimi (月之暗面)", color: "#6366F1", icon: "🌙",
    placeholder: "sk-...", defaultUrl: "https://api.moonshot.cn/anthropic",
    models: [
      { id: "kimi-k2.6", name: "Kimi K2.6 — 旗舰模型" },
    ],
  },
  {
    value: "zhipu", label: "智谱 GLM", color: "#0F62FE", icon: "🏯",
    placeholder: "your-api-key", defaultUrl: "https://open.bigmodel.cn/api/anthropic",
    models: [
      { id: "glm-5", name: "GLM-5 — 旗舰模型" },
    ],
  },
  {
    value: "stepfun", label: "阶跃星辰 StepFun", color: "#16D6D2", icon: "⚡",
    placeholder: "sk-...", defaultUrl: "https://api.stepfun.com/step_plan",
    models: [
      { id: "step-3.5-flash-2603", name: "Step 3.5 Flash — 快速推理" },
    ],
  },
  {
    value: "bailian", label: "阿里百炼", color: "#624AFF", icon: "☁️",
    placeholder: "sk-...", defaultUrl: "https://dashscope.aliyuncs.com/apps/anthropic",
    models: [
      { id: "qwen-max", name: "通义千问 Max" },
      { id: "qwen-plus", name: "通义千问 Plus" },
    ],
  },
  {
    value: "longcat", label: "LongCat", color: "#29E154", icon: "🐱",
    placeholder: "sk-...", defaultUrl: "https://api.longcat.chat/anthropic",
    models: [
      { id: "LongCat-Flash-Chat", name: "LongCat Flash Chat" },
    ],
  },
  {
    value: "xiaomimimo", label: "小米 MiMo", color: "#FF6900", icon: "📱",
    placeholder: "sk-...", defaultUrl: "https://api.xiaomimimo.com/anthropic",
    models: [
      { id: "mimo-v2.5-pro", name: "MiMo V2.5 Pro — 旗舰模型" },
    ],
  },
  {
    value: "baidu", label: "百度千帆", color: "#2932E1", icon: "🌊",
    placeholder: "your-api-key", defaultUrl: "https://qianfan.baidubce.com/anthropic/coding",
    models: [
      { id: "qianfan-code-latest", name: "千帆 Code — 编程专用" },
    ],
  },
  {
    value: "openrouter", label: "OpenRouter", color: "#6566F1", icon: "🔀",
    placeholder: "sk-or-...", defaultUrl: "https://openrouter.ai/api",
    models: [
      { id: "anthropic/claude-opus-4-7", name: "Claude Opus 4.7" },
      { id: "anthropic/claude-sonnet-4-7", name: "Claude Sonnet 4.7" },
      { id: "anthropic/claude-haiku-4-5", name: "Claude Haiku 4.5" },
    ],
  },
  {
    value: "siliconflow", label: "硅基流动", color: "#6E29F6", icon: "💜",
    placeholder: "sk-...", defaultUrl: "https://api.siliconflow.cn",
    models: [
      { id: "Pro/MiniMaxAI/MiniMax-M2.7", name: "MiniMax M2.7" },
      { id: "deepseek-ai/DeepSeek-V4-Pro", name: "DeepSeek V4 Pro" },
    ],
  },
  {
    value: "custom", label: "自定义 (Anthropic 兼容)", color: "#6B7280", icon: "🔧",
    placeholder: "your-api-key", defaultUrl: "",
    models: [],
  },
];

export default function ProviderDialog({ provider: initialProvider, existingKey, existingUrl, onSave, onClose }: ProviderDialogProps) {
  const isEditing = !!initialProvider;
  const [selectedPreset, setSelectedPreset] = useState<ModelProvider>(initialProvider || "anthropic");
  const [apiKey, setApiKey] = useState(existingKey || "");
  const [baseUrl, setBaseUrl] = useState(existingUrl || "");
  const [showKey, setShowKey] = useState(false);
  const [saving, setSaving] = useState(false);

  const preset = PROVIDERS.find((p) => p.value === selectedPreset);

  useEffect(() => {
    // Auto-fill base URL from preset when provider changes (only when adding new)
    if (!isEditing && preset) {
      setBaseUrl(preset.defaultUrl);
    }
  }, [selectedPreset, isEditing]);

  async function handleSave() {
    if (!apiKey.trim()) {
      alert("请输入 API Key");
      return;
    }
    if (selectedPreset === "custom" && !baseUrl.trim()) {
      alert("自定义供应商需要填写 Base URL");
      return;
    }
    setSaving(true);
    try {
      await onSave(selectedPreset, apiKey.trim(), baseUrl.trim() || preset?.defaultUrl || "");
      onClose();
    } catch (err) {
      alert(`保存失败: ${err}`);
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal provider-dialog" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <div className="modal-title">
            <KeyIcon size={18} />
            {isEditing ? `编辑 ${preset?.label} 配置` : "添加供应商"}
          </div>
          <button className="btn-icon" onClick={onClose}><CloseIcon size={18} /></button>
        </div>
        <div className="modal-body">
          {/* Provider Preset Selector */}
          <div className="form-group">
            <label className="form-label">供应商</label>
            <div className="preset-list">
              {PROVIDERS.map((p) => (
                <button
                  key={p.value}
                  className={`preset-item ${selectedPreset === p.value ? "active" : ""}`}
                  disabled={isEditing}
                  onClick={() => setSelectedPreset(p.value)}
                  style={{
                    borderColor: selectedPreset === p.value ? p.color : "var(--color-border)",
                  }}
                >
                  <span className="preset-icon" style={{ background: `${p.color}18`, color: p.color }}>
                    {p.icon}
                  </span>
                  <span className="preset-label">{p.label}</span>
                  {selectedPreset === p.value && (
                    <span className="preset-check" style={{ color: p.color }}>
                      <CheckIcon size={14} />
                    </span>
                  )}
                </button>
              ))}
            </div>
          </div>

          {/* API Key Input */}
          <div className="form-group">
            <label className="form-label">API Key {!isEditing && <span style={{ color: "var(--color-danger)" }}>*</span>}</label>
            <div className="provider-input-row">
              <input
                type={showKey ? "text" : "password"}
                className="form-input"
                placeholder={preset?.placeholder || "输入 API Key"}
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                autoFocus
              />
              <button
                className="btn-icon toggle-vis"
                onClick={() => setShowKey(!showKey)}
                title={showKey ? "隐藏" : "显示"}
              >
                {showKey ? <EyeOffIcon size={16} /> : <EyeIcon size={16} />}
              </button>
            </div>
          </div>

          {/* Base URL */}
          <div className="form-group">
            <label className="form-label">
              <ServerIcon size={12} style={{ marginRight: 4 }} />
              Base URL
            </label>
            <input
              type="text"
              className="form-input"
              placeholder={preset?.defaultUrl || "https://api.example.com/v1"}
              value={baseUrl}
              onChange={(e) => setBaseUrl(e.target.value)}
            />
            {preset?.defaultUrl && (
              <div className="form-hint">
                默认：{preset.defaultUrl}
                <button
                  className="btn-link"
                  onClick={() => setBaseUrl(preset.defaultUrl)}
                >
                  使用默认
                </button>
              </div>
            )}
          </div>

          {/* Available Models for this provider */}
          {preset && preset.models.length > 0 && (
            <div className="form-group">
              <label className="form-label">可用子型号</label>
              <div className="provider-models-list">
                {preset.models.map((m) => (
                  <div key={m.id} className="provider-model-item">
                    <span className="provider-model-id">{m.id}</span>
                    <span className="provider-model-desc">{m.name}</span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
        <div className="modal-footer">
          <button className="btn btn-secondary" onClick={onClose}>取消</button>
          <button className="btn btn-primary" onClick={handleSave} disabled={saving}>
            {saving ? "保存中..." : isEditing ? "更新配置" : "添加供应商"}
          </button>
        </div>
      </div>
    </div>
  );
}
