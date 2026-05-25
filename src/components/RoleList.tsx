import { useAppStore } from "../store";
import type { RoleInfo } from "../types";

interface RoleListProps {
  roles: RoleInfo[];
}

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

export default function RoleList({ roles }: RoleListProps) {
  const { selectedRole, selectRole } = useAppStore();

  if (roles.length === 0) {
    return (
      <div className="role-list">
        <div className="role-list-title">📋 角色列表</div>
        <div className="empty-state">
          暂无角色定义<br />
          请在设置中配置 Claude.md 路径
        </div>
      </div>
    );
  }

  return (
    <div className="role-list">
      <div className="role-list-title">📋 角色列表 ({roles.length})</div>
      {roles.map((role) => (
        <div
          key={role.name}
          className={`role-item ${selectedRole?.name === role.name ? "active" : ""}`}
          onClick={() => selectRole(role)}
        >
          <div className="role-item-icon">{getRoleIcon(role.name)}</div>
          <div className="role-item-info">
            <div className="role-item-name">{role.name}</div>
            {role.alias && <div className="role-item-alias">{role.alias}</div>}
          </div>
        </div>
      ))}
    </div>
  );
}
