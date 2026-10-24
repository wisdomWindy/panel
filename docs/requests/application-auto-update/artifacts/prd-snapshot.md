# 应用自动更新与检查更新 - 来源快照

## Verbatim Source Evidence Index

| ID | 来源 | 原文/稳定锚点 |
| --- | --- | --- |
| SRC-01 | 用户直接指令 | “当前项目没有自动更新、检查更新功能，添加自动更新、检查更新功能” |
| SRC-02 | PRD 9.18.1 | “启动自动检测更新，展示更新日志、新版本特性，离线静默跳过。支持灰度发布、版本黑名单、强制更新、回滚。” |
| SRC-03 | PRD 9.18.2 | “应用启动后，Rust 调用 `tauri-plugin-updater` 按安装包通道检测。” |
| SRC-04 | PRD 9.18.2 | “前端弹窗展示更新日志与「立即更新」「稍后」。” |
| SRC-05 | PRD 9.18.2 | “用户确认后下载、签名校验、安装、重启。” |

## Source Summary

用户要求补齐自动更新与手动检查更新。PRD 进一步限定为：启动自动检查，离线静默失败，发现更新后显示日志并走 Tauri 签名下载/安装流程。

## Key Business Goals

- `source-backed`：启动主应用时自动检查更新（SRC-01, SRC-02, SRC-03）。
- `source-backed`：提供用户主动触发的检查更新能力（SRC-01）。
- `source-backed`：发现更新时展示更新日志和操作对话框（SRC-02, SRC-04）。
- `source-backed`：用户确认后下载、验签、安装并重启（SRC-05）。
- `source-backed`：离线检查失败不阻断应用使用（SRC-02）。

## Greenfield Scaffold Sensitivity

不适用：not a project/app/package/frontend-surface creation request。

## Explicit Behavior Constraints

- 启动检查必须在主应用运行时发生，不应在独立效果窗口中运行。
- 必须复用 Tauri updater 按 free/member 通道检查，签名校验是安装前提。
- 自动检查的网络/离线失败不显示阻断式错误，不影响其他运行时就绪。
- 有可用更新时打开现有更新对话框，对话框展示当前/最新版本和更新日志。
- 强制更新不允许稍后或跳过；非强制更新保留现有“稍后”和“跳过此版本”。
- 手动检查需有防重入 pending 状态，并让用户看到最新、可用或失败结果。

## Closed-World Product Scope Allowlist

1. 启动时自动检查一次。
2. 设置页“关于与许可”中的手动检查按钮和结果反馈。
3. 全局更新对话框，复用现有更新操作与状态。
4. 中英文状态文案与无障碍状态表达。
5. 支撑上述行为所需的运行时编排、store 返回值与测试。

## Forms, Tables, Displays, And Interactions Extracted From Source

- 无新表单或表格。
- 展示：当前版本、最新版本、更新日志、下载进度、手动检查结果。
- 交互：检查更新、立即更新/下载、安装重启、稍后、跳过此版本。

## Workflow And State Rules Extracted From Source

```text
主应用启动
  -> release runtime bootstrap + 事件订阅
  -> 自动检查当前通道
  -> 无更新/离线：静默结束
  -> 有更新：打开全局对话框
  -> 下载 -> Tauri 签名验证 -> 安装/重启
```

```text
手动检查
  -> 检查期间按钮 loading/防重入
  -> current：显示已是最新版本
  -> available/downloaded：打开更新对话框
  -> failure：显示可重试错误，不改变其他应用状态
```

## Relevant Modules Or Pages

- `src/renderer/composables/useReleaseRuntime.ts`
- `src/renderer/stores/release.ts`
- `src/renderer/App.vue`
- `src/renderer/views/settings/index.vue`
- `src/renderer/views/update-log/UpdateAvailableDialog.vue`
- `src-tauri/src/infrastructure/release.rs`
- `.github/workflows/release-tauri.yml`

## Repository Facts

- `code-fact-backed`：原生更新 provider、IPC 命令、发布脚本和更新对话框已存在。
- `code-fact-backed`：`useReleaseRuntime.startGeneration()` 只执行 `store.bootstrap()` 和事件订阅，未执行 `store.checkUpdate()`。
- `code-fact-backed`：`/update-log` 页有手动检查按钮，但当前导航树不暴露该路由；设置页只显示版本/通道。
- `code-fact-backed`：基础 Tauri 配置使更新器保持 disabled mode，发布工作流使用公钥和通道 endpoint 生成临时 release config，这是预期安全边界。

## Notable Open Questions

无需前置人工确认的产品问题。“自动更新”按 PRD 解释为“启动时自动检查并在用户确认后下载/安装”，不扩展为无提示强制下载安装。

## Out Of Scope

- 新增自建更新服务器或替换 GitHub 发布通道。
- 无用户确认的静默下载/安装。
- 插件自动更新、发布流程重构或新的更新频率设置。
- 与本功能无关的设置页重构。
