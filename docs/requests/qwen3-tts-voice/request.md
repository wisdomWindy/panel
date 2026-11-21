# 请求：语音播报与 AI 语音回复改用 Qwen3-TTS 生成语音

- 请求 ID：`qwen3-tts-voice`
- 提出时间：2026-11-20
- 参考实现：[QwenLM/Qwen3-TTS](https://github.com/QwenLM/Qwen3-TTS#model-architecture)
- 代码基线：`packages/bilibili-live-tool`（Tauri 2 + Vue 3 + Rust）

## 原始诉求

调整当前项目的语音播报和 AI 语音回复功能，使用 Qwen3-TTS 项目生成语音。

## 范围

范围内：

1. 语音播报（直播间事件模板播报）的合成引擎接入 Qwen3-TTS。
2. AI 语音回复（LLM 回复朗读）的合成引擎接入 Qwen3-TTS。
3. 两条链路共用的服务端、配置项、降级开关、健康探测与前端设置界面。

范围外（本次不做）：

- 声音克隆（Base 模型 3 秒克隆）与声音设计（VoiceDesign）的产品化。
- 流式首包播放（一期使用整段 WAV）。
- 模型随应用分发的打包方案。
- 付费 DashScope API 通道（仅保留可配置的能力位）。

## 约束

| 约束 | 说明 |
| --- | --- |
| 不破坏既有能力 | 系统 TTS 必须保持可用，且可一键回退 |
| AI 回复音频路由 | 必须继续尊重运维选定的输出设备（SAPI 设备语义） |
| 无明文密钥 | 服务端与客户端不得内置任何密钥 |
| 无硬编码绝对路径 | 路径走配置/临时目录 |
| 离线可用 | 未启动模型服务时，应用照常工作 |

## 验收口径

见同目录 `../verification/verification.md` 的清单与证据；产品级验收以“播报与 AI 回复两条路径均可由 Qwen3-TTS 发声，且关闭开关后 100% 回到系统语音”为准。
