# Code Context

## Context requirement

- 本请求删除跨 renderer/main/plugin/docs 的既有功能链，intake 起需要结构理解；code graph 为优先但非唯一手段。

## Code graph 状态

- 本仓库未发现 `.codegraph` 配置或可执行的 `codegraph` 命令。
- 仓库没有记录团队首选的安装或 bootstrap 命令，因此没有可安全执行的自动 bootstrap 入口。
- 采用 scoped `rg`、直接 imports/callers 阅读和编译检查恢复依赖闭包。
- 检测/执行上下文：loop iteration 4；bootstrap attempted：no；result：not_needed；fallback confidence：高，残留搜索与三段 build 均通过。

## 生产依赖链

1. `DanmakuSender` 调用 `/msg/send`。
2. `DanmakuManager.sendDanmaku` 被手动发送、角色回复、触发词、礼物感谢、舰长感谢和随机/定时发送调用。
3. `DanmakuService` 将发送结果写入 `useDanmakuStore` 并向页面/插件暴露发送和弹幕池方法。
4. `danmaku-list`、`danmaku-pool`、settings、router/App 菜单和 plugin API 构成用户及扩展入口。
5. main database/IPC 保存弹幕池和发送配置。

## 保留链路

- Bilibili Open Platform start/WebSocket/end。
- 弹幕与礼物接收、解析、展示、统计、TTS、录制与黑名单。

## 变更后链路与边界

- `views -> DanmakuService/store -> DanmakuManager -> BilibiliClient/parser`，只处理官方连接和接收事件。
- `plugins -> PluginAPI.onDanmaku/onGift/onCaptain`，只订阅事件，不持有发送 service。
- `useRoomStore` 拥有连接状态；`useDanmakuStore` 拥有接收事件；不存在发送/pool writable owner。
- 历史 config 在 `mergeAppConfig` / `mergeAppDataConfig` 白名单边界归一，旧发送字段不进入当前模型。

## 文档闭包

- 产品自有 README、requirements、architecture、implementation、plugin-development、homepage 文档需要去除现状能力宣称。
- `description-doc/` 是官方资料快照，保持不变。
- 已完成历史请求工件是审计记录，保持不变，不做追溯改写。

## Impact scope 与复查结果

- 影响面：manager/service/store/types、main/renderer persistence、views/router/settings/locales、plugin API/runtime、tests 和当前产品文档。
- 生产源码发送符号残留为零；接收、录制、黑名单、TTS 与礼物链保留真实调用方。
- open follow-up checks：none；verify/review 仅需独立复核证据与基线失败归属。
