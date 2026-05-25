import { useEffect, useState } from "react";
import { useAppStore } from "../store";
import { readTextFile } from "@tauri-apps/plugin-fs";

export default function ClaudeMdPreview() {
  const { settings } = useAppStore();
  const [content, setContent] = useState<string>("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (settings.claudeMdPath) {
      loadContent();
    }
  }, [settings.claudeMdPath]);

  async function loadContent() {
    try {
      const text = await readTextFile(settings.claudeMdPath);
      // 仅显示模型配置部分（前 50 行 + "模型配置"块）
      const lines = text.split("\n");
      const configIdx = lines.findIndex(l => l.includes("model-config") || l.includes("## 模型配置"));
      if (configIdx >= 0) {
        setContent(lines.slice(configIdx, configIdx + 20).join("\n"));
      } else {
        setContent(lines.slice(0, 30).join("\n") + "\n\n[未找到模型配置块，确认后将自动写入]");
      }
      setError(null);
    } catch (err) {
      setError(String(err));
    }
  }

  return (
    <div className="claude-md-preview">
      <div className="panel-title">📝 Claude.md 预览</div>
      <div style={{ fontSize: 11, color: "var(--color-text-muted)", marginBottom: 12 }}>
        {settings.claudeMdPath || "未设置路径"}
      </div>
      <div className="preview-content">
        {error ? `读取失败: ${error}` : content || "[文件为空或未加载]"}
      </div>
    </div>
  );
}
