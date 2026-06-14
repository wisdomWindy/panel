# 请求: API 对齐 - Bilibili 开放平台官方 API

## 基本信息

- **请求 ID**: api-alignment-open-platform
- **创建日期**: 2026-06-11
- **来源**: description-doc/ 目录中的 Bilibili 直播开放平台官方文档
- **文档来源**: https://open-live.bilibili.com/document/

## 描述

根据 description-doc/ 目录中 23 个 Bilibili 直播开放平台官方文档，将项目中的 API 实现从旧版广播协议迁移对齐到官方开放平台 API 体系。

### 关键差异分析

| 领域 | 当前项目实现 | 官方平台 API |
|------|-------------|-------------|
| 域名 | `broadcastlv.chat.bilibili.com` (广播服务器) | `https://live-open.biliapi.com` (开放平台) |
| 鉴权 | 基于 room_id + token 的认证协议 | HMAC-SHA256 签名 + access_key/access_secret |
| 连接方式 | 直接连接广播 WebSocket，发送 AUTH 包 | `/v2/app/start` API 返回 wss_link + auth_body |
| 长连消息 | 旧版 `DANMU_MSG`, `SEND_GIFT` 等 cmd | 新版 `LIVE_OPEN_PLATFORM_DM`, `LIVE_OPEN_PLATFORM_SEND_GIFT` 等 |
| 生命周期 | 手动心跳 | `/v2/app/start` → `/v2/app/heartbeat` → `/v2/app/end` |
| 用户标识 | `uid` (已废弃) | `open_id`, `union_id` |
| 消息格式 | proto 二进制协议 | WebSocket JSON + proto 压缩 |

## 状态

- **阶段**: intake