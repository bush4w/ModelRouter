import { create } from "zustand";
import type {
  RoleInfo,
  ApiKeyConfig,
  ModelInfo,
  AppSettings,
  ModelRecommendation,
  TaskType,
} from "../types";

interface AppState {
  // 角色相关
  roles: RoleInfo[];
  selectedRole: RoleInfo | null;
  setRoles: (roles: RoleInfo[]) => void;
  selectRole: (role: RoleInfo | null) => void;

  // 任务相关
  currentTaskType: TaskType;
  setTaskType: (taskType: TaskType) => void;

  // 推荐相关
  currentRecommendation: ModelRecommendation | null;
  setRecommendation: (rec: ModelRecommendation | null) => void;

  // 模型列表
  availableModels: ModelInfo[];
  setAvailableModels: (models: ModelInfo[]) => void;

  // API Key 配置
  apiKeys: ApiKeyConfig[];
  setApiKeys: (keys: ApiKeyConfig[]) => void;

  // 应用设置
  settings: AppSettings;
  updateSettings: (settings: Partial<AppSettings>) => void;
}

const defaultSettings: AppSettings = {
  autoWriteClaudeMd: false,
  confirmBeforeSwitch: true,
  learningMode: false,
  claudeMdPath: "",
  language: "zh-CN",
};

export const useAppStore = create<AppState>((set) => ({
  roles: [],
  selectedRole: null,
  setRoles: (roles) => set({ roles }),
  selectRole: (selectedRole) => set({ selectedRole }),

  currentTaskType: "general",
  setTaskType: (currentTaskType) => set({ currentTaskType }),

  currentRecommendation: null,
  setRecommendation: (currentRecommendation) => set({ currentRecommendation }),

  availableModels: [],
  setAvailableModels: (availableModels) => set({ availableModels }),

  apiKeys: [],
  setApiKeys: (apiKeys) => set({ apiKeys }),

  settings: defaultSettings,
  updateSettings: (newSettings) =>
    set((state) => ({ settings: { ...state.settings, ...newSettings } })),
}));
