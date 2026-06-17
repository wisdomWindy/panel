# PRD 快照

## 来源摘要

来自 `docs/requirements/Bilibili_Live_Tool_PRD.md` 第 3.5 节「弹幕语音播报」。该功能为 Should Have 级别功能，将直播间的弹幕内容通过语音合成（TTS）技术实时播报为主播可听到的语音。

## 关键业务目标

- 帮主播在不看屏幕的情况下了解弹幕内容
- 提升互动体验和直播效率
- 支持选择性播报和智能过滤
- 支持礼物和舰长播报

## 相关模块/页面

- 新增 `src/renderer/modules/tts/`：TTS 模块
- 新增 `src/renderer/services/tts-service.ts`：TTS 服务
- 新增 `src/renderer/stores/tts.ts`：TTS 状态管理
- `src/renderer/views/Settings.vue`：设置页面新增 TTS 配置
- `src/renderer/App.vue`：主框架新增 TTS 快捷开关
- `src/main/database.ts`：新增 tts_config 和 tts_logs 数据表

## 上游需确认问题

1. TTS 引擎选择：使用 Web Speech API 还是集成第三方 TTS 服务？
2. 播报队列大小：最大支持多少条弹幕排队？
3. 语音缓存策略：是否需要缓存常用语音片段？
4. 多语言播报：是否需要支持中英文混合播报？
