# ModelRouter - Claude Code 智能模型路由工具

一款跨平台桌面应用，自动识别 Claude.md 中的角色定义，根据任务类型智能匹配最合适的 AI 模型，实现多模型调度与自动切换。

## 功能特性

- **角色解析** - 读取 Claude.md，自动提取角色定义（名称、花名、技能）
- **智能路由** - 根据角色 + 任务类型匹配合适的模型（支持 8+ 种任务类型）
- **配置写入** - 规范格式将模型配置写入 Claude.md（含备份机制）
- **多提供商支持** - Anthropic / OpenAI / Gemini / DeepSeek / 通义千问 / 自定义
- **学习进化** - 记录用户选择，逐步优化推荐（Phase 2+）
- **跨平台** - Windows / macOS / Linux

## 技术栈

| 组件 | 选型 |
|------|------|
| 桌面框架 | Tauri 2.x |
| 前端 | React + TypeScript + Vite + Zustand |
| 后端 | Rust |
| 配置存储 | JSON（tauri-plugin-store）|
| 密钥存储 | 系统 Keychain（计划）|

## 项目结构

```
ModelRouter/
├── src/                    # React 前端
│   ├── components/         # UI 组件
│   │   ├── RoleList.tsx         # 角色列表
│   │   ├── ModelRecommendationPanel.tsx  # 模型推荐
│   │   ├── ClaudeMdPreview.tsx  # Claude.md 预览
│   │   └── SettingsModal.tsx    # 设置面板
│   ├── services/api.ts    # Tauri invoke 封装
│   ├── store/index.ts     # Zustand 状态管理
│   ├── types/index.ts     # TypeScript 类型
│   └── styles/global.css  # 全局样式（暗色主题）
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── commands/      # Tauri 命令
│   │   │   ├── claude_md.rs  # Claude.md 读写
│   │   │   ├── model.rs     # 模型路由
│   │   │   └── config.rs    # 配置管理
│   │   ├── services/
│   │   │   ├── parser.rs   # 角色解析引擎
│   │   │   └── router.rs   # 路由推荐引擎
│   │   └── models/mod.rs   # 数据结构
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── README.md
```

## 开发环境要求

### Windows

1. **Node.js** >= 18（已安装 v26.1.0）
2. **Rust** >= 1.70（通过 rustup 安装）:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   ```
3. **C++ 编译工具**（二选一）:
   - **选项 A**: Visual Studio 2022 Build Tools（推荐）:
     ```bash
     winget install Microsoft.VisualStudio.2022.BuildTools
     ```
     安装后运行 Visual Studio Installer，添加 "MSVC v143 - VS 2022 C++ x64/x86 build tools" 和 "Windows 11 SDK"
   - **选项 B**: MinGW-w64:
     ```bash
     scoop install mingw
     ```
     然后设置 Rust 工具链: `rustup default stable-x86_64-pc-windows-gnu`

### macOS

```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 安装 Node.js
brew install node
```

### Linux

```bash
# Ubuntu/Debian
sudo apt install build-essential pkg-config libwebkit2gtk-4.1-dev \
  libappindicator3-dev librsvg2-dev patchelf curl wget file

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

## 开发命令

```bash
# 安装依赖
npm install

# 仅前端开发（不依赖 Rust 后端）
npm run dev

# 完整 Tauri 开发（需要 Rust 环境）
npm run tauri:dev

# 构建生产版本
npm run tauri:build
```

## 模型路由规则

| 角色 | 推荐模型 | 备选模型 |
|------|---------|---------|
| 项目经理 | claude-opus-4-7 | gpt-4o |
| 前端工程师 | claude-sonnet-4-7 | gpt-4o-mini |
| 后端工程师 | claude-opus-4-7 | claude-sonnet-4-7 |
| 安全工程师 | claude-opus-4-7 | gpt-4o |
| 运维工程师 | claude-sonnet-4-7 | gpt-4o-mini |

| 任务类型 | 推荐模型 |
|---------|---------|
| 代码审查 | claude-opus-4-7 |
| 代码生成 | claude-sonnet-4-7 |
| 架构设计 | claude-opus-4-7 |
| 文档撰写 | gpt-4o |
| 数据分析 | gpt-4o |

## Claude.md 配置格式

ModelRouter 写入的配置块格式：

```markdown
---
name: model-config
description: ModelRoute Auto Config
---

## 模型配置

| 任务类型 | 推荐模型 | 提供商 | 切换时机 | 更新时间 |
|---------|---------|--------|---------|----------|
| code-review | claude-opus-4-7 | anthropic | 识别到 [代码审查] 任务时 | 2026-05-25T... |
```

## License

MIT
