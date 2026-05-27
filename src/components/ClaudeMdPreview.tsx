import { useAppStore } from "../store";
import { DocIcon } from "./Icons";

function getRoleIcon(name: string): string {
  const map: Record<string, string> = {
    "项目经理": "📋",
    "产品工程师": "💡",
    "产品架构师": "🏗",
    "UI设计师": "🎨",
    "前端工程师": "🖥",
    "后端工程师": "⚙️",
    "数据库工程师": "🗄",
    "外部系统集成工程师": "🔌",
    "性能优化工程师": "⚡",
    "安全测试工程师": "🛡",
    "运维工程师": "🔧",
    "售前工程师": "📊",
    "商务经理": "💼",
  };
  return map[name] || "👤";
}

export default function ClaudeMdPreview() {
  const { settings, roles } = useAppStore();

  return (
    <div className="claude-md-preview">
      <div className="panel-title">
        <DocIcon size={16} />
        Claude.md 角色预览
      </div>
      <div className="preview-file-path">
        {settings.claudeMdPath || "未设置路径"}
      </div>

      {roles.length === 0 ? (
        <div className="empty-state">
          暂无角色定义<br />
          请在设置中配置 Claude.md 路径并刷新
        </div>
      ) : (
        <div className="role-cards-grid">
          {roles.map((role) => (
            <div key={role.name} className="role-card">
              <div className="role-card-header">
                <span className="role-card-icon">{getRoleIcon(role.name)}</span>
                <div className="role-card-title">
                  <span className="role-card-name">{role.name}</span>
                  {role.alias && <span className="role-card-alias">{role.alias}</span>}
                </div>
              </div>
              {role.description && (
                <div className="role-card-desc">{role.description}</div>
              )}
              <div className="role-card-skills">
                {role.skills.map((skill) => (
                  <span key={skill} className="skill-tag">{skill}</span>
                ))}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
