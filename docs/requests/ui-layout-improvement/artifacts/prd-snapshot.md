# PRD 快照: UI 样式与布局优化

## 来源

用户直接需求（非 PRD 来源），基于当前项目页面审查

## 核心业务目标

提升 Bilibili 直播助手整体的视觉美观度和用户体验，使布局间距合理、样式美观。

## 涉及模块/页面

| 页面 | 文件路径 | 主要改进点 |
|------|---------|-----------|
| 主布局 | `src/renderer/App.vue` | header 渐变配色、侧边栏样式、主区域背景 |
| 登录页 | `src/renderer/views/Login.vue` | 卡片化设计、品牌感增强 |
| 仪表盘 | `src/renderer/views/Dashboard.vue` | 间距统一、卡片阴影层次 |
| 连接配置 | `src/renderer/views/Connection.vue` | 表单间距、描述列表样式 |
| 弹幕池管理 | `src/renderer/views/DanmakuPool.vue` | 表格样式、控制面板视觉 |
| 弹幕列表 | `src/renderer/views/DanmakuList.vue` | 弹幕卡片样式、统计区优化 |
| 设置 | `src/renderer/views/Settings.vue` | 左侧菜单样式、表单分组 |
| 黑名单管理 | `src/renderer/views/Blacklist.vue` | 表格样式、弹窗优化 |
| 特效预览 | `src/renderer/views/EffectPreview.vue` | 参数面板、预览区样式 |
| 统计卡片 | `src/renderer/components/StatCard.vue` | 图标、颜色指示、悬停效果 |

## 当前 UI 问题汇总

1. 整体 header 仅使用单一蓝色 `bg-blue-500`，缺乏层次感
2. 侧边栏颜色较为单调 (`bg-gray-50`)
3. 各页面间距不统一（`class="mt-5"` 有硬编码间距）
4. StatCard 组件样式过于简单，缺少视觉吸引力
5. 表格和表单在不同页面间风格不完全统一
6. 缺少页面级的视觉过渡和品牌感

## 值得注意的开放问题

- 用户未指定具体的配色方案偏好
- 需要保留所有现有功能不变
- 需兼容 Element Plus 组件库的默认样式