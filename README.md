# MomoTalk Plus (MTP)

![MTP Logo](./src-tauri/icon.png)

MomoTalk Plus (MTP) 是一款基于 Tauri 2 + Nuxt 3 + TypeScript 开发的 BA 学生扮演聊天软件。本项目使用现代化的技术栈，提供了流畅的桌面应用体验。

## 项目简介

MomoTalk Plus 是一款模拟游戏《蔚蓝档案》(Blue Archive) 中通讯软件的桌面应用，允许用户与游戏中的学生角色进行对话互动。项目集成了大语言模型 (LLM) 功能，使角色对话更加智能和自然。

## 功能特点

- **角色扮演聊天**：与游戏中的学生角色进行对话互动
- **多窗口支持**：主界面、设置、关于等多窗口管理
- **数据持久化**：使用 SQLite 数据库存储聊天记录和用户数据
- **现代化 UI**：基于 Shadcn UI 和 Tailwind CSS 的美观界面
- **跨平台支持**：支持 Windows、macOS 和 Linux 系统

## 计划中

[ ] 支持`open ai`、`qwen`、`qwq`等 api
[ ] 重构暗色主题
[ ] 支持 Android、ios、ipad os 等移动端系统

## 安装与使用

> [!Caution]
> 目前只支持 ds 官方 api

### 环境要求

- Node.js 18+ 和 Yarn
- Rust 工具链
- 支持的操作系统：Windows 10+、macOS 10.15+、Ubuntu 20.04+

### 安装步骤

从 `release` 内下载对应版本的安装包安装即可

## 技术架构

### 前端技术栈

- **框架**：Vue 3 + TypeScript
- **构建工具**：Nuxt 3
- **UI 库**：Shadcn-Vue
- **状态管理**：Pinia

### 后端技术栈

- **框架**：Tauri 2.0 (Rust)
- **数据库**：SQLite + Sea-ORM
- **LLM 集成**：LangChain、OpenAI API、DeepSeek
- **插件**：Tauri Store、Tauri Opener

## 项目结构

```
├── src/                  # 前端源代码
│   ├── public/           # 公共静态资源
│   ├── assets/           # 静态资源
│   ├── components/       # Vue 组件
│   ├── pages/            # 页面组件
│   ├── stores/           # Pinia 状态管理
│   └── services/         # API 服务
├── src-tauri/            # Rust 后端代码
│   ├── src/              # 主要源代码
│   ├── llm/              # LLM 集成模块
│   ├── entity/           # 数据库实体
│   └── migration/        # 数据库迁移
```

## 开发指南

### 开发步骤

1. 克隆仓库

```bash
git clone https://github.com/MTPGroup/MTP.git
cd MTP
```

2. 安装依赖

```bash
yarn install
```

3. 开发模式运行

```bash
yarn tauri dev
```

4. 构建应用

```bash
yarn tauri build
```

### 推荐的 IDE 设置

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### TypeScript 支持

由于 TypeScript 无法处理 `.vue` 导入的类型信息，它们默认被视为通用 Vue 组件类型。如果你希望在 `.vue` 导入中获取实际的 prop 类型，可以启用 Volar 的 Take Over 模式：

1. 从 VS Code 的命令面板运行 `Extensions: Show Built-in Extensions`，找到 `TypeScript and JavaScript Language Features`，然后右键选择 `Disable (Workspace)`。默认情况下，如果禁用了默认的 TypeScript 扩展，Take Over 模式将自动启用。
2. 通过运行命令面板中的 `Developer: Reload Window` 重新加载 VS Code 窗口。

你可以在[这里](https://github.com/johnsoncodehk/volar/discussions/471)了解更多关于 Take Over 模式的信息。

## 贡献指南

欢迎贡献代码、报告问题或提出新功能建议。请遵循以下步骤：

1. Fork 仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

[MIT License](LICENSE)

## 联系方式

- 作者：hanasaki
- 邮箱：hanasakayui2022@gmail.com
