import { useState, useEffect } from "react";
import { useAppStore } from "./store";
import RoleList from "./components/RoleList";
import ModelRecommendationPanel from "./components/ModelRecommendationPanel";
import RoleDetailPanel from "./components/RoleDetailPanel";
import SettingsModal from "./components/SettingsModal";
import NewRoleModal from "./components/NewRoleModal";
import * as api from "./services/api";
import { RouteIcon, RefreshIcon, GearIcon } from "./components/Icons";

function App() {
  const [showSettings, setShowSettings] = useState(false);
  const [showNewRole, setShowNewRole] = useState(false);
  const { roles, setRoles, settings, updateSettings } = useAppStore();

  useEffect(() => {
    loadInitialData();
  }, []);

  async function loadInitialData() {
    try {
      const defaultPath = await api.getDefaultClaudeMdPath();
      const appSettings = await api.getSettings();
      updateSettings({ ...appSettings, claudeMdPath: appSettings.claudeMdPath || defaultPath });

      if (defaultPath) {
        const parsedRoles = await api.parseClaudeMd(defaultPath);
        setRoles(parsedRoles);
      }
    } catch (err) {
      console.error("Failed to load initial data:", err);
    }
  }

  async function refreshRoles() {
    if (!settings.claudeMdPath) return;
    try {
      const parsedRoles = await api.parseClaudeMd(settings.claudeMdPath);
      setRoles(parsedRoles);
    } catch (err) {
      console.error("Failed to refresh roles:", err);
    }
  }

  return (
    <div className="app">
      <header className="app-header">
        <div className="app-title">
          <span className="app-logo">
            <RouteIcon size={24} />
          </span>
          <h1>ModelRouter</h1>
          <span className="app-subtitle">Claude Code 智能模型路由</span>
        </div>
        <div className="app-actions">
          <button className="btn-icon" onClick={refreshRoles} title="刷新角色">
            <RefreshIcon size={18} />
          </button>
          <button className="btn-icon" onClick={() => setShowSettings(true)} title="设置">
            <GearIcon size={18} />
          </button>
        </div>
      </header>

      <main className="app-main">
        <aside className="app-sidebar">
          <RoleList roles={roles} onNewRole={() => setShowNewRole(true)} />
        </aside>

        <section className="app-content">
          <ModelRecommendationPanel />
          <RoleDetailPanel />
        </section>
      </main>

      {showSettings && <SettingsModal onClose={() => setShowSettings(false)} />}
      {showNewRole && (
        <NewRoleModal
          onClose={() => setShowNewRole(false)}
          onCreated={refreshRoles}
        />
      )}
    </div>
  );
}

export default App;
