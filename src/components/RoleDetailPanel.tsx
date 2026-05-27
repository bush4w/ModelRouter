import { useAppStore } from "../store";
import { PersonIcon, CheckIcon } from "./Icons";
import type { TaskType } from "../types";

const ROLE_TASK_MAP: Record<string, TaskType[]> = {
  "项目经理": ["general", "documentation"],
  "产品工程师": ["general", "documentation", "data-analysis"],
  "产品架构师": ["architecture", "general", "documentation"],
  "UI设计师": ["ui-design", "general"],
  "前端工程师": ["code-generation", "ui-design", "debugging", "performance"],
  "后端工程师": ["code-generation", "architecture", "debugging", "performance"],
  "数据库工程师": ["data-analysis", "architecture", "performance"],
  "外部系统集成工程师": ["code-generation", "architecture", "debugging"],
  "性能优化工程师": ["performance", "debugging"],
  "安全测试工程师": ["security-review", "code-review"],
  "运维工程师": ["debugging", "performance", "general"],
  "售前工程师": ["documentation", "general"],
  "商务经理": ["general", "documentation", "data-analysis"],
};

const TASK_LABELS: Record<TaskType, string> = {
  "general": "通用任务",
  "code-generation": "代码生成",
  "code-review": "代码审查",
  "debugging": "调试排错",
  "documentation": "文档撰写",
  "architecture": "架构设计",
  "data-analysis": "数据分析",
  "ui-design": "UI 设计",
  "security-review": "安全审查",
  "performance": "性能优化",
};

export default function RoleDetailPanel() {
  const { selectedRole } = useAppStore();

  if (!selectedRole) {
    return (
      <div className="role-detail-panel">
        <div className="panel-title">
          <PersonIcon size={16} />
          角色详情
        </div>
        <div className="empty-state">
          请在左侧选择一个角色<br />
          查看其职责、技能与模型配置
        </div>
      </div>
    );
  }

  const { name, alias, description, skills } = selectedRole;
  const tasks = ROLE_TASK_MAP[name] || ["general"];

  return (
    <div className="role-detail-panel">
      <div className="panel-title">
        <PersonIcon size={16} />
        角色详情
      </div>

      <div className="role-detail-card">
        {/* Header */}
        <div className="role-detail-header">
          <div className="role-detail-avatar">{name[0]}</div>
          <div className="role-detail-title">
            <div className="role-detail-name">
              {name}
              {alias && <span className="role-detail-alias">「{alias}」</span>}
            </div>
            <div className="role-detail-desc">{description || "暂无描述"}</div>
          </div>
        </div>

        {/* Skills */}
        <div className="role-detail-section">
          <div className="role-detail-section-title">技能标签</div>
          <div className="skills-tags">
            {skills.map((skill) => (
              <span key={skill} className="skill-tag">{skill}</span>
            ))}
          </div>
        </div>

        {/* Suitable Tasks */}
        <div className="role-detail-section">
          <div className="role-detail-section-title">擅长任务</div>
          <div className="tasks-list">
            {tasks.map((task) => (
              <div key={task} className="task-item">
                <CheckIcon size={14} />
                {TASK_LABELS[task] || task}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
