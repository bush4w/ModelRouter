# ModelRouter — Claude Code 智能模型路由

一款基于 Tauri 2.x 的跨平台桌面应用。自动解析 CLAUDE.md 中的角色定义，根据角色与任务类型智能推荐最合适的 AI 模型。

## 功能特性

- **角色解析** — 读取 CLAUDE.md，自动提取角色定义（名称、花名、描述、技能）
- **智能路由** — 根据角色 + 任务类型匹配推荐模型，覆盖 10 种任务类型
- **美化预览** — 角色详情以卡片形式展示，清晰直观
- **配置写入** — 一键将模型配置写入 CLAUDE.md（自动备份）
- **多提供商** — Anthropic / OpenAI / Gemini / DeepSeek / 通义千问 / 自定义
- **自定义模型** — 支持手动添加任意模型，扩展内置模型库
- **配置模板** — 多套 API Key 配置 Profile，一键切换（如"工作"/"个人"）
- **持久化存储** — 设置、密钥、自定义模型、Profile 全部持久化到磁盘
- **学习进化** — 记录用户选择，逐步优化推荐（Phase 2+）

## 技术栈

| 组件 | 选型 |
|------|------|
| 桌面框架 | Tauri 2.x |
| 前端 | React 18 + TypeScript + Vite + Zustand |
| 后端 | Rust |
| 存储 | tauri-plugin-store（JSON 持久化） |
| 样式 | 暖色调 Claude 风格 |

## 项目结构

```
ModelRouter/
├── src/                          # React 前端
│   ├── components/
│   │   ├── RoleList.tsx                # 角色侧边栏
│   │   ├── ModelRecommendationPanel.tsx # 模型推荐面板
│   │   ├── ClaudeMdPreview.tsx         # 角色卡片预览
│   │   ├── SettingsModal.tsx           # 设置面板（密钥/模型/Profile）
│   │   └── Icons.tsx                   # SVG 图标组件
│   ├── services/api.ts           # Tauri invoke 桥接层
│   ├── store/index.ts            # Zustand 全局状态
│   ├── types/index.ts            # TypeScript 类型定义
│   └── styles/global.css         # 全局样式
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── commands/
│   │   │   ├── claude_md.rs       # Claude.md 解析/写入
│   │   │   ├── model.rs           # 模型路由 & 自定义模型
│   │   │   └── config.rs          # 设置/API Key/Profile 管理
│   │   ├── services/
│   │   │   ├── parser.rs          # 角色解析引擎
│   │   │   └── router.rs          # 推荐引擎
│   │   ├── models/mod.rs          # 数据模型
│   │   └── lib.rs                 # Tauri 入口
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── README.md
```

## 开发环境

### Windows

1. **Node.js** >= 18
2. **Rust** >= 1.70:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   ```
3. **Visual Studio 2022 Build Tools**（含 MSVC v143 和 Windows 11 SDK）:
   ```bash
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

### macOS

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
brew install node
```

### Linux

```bash
sudo apt install build-essential pkg-config libwebkit2gtk-4.1-dev \
  libappindicator3-dev librsvg2-dev patchelf curl wget file
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

## 开发命令

```bash
npm install              # 安装依赖
npm run dev              # 仅前端开发（不依赖 Rust）
npm run tauri dev        # 完整 Tauri 开发（含 Rust 后端）
npm run tauri build      # 构建发布版本（含 NSIS 安装包）
npm run build            # 仅构建前端
cargo test               # 运行 Rust 测试（在 src-tauri 目录）
```

## 模型路由规则

### 按角色

| 角色 | 推荐模型 | 备选 |
|------|---------|------|
| 项目经理 | claude-opus-4-7 | gpt-4o |
| 产品工程师 | claude-sonnet-4-7 | gpt-4o |
| 产品架构师 | claude-opus-4-7 | claude-sonnet-4-7 |
| UI 设计师 | claude-sonnet-4-7 | gpt-4o |
| 前端工程师 | claude-sonnet-4-7 | gpt-4o-mini |
| 后端工程师 | claude-opus-4-7 | claude-sonnet-4-7 |
| 数据库工程师 | claude-opus-4-7 | gpt-4o |
| 集成工程师 | claude-sonnet-4-7 | gpt-4o |
| 安全工程师 | claude-opus-4-7 | gpt-4o |
| 运维工程师 | claude-sonnet-4-7 | gpt-4o-mini |

### 按任务类型

| 任务 | 推荐模型 | 说明 |
|------|---------|------|
| 代码生成 | claude-sonnet-4-7 | 平衡速度与质量 |
| 代码审查 | claude-opus-4-7 | 需要最强推理 |
| 调试排错 | claude-opus-4-7 | 复杂逻辑分析 |
| 文档撰写 | gpt-4o | 通用理解生成 |
| 架构设计 | claude-opus-4-7 | 深度技术决策 |
| 数据分析 | gpt-4o | 数据处理能力 |
| UI 设计 | claude-sonnet-4-7 | 前端设计建议 |
| 安全审查 | claude-opus-4-7 | 安全漏洞分析 |
| 性能优化 | claude-opus-4-7 | 系统级调优 |
| 通用任务 | claude-sonnet-4-7 | 日常开发 |

## 配置格式

ModelRouter 在 CLAUDE.md 中写入的配置块：

```markdown
---
name: model-config
description: ModelRoute Auto Config
---

## 模型配置

| 任务类型 | 推荐模型 | 提供商 | 切换时机 | 更新时间 |
|---------|---------|--------|---------|----------|
| general | claude-sonnet-4-7 | anthropic | 识别到 [通用任务] 时自动切换 | 2026-05-25T... |
```

## License

MIT
