# 任务看板: UI 样式与布局优化

## 状态说明

- ⬜ 待开始
- 🔄 进行中
- ✅ 已完成
- ⏸️ 暂停
- ❌ 已取消

## 任务列表

| 编号 | 状态 | 任务 | 文件 | 依赖 |
|------|------|------|------|------|
| T2 | ⬜ | 主布局框架优化 | `App.vue` | — |
| T1 | ⬜ | StatCard 组件优化 | `StatCard.vue` | — |
| T3 | ⬜ | 登录页优化 | `Login.vue` | T2 |
| T4 | ⬜ | 仪表盘优化 | `Dashboard.vue` | T1, T2 |
| T5 | ⬜ | 连接配置优化 | `Connection.vue` | T2 |
| T6 | ⬜ | 弹幕池管理优化 | `DanmakuPool.vue` | T2 |
| T7 | ⬜ | 弹幕列表优化 | `DanmakuList.vue` | T2 |
| T8 | ⬜ | 设置页优化 | `Settings.vue` | T2 |
| T9 | ⬜ | 黑名单管理优化 | `Blacklist.vue` | T2 |
| T10 | ⬜ | 特效预览优化 | `EffectPreview.vue` | T2 |

## 执行记录

_执行中更新_

## 验收检查

- [ ] App.vue: header 渐变深蓝、深色侧边栏、主区域 #F0F2F5 背景
- [ ] StatCard.vue: 白色背景、阴影、hover 放大效果
- [ ] Login.vue: 白色卡片、品牌元素、表单居中
- [ ] Dashboard.vue: 间距统一 mt-4、统计块有背景、hover 效果
- [ ] Connection.vue: 表单 label 140px、间距合理
- [ ] DanmakuPool.vue: 表格列宽合理、控制面板清晰
- [ ] DanmakuList.vue: 弹幕卡片白色背景、蓝/绿区分
- [ ] Settings.vue: Tab 高度稳定、分隔清晰
- [ ] Blacklist.vue: 斑马纹、行高亮
- [ ] EffectPreview.vue: Canvas 有边框
- [ ] 编译通过，功能正常