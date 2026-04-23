<div align="center">
  <img src="./src-tauri/icons/128x128@2x.png" width="100" height="100" alt="Storyboard Copilot" style="margin-bottom: -50px;">
  <h1 style="color: ##111227;">分镜助手</h1>
  <h3>基于节点画布的 AI 分镜工作台，一站式完成图片生成、编辑与分镜流程</h3>
</div>

<div align="center">
  <img src="./docs/imgs/readme/storyboard-copilot-homepage.webp" alt="Storyboard Copilot 首页截图" width="820" />
</div>

## ✨ 本 fork 特性：OpenAI 兼容格式开箱即用

> 相对上游版本，此 fork 新增了对 **OpenAI 兼容图像 API** 的原生支持，尤其对 [Uni API](https://uniapi.io) 这类聚合网关做了默认适配，是**更易用的版本**——不用自建代理、不用改源码，填一个 Key 就能跑。

- **零配置对接 Uni API**：内置 `https://api.uniapi.io` 作为默认 Base URL，设置页 → 供应商 → 「OpenAI 兼容」 里填入 API Key 即可开始生成。
- **Gemini 图像全家桶**：在「OpenAI 兼容」下直接选 Nano Banana / Nano Banana 2 / Nano Banana Pro（对应 Uni API 的 `google/nano-banana`、`google/nano-banana-2`、`google/nano-banana-pro`）。
- **文生图 + 图生图一键切换**：无参考图走 `POST /v1/images/generations`（JSON），连上参考图自动切到 `POST /v1/images/edits`（multipart，最多 16 张）。
- **结果自动落盘**：优先取 `b64_json` 拼 data URL 直接渲染，缺省时回落到 `url` 字段，不依赖额外的文件存储。

只要你在用的是 OpenAI `/v1/images/*` 协议的网关（Uni API、自建中转、其他兼容层），都能直接复用这一路。

## 下载

<div align="center">
Windows 用户请下载 <strong>.exe</strong> 文件，macOS 用户请下载 <strong>.dmg</strong> 文件

Windows 用户如果在启动时遇到了报错，请尝试安装 [WebView2 运行时](https://developer.microsoft.com/zh-cn/Microsoft-edge/webview2#download)

### Github 下载
[![Download Latest Release](https://img.shields.io/github/v/release/misakaikato/Storyboard-Copilot?style=for-the-badge&color=blue)](https://github.com/misakaikato/Storyboard-Copilot/releases/latest)

</div>

## 赞助

<div align="center">
  <div style="text-align: center; font-weight: 700; margin-bottom: 10px; font-size: 20px;">
    <a href="https://platform.minimaxi.com/subscribe/token-plan?code=H58G1T6WnZ&source=link" target="_blank" style="color: #f0440bff; text-decoration: none;">
      🚀 MiniMax Token Plan 惊喜上线
    </a>
  </div>
  <p>
    新增语音、音乐、视频和图片生成权益。邀请好友享双重好礼，助力开发体验！<br />
    好友立享 <strong>9 折专属优惠</strong> + Builder 权益，你赢返利 + 社区特权！
  </p>
  <a href="https://platform.minimaxi.com/subscribe/token-plan?code=H58G1T6WnZ&source=link" target="_blank">
    <img src="./docs/imgs/readme/token_plan.webp" alt="MiniMax Token Plan" width="720">
  </a>
</div>

<div align="center" style="margin-top: 28px;">
  <div style="text-align: center; font-weight: 700; margin-bottom: 10px; font-size: 20px;">
    <a href="https://www.bigmodel.cn/glm-coding?ic=JXQACHMWPT" target="_blank" style="color: #f0440bff; text-decoration: none;">
      🙋 蹲队友拼智谱 Coding Plan
    </a>
  </div>
  <p>
    🧩 国内顶流编程大模型，20+ 主流工具全适配，性价比拉满。<br />
    👉 立即参与「拼好模」：<a href="https://www.bigmodel.cn/glm-coding?ic=JXQACHMWPT" target="_blank">https://www.bigmodel.cn/glm-coding</a>
  </p>
  <a href="https://www.bigmodel.cn/glm-coding?ic=JXQACHMWPT" target="_blank">
    <img src="./docs/imgs/readme/glm_coding_plan.png" alt="智谱 GLM Coding Plan" width="420">
  </a>
</div>


## 技术栈

- 前端：React 18 + TypeScript + Zustand + `@xyflow/react` + TailwindCSS
- 桌面容器：Tauri 2
- 后端：Rust 命令接口
- 数据存储：SQLite（`rusqlite`，WAL）
- i18n：`react-i18next` + `i18next`

## 环境要求

- Node.js 20+
- npm 10+
- Rust stable（含 Cargo）
- Tauri 平台依赖（Windows/macOS）

安装与平台准备可参考：
- [基础工具安装配置（Windows / macOS）](./docs/development-guides/base-tools-installation.md)

## 快速开始

```bash
npm install
```

仅前端开发：

```bash
npm run dev
```

Tauri 联调（推荐）：

```bash
npm run tauri dev
```

## 常用命令

```bash
# TypeScript 类型检查
npx tsc --noEmit

# Rust 快速检查
cd src-tauri && cargo check

# 前端构建检查
npm run build

# Tauri 构建桌面应用
npm run tauri build
```

## 一键发布（自动构建 + Release）

本项目支持一条命令完成版本联动、触发 GitHub Actions 构建并发布 Release。

```bash
# patch 递增（例如 0.1.0 -> 0.1.1），并写入本次更新说明
npm run release -- patch "修复导出节点在大图下崩溃；优化启动速度"

# 或指定版本号
npm run release -- 0.2.0 "新增分镜批量裁剪工具"
```

命令会自动执行：
- 同步版本号到 `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`
- 提交版本变更并创建带说明的 tag（如 `v0.2.0`）
- 推送分支和 tag，触发 `.github/workflows/build.yml`
- 由 Action 构建 Windows/macOS 安装包并发布到 GitHub Releases（说明显示为 tag 注释）

## 项目结构（核心）

```text
src/
  features/canvas/          # 画布主流程（节点、工具、模型、UI）
  stores/                   # 全局状态与自动持久化策略
  commands/                 # 前端到 Tauri 命令桥接
  i18n/                     # 国际化入口与语言包
src-tauri/src/
  commands/                 # Rust 侧命令实现（含 project_state）
  lib.rs                    # Tauri 命令注册入口
docs/development-guides/    # 开发与扩展文档
```

## 架构要点

- 分层数据流：`UI -> Store -> Application Service -> Command/API -> Persistence`
- 节点注册单一真相源：`src/features/canvas/domain/nodeRegistry.ts`
- 工具体系分层：`tools/types.ts`、`tools/builtInTools.ts`、`ui/tool-editors/*`、`application/toolProcessor.ts`
- 持久化双通道：
  - 项目快照：`upsert_project_record`
  - 视口快照：`update_project_viewport_record`

## 扩展开发

### 新增模型

1. 在 `src/features/canvas/models/image/<provider>/` 新增模型文件
2. 声明 `displayName`、`providerId`、分辨率/比例、默认参数
3. 实现请求映射函数 `resolveRequest`

### 新增工具

1. 在 `src/features/canvas/tools/types.ts` 声明能力
2. 在 `src/features/canvas/tools/builtInTools.ts` 注册
3. 在 `src/features/canvas/ui/tool-editors/` 新增编辑器
4. 在 `src/features/canvas/application/toolProcessor.ts` 接入执行

### 新增节点

1. 在 `src/features/canvas/domain/canvasNodes.ts` 增加类型与数据结构
2. 在 `src/features/canvas/domain/nodeRegistry.ts` 注册默认数据与连线能力
3. 在 `src/features/canvas/nodes/index.ts` 注册渲染组件

详细指南：
- [项目开发环境与注意事项](./docs/development-guides/project-development-setup.md)
- [供应商与模型扩展指南](./docs/development-guides/provider-and-model-extension.md)

## 持久化与数据说明

- 自动持久化由 `projectStore` 驱动，不需要手动保存
- SQLite 文件位于 Tauri `app_data_dir/projects.db`
- `projects` 表核心字段：`nodes_json`、`edges_json`、`viewport_json`、`history_json`、`node_count`
- 图片字段通过 `imagePool + __img_ref__` 去重编码

## i18n 约定

- 入口：`src/i18n/index.ts`
- 语言包：`src/i18n/locales/zh.json`、`src/i18n/locales/en.json`
- 代码中使用 `useTranslation()` + `t('key.path')`，避免硬编码文案

## 开发文档导航

- [项目开发环境与注意事项](./docs/development-guides/project-development-setup.md)
- [供应商与模型扩展指南](./docs/development-guides/provider-and-model-extension.md)
- [基础工具安装配置（Windows / macOS）](./docs/development-guides/base-tools-installation.md)
