# 需求拆解：按 PRD v2 与 UI 设计图重构主界面

请求 ID：`ui-redesign` ｜ 输入：`docs/requirements/Bilibili 直播助手 - 产品需求文档 (PRD) v2.md`、`docs/requirements/ui/*.png`、`ui/bilibili-live-helper-ui.html`

## 1. 目标信息架构（PRD §8.1）

顶部工具栏 48px + 左侧导航 208px + 中央内容区 + 底部状态栏。

| # | 导航项 | 目标内容 | 现有实现 | 处置 |
| --- | --- | --- | --- | --- |
| 1 | 直播间 | 房间信息 + 4 指标卡 + 最近事件流 | 无（当前 `/` 跳 dashboard） | **新建** |
| 2 | 弹幕面板 | 身份过滤 chip + 搜索 + 消息流 + 身份图例 | `views/danmaku-list` | 重写模板 |
| 3 | 数据看板 | 指标卡 + 三张趋势图 + 导出 | `views/dashboard` | 保留，套深色 |
| 4 | 排行榜 | 送礼/弹幕/舰长榜 | `views/rankings` | 保留，套深色 |
| 5 | 风控中心 | 敏感词 + 黑名单 | `sensitive-words`、`blacklist`、`moderation` | **合并为分组页**（页签） |
| 6 | 提醒配置 | 礼物提醒 + 舰长特效 + 礼物音效 | `gift-reminder`、`captain-effect`、`gift-sound` | **合并为分组页** |
| 7 | 语音播报 | TTS 设置 + 队列 | `views/tts` | 保留，套深色 |
| 8 | 直播录制 | 录制控制 + 记录列表 | `views/recording` | 保留，套深色 |
| 9 | AI 助手 | 人设/模型/语音回复/日志 | `views/ai` | 保留，套深色 |
| 10 | 系统设置 | 通用 + 语言 + 日志 + 更新 + 配置 | `settings`、`logs`、`update-log` | **合并为分组页** |

## 2. 视觉契约（取自 `ui/bilibili-live-helper-ui.html`）

```
--bg:#13151a  --panel:#1c1f27  --panel2:#232732  --side:#171920  --border:#2b303b
--text:#e9ebf0  --text2:#a6adb9  --text3:#737b89
--primary:#00aeec  --primary2:#2fc0f5
--gold:#f5a623  --green:#34c759  --purple:#af52de  --blue:#5ac8fa  --orange:#ff9500  --red:#ff453a
```

尺寸基线：工具栏 48px、导航宽 208px、导航项内边距 12px 10px、徽标高 26px、状态点 8px、内容区 padding 16px、卡片间距 16px。

身份配色：房管=primary、总督=purple、提督=blue、舰长=gold、老爷=orange、普通=text3（与 `preview-danmu.png` 的「身份图例」一致）。

## 3. 明确不做

- PRD §9 的功能语义已有 Rust 实现，本次**不改 IPC 契约、不改数据库**，只改前端呈现与路由组织。
- 会员/登录/充值入口：PRD 明确工具栏与导航不得出现，保持现状（现有导航无此类入口）。
- 多窗口特效：`captain-effect` 独立窗口保持不动。

## 4. 风险

| 风险 | 处置 |
| --- | --- |
| 一次性重写 15 个视图会失控 | 先落设计令牌 + 外壳 + 直播间/弹幕面板两个像素级页面；其余视图通过 Element Plus 深色变量与令牌统一染色，保持功能不变 |
| 合并分组页导致既有深链失效 | 保留旧路径作为重定向别名 |
| 现有契约测试按字符串断言视图内容 | 改动后同步更新断言，保证测试仍表达真实约束 |
