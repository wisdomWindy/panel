# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-06-16

### Added
- 新增更新日志查看功能，支持菜单栏「帮助 → 更新日志」入口
- 新增版本自动检测功能，启动时检查是否有新版本可用
- 新增"新版本特性"弹窗，版本变更后首次启动展示更新摘要
- 新增状态栏更新提示图标，检测到新版本时显示红点
- 新增设置页面更新检查开关，支持开启/关闭自动检查更新

### Changed
- 优化更新检查为异步执行，不阻塞主界面加载

### Fixed
- 修复离线状态下可能出现错误提示的问题

## [0.1.0] - 2026-06-01

### Added
- 初始发布版本
- 支持 Bilibili 直播间连接与弹幕接收
- 支持自动弹幕发送（定时/随机）
- 支持礼物感谢弹幕
- 支持基础 UI 界面
- 支持配置文件保存与加载
- 支持本地敏感词过滤
- 支持直播数据看板
- 支持上舰特效展示
