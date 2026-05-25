import { useEffect, useState } from "react";
import { useAppStore } from "../store";
import * as api from "../services/api";
import { open } from "@tauri-apps/plugin-dialog";
import type { ModelProvider } from "../types";

interface SettingsModalProps {
  onClose: () => void;
}

const PROVIDERS: { value: ModelProvider; label: string; placeholder: string; defaultUrl: string }[] = [
  { value: "anthropic", label: "Anthropic (Claude)", placeholder: "sk-ant-...", defaultUrl: "https://api.anthropic.com" },
  { value: "openai", label: "OpenAI (GPT)", placeholder: "sk-...", defaultUrl: "https://api.openai.com/v1" },
  { value: "gemini", label: "Google Gemini", placeholder: "AIza...", defaultUrl: "https://generativelanguage.googleapis.com" },
  { value: "deepseek", label: "DeepSeek", placeholder: "sk-...", defaultUrl: "https://api.deepseek.com" },
  { value: "qwen", label: "通义千问 Qwen", placeholder: "sk-...", defaultUrl: "https://dashscope.aliyuncs.com/api/v1" },
  { value: "custom", label: "自定义 (OpenAI 兼容)", placeholder: "your-key", defaultUrl: "" },
];

export default function SettingsModal({ onClose }: SettingsModalProps) {
  const { settings, updateSettings, apiKeys, setApiKeys } = useAppStore();
  const [editKeys, setEditKeys] = useState<Record<string, string>>({});
  const [editUrls, setEditUrls] = useState<Record<string, string>>({});
  const [localSettings, setLocalSettings] = useState(settings);

  useEffect(() => {
    loadApiKeys();
  }, []);

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
    } catch (err) {
      console.error("Failed to load API keys:", err);
    }
  }

  async function handleSaveKey(provider: ModelProvider) {
    const key = editKeys[provider];
    const url = editUrls[provider];
    if (!key || key.startsWith("••")) {
      alert("请输入有效的 API Key");
      return;
    }
    try {
      await api.setApiKey(provider, key, url || undefined);
      await loadApiKeys();
      alert(`${provider} API Key 已保存`);
    } catch (err) {
      alert(`保存失败: ${err}`);
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

  async function handleSaveSettings() {
    try {
      await api.updateSettings(localSettings);
      updateSettings(localSettings);
      onClose();
    } catch (err) {
      alert(`保存设置失败: ${err}`);
    }
  }

  function isKeyConfigured(provider: ModelProvider): boolean {
    return apiKeys.some((k) => k.provider === provider);
  }

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <div className="modal-title">⚙️ 设置</div>
          <button className="btn-icon" onClick={onClose}>✕</button>
        </div>
        <div className="modal-body">
          <div className="settings-section">
            <div className="settings-section-title">🤖 API Key 配置</div>
            {PROVIDERS.map((p) => (
              <div key={p.value} className="form-group">
                <label className="form-label">
                  {p.label} {isKeyConfigured(p.value) && <span style={{ color: "var(--color-success)" }}>✓ 已配置</span>}
                </label>
                <div className="form-row">
                  <input
                    type="password"
                    className="form-input"
                    placeholder={p.placeholder}
                    value={editKeys[p.value] || ""}
                    onChange={(e) => setEditKeys({ ...editKeys, [p.value]: e.target.value })}
                  />
                  <button className="btn btn-primary" onClick={() => handleSaveKey(p.value)}>
                    保存
                  </button>
                  {isKeyConfigured(p.value) && (
                    <button className="btn btn-danger" onClick={() => handleDeleteKey(p.value)}>
                      删除
                    </button>
                  )}
                </div>
                {p.value === "custom" && (
                  <input
                    type="text"
                    className="form-input"
                    placeholder="自定义 Base URL"
                    style={{ marginTop: 6 }}
                    value={editUrls[p.value] || ""}
                    onChange={(e) => setEditUrls({ ...editUrls, [p.value]: e.target.value })}
                  />
                )}
              </div>
            ))}
          </div>

          <div className="settings-section">
            <div className="settings-section-title">🔧 路由设置</div>
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
            <div className="settings-section-title">📁 Claude.md 路径</div>
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
        </div>
        <div className="modal-footer">
          <button className="btn btn-secondary" onClick={onClose}>取消</button>
          <button className="btn btn-primary" onClick={handleSaveSettings}>保存</button>
        </div>
      </div>
    </div>
  );
}
