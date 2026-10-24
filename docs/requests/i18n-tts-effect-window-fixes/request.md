# 请求：国际化、语音播报与上舰特效窗口修复

## 请求标识

`i18n-tts-effect-window-fixes`

## 来源

用户在 2026-09-22 提交的直接缺陷清单，共五项。

## 业务摘要

修复语言切换只影响菜单、TTS 默认音色与重置反馈缺失，以及上舰特效窗口不可配置尺寸且位置错误的问题。

## 目标

免费版导航可达页面在切换 English 后显示英文界面；TTS 默认优先选中 Microsoft HuiHui，重置会保存并明确提示结果；上舰特效窗口尺寸可配置并按所选屏幕位置显示。

## 初始完成信号

- English 模式下，免费版各业务页面及其主要弹窗/子组件不再保留硬编码中文界面文案。
- 可用系统音色包含 Microsoft HuiHui 且原配置未选音色时，页面默认选中该音色。
- TTS 重置确认后持久化默认配置，并显示成功或失败提示。
- 上舰特效页面可配置窗口宽度和高度，保存及预览链路均携带该配置。
- 特效窗口按主显示器上的所选位置创建，居中不会再落在左上角。

## 触发条件

用户切换界面语言、进入语音播报页、执行语音播报重置、保存或预览上舰特效。

## 初始上下文来源

- 用户缺陷原文。
- Vue 页面、locale 资源、Pinia store 与桌面 IPC 契约。
- Rust media domain/application/infrastructure 窗口与 TTS 实现。

## 人工交接点

完成后交付验证结果、运行预览地址和更新后的安装包。

## 影响区域

renderer 国际化与媒体页面、TTS 配置交互、CaptainEffectConfig 前后端契约、Tauri 特效窗口创建与定位。

## 参与模块

- Vue 3 / vue-i18n / Element Plus 页面层
- renderer media contracts/adapters/stores
- Rust media domain/application/IPC/infrastructure

