import { useEffect, useState } from "react";
import { useAppStore } from "../store";
import * as api from "../services/api";
import { open } from "@tauri-apps/plugin-dialog";
import type { ModelProvider, ModelInfo, ApiProfile } from "../types";
import { GearIcon, CloseIcon, KeyIcon, SlidersIcon, FolderIcon, CheckIcon, PlusIcon, TrashIcon, BotIcon, ProfileIcon } from "./Icons";
import ProviderDialog from "./ProviderDialog";

interface SettingsModalProps {
  onClose: () => void;
}

const PROVIDERS: { value: ModelProvider; label: string; color: string; icon: string }[] = [
  { value: "anthropic", label: "Anthropic (官方)", color: "#F97316", icon: "🧠" },
  { value: "openai", label: "OpenAI (官方)", color: "#10A37F", icon: "⚡" },
  { value: "gemini", label: "Google Gemini (官方)", color: "#4285F4", icon: "🌐" },
  { value: "deepseek", label: "DeepSeek", color: "#6366F1", icon: "🔍" },
  { value: "minimax", label: "MiniMax", color: "#F64551", icon: "💎" },
  { value: "kimi", label: "Kimi (月之暗面)", color: "#6366F1", icon: "🌙" },
  { value: "zhipu", label: "智谱 GLM", color: "#0F62FE", icon: "🏯" },
  { value: "stepfun", label: "阶跃星辰 StepFun", color: "#16D6D2", icon: "⚡" },
  { value: "bailian", label: "阿里百炼", color: "#624AFF", icon: "☁️" },
  { value: "longcat", label: "LongCat", color: "#29E154", icon: "🐱" },
  { value: "xiaomimimo", label: "小米 MiMo", color: "#FF6900", icon: "📱" },
  { value: "baidu", label: "百度千帆", color: "#2932E1", icon: "🌊" },
  { value: "openrouter", label: "OpenRouter", color: "#6566F1", icon: "🔀" },
  { value: "siliconflow", label: "硅基流动", color: "#6E29F6", icon: "💜" },
  { value: "custom", label: "自定义", color: "#6B7280", icon: "🔧" },
];

const emptyModel = (): Partial<ModelInfo> => ({
  id: "",
  name: "",
  provider: "custom" as ModelProvider,
  family: "custom",
  contextLength: 128000,
  capabilities: [],
});

export default function SettingsModal({ onClose }: SettingsModalProps) {
  const { settings, updateSettings, apiKeys, setApiKeys } = useAppStore();
  const [editKeys, setEditKeys] = useState<Record<string, string>>({});
  const [editUrls, setEditUrls] = useState<Record<string, string>>({});
  const [localSettings, setLocalSettings] = useState(settings);
  const [customModels, setCustomModels] = useState<ModelInfo[]>([]);
  const [newModel, setNewModel] = useState<Partial<ModelInfo>>(emptyModel());
  const [capInput, setCapInput] = useState("");
  const [profiles, setProfiles] = useState<ApiProfile[]>([]);
  const [activeProfileId, setActiveProfileId] = useState("");
  const [newProfileName, setNewProfileName] = useState("");
  const [showProviderDialog, setShowProviderDialog] = useState(false);
  const [editingProvider, setEditingProvider] = useState<ModelProvider | undefined>(undefined);
  const [editingKey, setEditingKey] = useState("");
  const [editingUrl, setEditingUrl] = useState("");
  const [claudeSettingsJson, setClaudeSettingsJson] = useState("");
  const [showJsonPreview, setShowJsonPreview] = useState(false);

  useEffect(() => {
    loadProfiles();
    loadCustomModels();
    loadClaudeSettings();
  }, []);

  function openAddProvider() {
    setEditingProvider(undefined);
    setEditingKey("");
    setEditingUrl("");
    setShowProviderDialog(true);
  }

  function openEditProvider(provider: ModelProvider) {
    setEditingProvider(provider);
    setEditingKey(editKeys[provider] || "");
    setEditingUrl(editUrls[provider] || "");
    setShowProviderDialog(true);
  }

  async function handleSaveProvider(provider: ModelProvider, apiKey: string, baseUrl: string) {
    await api.setApiKey(provider, apiKey, baseUrl || undefined);
    // 从供应商 API 获取真实可用模型列表
    try {
      const models = await api.fetchProviderModels(provider, apiKey, baseUrl || undefined);
      console.log(`[Settings] Fetched ${models.length} models from ${provider}:`, models.map(m => m.id));
    } catch (err) {
      console.warn(`[Settings] Failed to fetch models from ${provider}:`, err);
    }
    await loadApiKeys();
    setShowProviderDialog(false);
  }

  async function loadProfiles() {
    try {
      const list = await api.listProfiles();
      setProfiles(list);
      const activeId = await api.getActiveProfileId();
      setActiveProfileId(activeId);
      await loadApiKeys();
    } catch (err) {
      console.error("Failed to load profiles:", err);
    }
  }

  async function loadApiKeys() {
    try {
      const keys = await api.getApiKeys();
      setApiKeys(keys);
      const keyMap: Record<string, string> = {};
      const urlMap: Record<string, string> = {};
      keys.forEach((k) => {
        keyMap[k.provider] = "••••••••" + k.apiKey.slice(-4);
        urlMap[k.provider] = k.baseUrl || "";
      });
      setEditKeys(keyMap);
      setEditUrls(urlMap);

      // 刷新所有已配置供应商的实际模型列表
      if (keys.length > 0) {
        try {
          const results = await api.refreshAllFetchedModels();
          console.log("[Settings] Refreshed models:", results);
        } catch (err) {
          console.warn("[Settings] Failed to refresh models:", err);
        }
      }
    } catch (err) {
      console.error("Failed to load API keys:", err);
    }
  }

  async function loadCustomModels() {
    try {
      const models = await api.listCustomModels();
      setCustomModels(models);
    } catch (err) {
      console.error("Failed to load custom models:", err);
    }
  }

  async function handleCreateProfile() {
    const name = newProfileName.trim();
    if (!name) {
      alert("请输入模板名称");
      return;
    }
    try {
      await api.createProfile(name);
      setNewProfileName("");
      await loadProfiles();
    } catch (err) {
      alert(`创建失败: ${err}`);
    }
  }

  async function handleDeleteProfile(id: string) {
    try {
      await api.deleteProfile(id);
      await loadProfiles();
    } catch (err) {
      alert(`删除失败: ${err}`);
    }
  }

  async function handleSwitchProfile(id: string) {
    try {
      await api.switchProfile(id);
      setActiveProfileId(id);
      await loadApiKeys();
    } catch (err) {
      alert(`切换失败: ${err}`);
    }
  }

  async function handleDeleteKey(provider: ModelProvider) {
    if (!confirm(`确认删除 ${provider} 的 API Key？`)) return;
    try {
      await api.deleteApiKey(provider);
      await loadApiKeys();
    } catch (err) {
      alert(`删除失败: ${err}`);
    }
  }

  async function handleAddModel() {
    if (!newModel.id || !newModel.name) {
      alert("请填写模型 ID 和显示名称");
      return;
    }
    try {
      const model: ModelInfo = {
        id: newModel.id!,
        name: newModel.name!,
        provider: newModel.provider || "custom",
        family: newModel.family || "custom",
        contextLength: newModel.contextLength || 128000,
        capabilities: capInput
          ? capInput.split(",").map((s) => s.trim()).filter(Boolean)
          : [],
      };
      await api.addCustomModel(model);
      setNewModel(emptyModel());
      setCapInput("");
      await loadCustomModels();
    } catch (err) {
      alert(`添加失败: ${err}`);
    }
  }

  async function handleRemoveModel(modelId: string) {
    if (!confirm(`确认删除自定义模型 "${modelId}"？`)) return;
    try {
      await api.removeCustomModel(modelId);
      await loadCustomModels();
    } catch (err) {
      alert(`删除失败: ${err}`);
    }
  }

  async function handleBrowseFile() {
    try {
      const path = await open({
        title: "选择 Claude.md 文件",
        filters: [{ name: "Markdown", extensions: ["md"] }],
      });
      if (path) {
        setLocalSettings({ ...localSettings, claudeMdPath: path as string });
      }
    } catch (err) {
      console.error(err);
    }
  }

  async function loadClaudeSettings() {
    try {
      const json = await api.getClaudeSettingsJson();
      setClaudeSettingsJson(json);
    } catch (err) {
      console.error("Failed to load Claude settings:", err);
      setClaudeSettingsJson("// 加载失败");
    }
  }

  async function handleSaveSettings() {
    try {
      await api.updateSettings(localSettings);
      updateSettings(localSettings);
      onClose();
    } catch (err) {
      alert(`保存设置失败: ${err}`);
    }
  }

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <div className="modal-title">
            <GearIcon size={18} />
            设置
          </div>
          <button className="btn-icon" onClick={onClose}><CloseIcon size={18} /></button>
        </div>
        <div className="modal-body">
          <div className="settings-section">
            <div className="settings-section-title">
              <KeyIcon size={15} />
              API Key 配置
            </div>

            {/* Profile Selector */}
            <div className="profile-bar">
              <div className="profile-select">
                <ProfileIcon size={14} />
                <select
                  className="form-input profile-select-input"
                  value={activeProfileId}
                  onChange={(e) => handleSwitchProfile(e.target.value)}
                >
                  {profiles.map((p) => (
                    <option key={p.id} value={p.id}>
                      {p.name} {p.isDefault ? "(默认)" : ""}
                    </option>
                  ))}
                </select>
              </div>
              {profiles.length > 1 && (
                <button className="btn-icon" onClick={() => { if (activeProfileId) handleDeleteProfile(activeProfileId); }} title="删除模板">
                  <TrashIcon size={14} />
                </button>
              )}
            </div>
            <div className="profile-create">
              <input
                type="text"
                className="form-input"
                placeholder="新模板名称（如：工作、个人）"
                value={newProfileName}
                onChange={(e) => setNewProfileName(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && handleCreateProfile()}
              />
              <button className="btn btn-secondary" onClick={handleCreateProfile}>
                <PlusIcon size={14} />
                新建模板
              </button>
            </div>

            {/* Configured Provider List */}
            <div className="configured-providers">
              <div className="section-subtitle">
                已配置的供应商
                <span className="count-badge">{apiKeys.length}</span>
              </div>
              {apiKeys.length === 0 ? (
                <div className="empty-providers">
                  <KeyIcon size={24} />
                  <p>尚未配置任何供应商</p>
                  <p className="hint">点击下方按钮添加 API Key</p>
                </div>
              ) : (
                <div className="provider-list">
                  {apiKeys.map((k) => {
                    const p = PROVIDERS.find((pp) => pp.value === k.provider);
                    const color = p?.color || "#6B7280";
                    const icon = p?.icon || "🔧";
                    const label = p?.label || k.provider;
                    return (
                      <div key={k.provider} className="provider-list-item" style={{ borderLeft: `3px solid ${color}` }}>
                        <span className="provider-list-icon" style={{ background: `${color}15`, color }}>{icon}</span>
                        <div className="provider-list-info">
                          <span className="provider-list-name">{label}</span>
                          <span className="provider-list-key">••••••••{k.apiKey.slice(-4)}</span>
                        </div>
                        <span className="provider-list-status configured">
                          <CheckIcon size={11} /> 已配置
                        </span>
                        <div className="provider-list-actions">
                          <button className="btn btn-secondary btn-sm" onClick={() => openEditProvider(k.provider)}>
                            编辑
                          </button>
                          <button className="btn btn-danger btn-sm" onClick={() => handleDeleteKey(k.provider)}>
                            删除
                          </button>
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
              <button className="btn btn-primary add-provider-btn" onClick={openAddProvider}>
                <PlusIcon size={16} />
                添加供应商
              </button>
            </div>
          </div>

          <div className="settings-section">
            <div className="settings-section-title">
              <BotIcon size={15} />
              自定义模型
            </div>
            {customModels.length > 0 && (
              <div className="custom-model-list">
                {customModels.map((m) => (
                  <div key={m.id} className="custom-model-item">
                    <div className="custom-model-info">
                      <span className="custom-model-id">{m.id}</span>
                      <span className="custom-model-name">{m.name}</span>
                      <span className="custom-model-provider">{m.provider}</span>
                    </div>
                    <button className="btn-icon" onClick={() => handleRemoveModel(m.id)} title="删除模型">
                      <TrashIcon size={15} />
                    </button>
                  </div>
                ))}
              </div>
            )}
            <div className="form-group">
              <label className="form-label">模型 ID</label>
              <input
                type="text"
                className="form-input"
                placeholder="如 my-custom-model"
                value={newModel.id || ""}
                onChange={(e) => setNewModel({ ...newModel, id: e.target.value })}
              />
            </div>
            <div className="form-group">
              <label className="form-label">显示名称</label>
              <input
                type="text"
                className="form-input"
                placeholder="如 My Custom Model"
                value={newModel.name || ""}
                onChange={(e) => setNewModel({ ...newModel, name: e.target.value })}
              />
            </div>
            <div className="form-group">
              <label className="form-label">提供商</label>
              <select
                className="form-input"
                value={newModel.provider || "custom"}
                onChange={(e) => setNewModel({ ...newModel, provider: e.target.value as ModelProvider })}
              >
                {PROVIDERS.map((p) => (
                  <option key={p.value} value={p.value}>{p.label}</option>
                ))}
              </select>
            </div>
            <div className="form-group">
              <label className="form-label">上下文长度</label>
              <input
                type="number"
                className="form-input"
                value={newModel.contextLength || 128000}
                onChange={(e) => setNewModel({ ...newModel, contextLength: parseInt(e.target.value) || 0 })}
              />
            </div>
            <div className="form-group">
              <label className="form-label">能力标签（逗号分隔）</label>
              <input
                type="text"
                className="form-input"
                placeholder="如 推理, 代码生成, 中文优化"
                value={capInput}
                onChange={(e) => setCapInput(e.target.value)}
              />
            </div>
            <button className="btn btn-primary" onClick={handleAddModel}>
              <PlusIcon size={14} />
              添加模型
            </button>
          </div>

          <div className="settings-section">
            <div className="settings-section-title">
              <SlidersIcon size={15} />
              路由设置
            </div>
            <div className="checkbox-group">
              <input
                type="checkbox"
                id="auto-write"
                checked={localSettings.autoWriteClaudeMd}
                onChange={(e) => setLocalSettings({ ...localSettings, autoWriteClaudeMd: e.target.checked })}
              />
              <label htmlFor="auto-write">自动写入 Claude.md（无需确认）</label>
            </div>
            <div className="checkbox-group">
              <input
                type="checkbox"
                id="confirm-switch"
                checked={localSettings.confirmBeforeSwitch}
                onChange={(e) => setLocalSettings({ ...localSettings, confirmBeforeSwitch: e.target.checked })}
              />
              <label htmlFor="confirm-switch">切换模型前需要确认</label>
            </div>
            <div className="checkbox-group">
              <input
                type="checkbox"
                id="learning-mode"
                checked={localSettings.learningMode}
                onChange={(e) => setLocalSettings({ ...localSettings, learningMode: e.target.checked })}
              />
              <label htmlFor="learning-mode">学习模式（记录选择优化推荐）</label>
            </div>
          </div>

          <div className="settings-section">
            <div className="settings-section-title">
              <FolderIcon size={15} />
              Claude.md 路径
            </div>
            <div className="form-row">
              <input
                type="text"
                className="form-input"
                value={localSettings.claudeMdPath}
                onChange={(e) => setLocalSettings({ ...localSettings, claudeMdPath: e.target.value })}
                placeholder="C:\Users\xxx\.claude\CLAUDE.md"
              />
              <button className="btn btn-secondary" onClick={handleBrowseFile}>
                浏览
              </button>
            </div>
          </div>

          <div className="settings-section">
            <div className="settings-section-title" onClick={() => { setShowJsonPreview(!showJsonPreview); loadClaudeSettings(); }} style={{ cursor: "pointer" }}>
              <span style={{ transform: showJsonPreview ? "rotate(90deg)" : "none", display: "inline-block", transition: "transform 0.15s", marginRight: 6, fontSize: 10 }}>▶</span>
              Claude Code settings.json 预览
              <button
                className="btn btn-secondary btn-sm"
                style={{ marginLeft: "auto" }}
                onClick={(e) => { e.stopPropagation(); loadClaudeSettings(); }}
              >
                刷新
              </button>
            </div>
            {showJsonPreview && (
              <div className="json-preview">
                <pre className="json-preview-content">{claudeSettingsJson || "加载中..."}</pre>
              </div>
            )}
          </div>
        </div>
        <div className="modal-footer">
          <button className="btn btn-secondary" onClick={onClose}>取消</button>
          <button className="btn btn-primary" onClick={handleSaveSettings}>保存</button>
        </div>
      </div>

      {showProviderDialog && (
        <ProviderDialog
          provider={editingProvider}
          existingKey={editingKey}
          existingUrl={editingUrl}
          onSave={handleSaveProvider}
          onClose={() => setShowProviderDialog(false)}
        />
      )}
    </div>
  );
}
