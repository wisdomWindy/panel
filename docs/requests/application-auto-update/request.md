# 应用自动更新与检查更新

## 请求标识

`application-auto-update`

## 来源

- 用户直接指令：“当前项目没有自动更新、检查更新功能，添加自动更新、检查更新功能”。
- 仓库 PRD：`docs/requirements/Bilibili 直播助手 - 产品需求文档 (PRD).md` 第 9.18 节。

## 业务摘要

在现有 Tauri 签名更新、发布通道、更新 IPC、Pinia store 和更新对话框基础上，补齐应用启动时的自动检查与用户可见的手动检查入口，使可用更新能够展示、下载、验签并安装。

## 目标

1. 主应用启动后自动检查当前发布通道的新版本。
2. 检测到可用更新时，在全局显示现有更新对话框，并支持下载、签名校验、安装/重启、稍后和跳过非强制版本。
3. 设置页的“关于与许可”区域提供明确的“检查更新”入口和当前结果反馈。
4. 离线或更新服务暂时不可用时不影响应用启动与正常使用。

## 初始完成信号

- 自动检查只在主应用运行时启动一次，效果窗口不启动更新流程。
- 手动检查有 pending 防重入与明确的“已是最新版本 / 发现新版本 / 检查失败”反馈。
- 自动检查发现新版本时打开对话框；强制更新不允许“稍后”或“跳过”。
- 现有发布配置继续使用 HTTPS 通道端点与 Tauri 签名校验，未签名或非法 manifest 不安装。
- 相关单元/契约测试、TypeScript 检查和构建通过。

## 触发条件

- 主应用根运行时 mounted 后启动发布运行时。
- 用户在设置页主动点击“检查更新”。

## 上下文来源

- 用户直接指令。
- PRD 9.18 的启动检查、离线静默、灰度/黑名单/强制更新与下载安装流程。
- 现有 `useReleaseRuntime` -> `useReleaseStore` -> Tauri IPC -> `TauriUpdateProvider` 链路。
- 现有 `UpdateAvailableDialog.vue` 与 `/update-log` 页面。
- 现有 `.github/workflows/release-tauri.yml` 和更新 manifest 生成脚本。

## 人工交接点

发布前由维护者在 GitHub Actions 配置 `TAURI_SIGNING_PRIVATE_KEY` / `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` 和 `TAURI_SIGNING_PUBLIC_KEY`，并使用发布工作流生成带签名的更新产物。

## 影响区域

- 发布运行时与 store 的启动检查编排。
- 主应用根节点的全局更新对话框。
- 设置页关于区域的手动检查。
- 中英文更新状态文案。
- 对应的 store/runtime/页面契约测试。

## 交付单元

本请求是单一请求级交付单元，不需要 PRD-only requirement-analysis / requirement-splitting：用户直接指令已给出目标，仓库 PRD 9.18 与现有更新契约可以封闭关键行为语义。不进行新页面设计，复用现有对话框和设置页视觉系统。
