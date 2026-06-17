# 弹幕语音播报功能

## 请求标识

- **request-id**: danmaku-tts
- **创建时间**: 2026-06-16

## 来源

`docs/requirements/Bilibili_Live_Tool_PRD.md` 第 3.5 节「弹幕语音播报」。

## 业务摘要

为 Bilibili 直播助手提供弹幕语音播报能力，将直播间的弹幕内容通过语音合成（TTS）技术实时播报为主播可听到的语音，帮助主播在不查看屏幕的情况下了解弹幕内容，提升互动体验和直播效率。

## 目标声明

在 Bilibili 直播助手中实现完整的弹幕语音播报功能，具体包括：

1. 使用 Web Speech API 实现本地 TTS 播报
2. 收到弹幕时自动朗读弹幕内容
3. 支持按用户身份过滤播报对象
4. 自动跳过敏感词、广告、刷屏内容
5. 弹幕进入播报队列，按顺序播放
6. 支持礼物播报和舰长播报
7. 支持快捷开关、音量控制、暂停功能
8. 播报配置持久化

## 初始完成信号

- [ ] TTS 引擎集成完成（Web Speech API）
- [ ] 弹幕播报功能正常
- [ ] 礼物播报功能正常
- [ ] 舰长播报功能正常
- [ ] 播报队列管理正常
- [ ] 智能过滤功能正常
- [ ] 设置页面可配置播报规则
- [ ] 快捷开关功能正常
- [ ] 音量控制功能正常
- [ ] 播报延迟 < 2 秒

## 影响范围

- 新增 `src/renderer/modules/tts/` — TTS 模块
- 新增 `src/renderer/services/tts-service.ts` — TTS 服务
- 新增 `src/renderer/stores/tts.ts` — TTS 状态管理
- `src/renderer/views/Settings.vue` — 设置页面新增 TTS 配置项
- `src/renderer/App.vue` — 主框架新增 TTS 快捷开关
- `src/main/database.ts` — 新增 tts_config 和 tts_logs 数据表

## 参与模块

- Web Speech API（浏览器原生 TTS）
- Pinia 状态管理（TTS 配置持久化）
- Element Plus UI 组件（设置页面）
- 弹幕模块（弹幕消息流）
- 礼物模块（礼物消息流）
