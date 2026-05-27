import { useState } from "react";
import { useAppStore } from "../store";
import * as api from "../services/api";
import { CloseIcon, PlusIcon, PersonIcon } from "./Icons";

interface NewRoleModalProps {
  onClose: () => void;
  onCreated: () => void;
}

const CATEGORIES = ["研发岗", "产品与设计", "管理与协调", "质量与运维", "商务与售前", "其他"];

export default function NewRoleModal({ onClose, onCreated }: NewRoleModalProps) {
  const { settings, roles } = useAppStore();
  const [name, setName] = useState("");
  const [alias, setAlias] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("研发岗");
  const [skillInput, setSkillInput] = useState("");
  const [skills, setSkills] = useState<string[]>([]);
  const [saving, setSaving] = useState(false);

  function addSkill() {
    const s = skillInput.trim();
    if (s && !skills.includes(s)) {
      setSkills([...skills, s]);
      setSkillInput("");
    }
  }

  function removeSkill(s: string) {
    setSkills(skills.filter((x) => x !== s));
  }

  async function handleCreate() {
    if (!name.trim() || !alias.trim() || !description.trim()) {
      alert("请填写角色名称、角色名字和角色定义");
      return;
    }
    if (!settings.claudeMdPath) {
      alert("请先在设置中配置目标文件路径（点击右上角 ⚙️ 图标 → 文件路径设置）");
      return;
    }
    if (roles.some((r) => r.name === name.trim())) {
      alert(`角色 "${name.trim()}" 已存在`);
      return;
    }

    setSaving(true);
    try {
      await api.writeRole(settings.claudeMdPath, name.trim(), alias.trim(), description.trim(), skills);
      onCreated();
      onClose();
    } catch (err) {
      const errMsg = String(err);
      alert(`创建角色失败：${errMsg}\n\n请检查：\n1. 目标文件路径是否正确\n2. 文件是否正在被其他程序占用\n3. 是否有写入权限`);
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <div className="modal-title">
            <PersonIcon size={18} />
            新建角色
          </div>
          <button className="btn-icon" onClick={onClose}><CloseIcon size={18} /></button>
        </div>
        <div className="modal-body">
          <div className="form-group">
            <label className="form-label">角色名称 *</label>
            <input
              type="text"
              className="form-input"
              placeholder="如 前端工程师"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label className="form-label">角色名字 *</label>
            <input
              type="text"
              className="form-input"
              placeholder="如 叶屏"
              value={alias}
              onChange={(e) => setAlias(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label className="form-label">角色定义 *</label>
            <textarea
              className="form-input form-textarea"
              placeholder="详细描述该角色的定位与职责，如&#10;负责用户界面开发与交互实现，确保跨浏览器兼容性与性能优化，对接 UI 设计稿并还原像素级实现..."
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={5}
              maxLength={300}
            />
            <div className="char-count">{description.length}/300</div>
          </div>
          <div className="form-group">
            <label className="form-label">所属分类</label>
            <select
              className="form-input"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
            >
              {CATEGORIES.map((c) => (
                <option key={c} value={c}>{c}</option>
              ))}
            </select>
          </div>
          <div className="form-group">
            <label className="form-label">技能标签</label>
            <div className="skill-input-row">
              <input
                type="text"
                className="form-input"
                placeholder="输入技能后回车添加"
                value={skillInput}
                onChange={(e) => setSkillInput(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    addSkill();
                  }
                }}
              />
              <button className="btn btn-secondary" onClick={addSkill} type="button">
                <PlusIcon size={14} />
              </button>
            </div>
            {skills.length > 0 && (
              <div className="role-card-skills" style={{ marginTop: 8 }}>
                {skills.map((s) => (
                  <span key={s} className="skill-tag" onClick={() => removeSkill(s)} style={{ cursor: "pointer" }}>
                    {s} ×
                  </span>
                ))}
              </div>
            )}
          </div>
        </div>
        <div className="modal-footer">
          <button className="btn btn-secondary" onClick={onClose}>取消</button>
          <button className="btn btn-primary" onClick={handleCreate} disabled={saving}>
            {saving ? "创建中..." : "创建角色"}
          </button>
        </div>
      </div>
    </div>
  );
}
