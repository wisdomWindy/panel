# PRD 快照

## 来源摘要

用户直接提出全局布局重设计需求：间距适当、层次分明、重点突出。无上游 PRD 文档。

## 关键业务目标

- 全局视觉层次清晰，内容区域有呼吸感
- 统计卡片间距统一、重点数据突出
- 各页面卡片间距舒适，内容分组明确
- 侧边栏和页头与内容区过渡自然

## 相关模块/页面

- `src/renderer/App.vue`：主框架
- `src/renderer/components/StatCard.vue`：统计卡片
- 所有视图：Dashboard, Connection, DanmakuPool, DanmakuList, GiftList, Settings, Blacklist, Logs, EffectPreview, Login

## 上游需确认问题

无。需求为全局审美优化。