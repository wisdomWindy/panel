# 执行计划: UI 样式与布局优化

## 计划概述

对所有 10 个前端页面/组件进行纯样式和布局优化，不修改任何业务逻辑。任务以文件为单位组织，按视觉影响从全局到局部排序。

## 任务分解

| 编号 | 任务 | 目标文件 | 说明 |
|------|------|---------|------|
| T1 | StatCard 组件样式优化 | `src/renderer/components/StatCard.vue` | 基础组件，被 Dashboard 使用 |
| T2 | 主布局框架优化 | `src/renderer/App.vue` | 全局 header、侧边栏、主区域背景 |
| T3 | 登录页优化 | `src/renderer/views/Login.vue` | 卡片化设计，品牌元素 |
| T4 | 仪表盘优化 | `src/renderer/views/Dashboard.vue` | 间距统一，统计块背景，hover效果 |
| T5 | 连接配置优化 | `src/renderer/views/Connection.vue` | 表单标签宽度，间距 |
| T6 | 弹幕池管理优化 | `src/renderer/views/DanmakuPool.vue` | 表格列宽，控制面板 |
| T7 | 弹幕列表优化 | `src/renderer/views/DanmakuList.vue` | 弹幕卡片颜色区分，阴影 |
| T8 | 设置页优化 | `src/renderer/views/Settings.vue` | Tab区高度稳定，分隔优化 |
| T9 | 黑名单管理优化 | `src/renderer/views/Blacklist.vue` | 斑马纹，表格高亮 |
| T10 | 特效预览优化 | `src/renderer/views/EffectPreview.vue` | Canvas边框，预览区样式 |

## 依赖关系

```
T1 (StatCard) ──→ T4 (Dashboard 依赖)
T2 (App.vue) ───→ 影响所有页面（全局框架）
T3 ~ T10 ───────→ 相互独立，可任意顺序执行
```

- T1 建议先于 T4 执行
- T2 最先执行以建立全局视觉基准
- T3-T10 无实质依赖，可并行

## 执行顺序

1. **第一波**: T2 (App.vue) — 建立全局框架
2. **第二波**: T1 (StatCard.vue) — 基础组件
3. **第三波**: T3-T10 全部 — 页面级变更（相互独立）

## 各任务详细变更清单

### T2: App.vue

- `<el-header>`: `bg-blue-500` → `style="background: linear-gradient(135deg, #1D3557, #457B9D)"`
- 标题: `text-2xl` → `text-xl font-semibold tracking-wide`
- `<el-aside>`: 移除 `bg-gray-50 border-r border-gray-200`，添加 `style="background-color: #304156"`
- `<el-menu>`: 添加 `background-color="#304156" text-color="#bfcbd9" active-text-color="#409EFF"`
- `<el-main>`: `bg-white p-5` → `p-6`，背景改为 `#F0F2F5`（通过全局样式或父级实现）
- 全局 body 背景设为 `#F0F2F5`

### T1: StatCard.vue

- 外层 div: `bg-gray-50 rounded-lg p-4` → `bg-white rounded-lg p-5 shadow-sm`
- hover: 添加 `hover:shadow-md hover:scale-[1.02] transition-all duration-300`
- label: `text-sm text-gray-600 mb-2` → `text-xs text-gray-500 uppercase tracking-wider mb-2`
- value: `text-2xl font-bold text-gray-800` → `text-3xl font-bold text-gray-800`
- 新增 `valueClass` prop 支持（已存在，确认可用）

### T3: Login.vue

- 外层 div: `bg-gray-100` → `style="background: #F0F2F5"`
- 标题 h1: `text-blue-500` → 使用渐变文字或保留但优化
- 表单外包裹: 新增 `bg-white rounded-lg shadow-lg p-8`
- 顶部 margin: `mb-8` → `mb-6`
- 底部说明: `text-gray-400` → `text-gray-500`

### T4: Dashboard.vue

- 所有 `class="mt-5"` → `class="mt-4"`
- 系统概览统计: 每个 el-col 内包 `p-4 bg-gray-50 rounded-lg`
- 最近活动: 每项 `py-3` 包 `hover:bg-blue-50 transition-colors duration-200`
- 页面外层: `p-5` → `p-6`

### T5: Connection.vue

- 表单 `label-width="120px"` → `label-width="140px"`
- 页面外层 `p-5` → `p-6`
- 连接信息卡片: 间距 `mt-5` 保持

### T6: DanmakuPool.vue

- 页面外层 `p-5` → `p-6`
- 表格内容列: `width="200"` → `min-width="180"`
- 操作按钮: `gap-2` → `gap-1`
- 对话框 `width="450px"` → `width="480px"`

### T7: DanmakuList.vue

- 页面外层 `p-5` → `p-6`
- 弹幕卡片: `bg-gray-50` → `bg-white shadow-sm rounded-lg`
- 收到弹幕: `border-l-blue-400 bg-blue-50`
- 已发送: `border-l-green-400 bg-green-50`
- hover: 添加 `hover:shadow-md transition-shadow duration-200`
- 统计行: 每项外包 `p-4 bg-gray-50 rounded-lg`

### T8: Settings.vue

- 右侧卡片: 添加 `min-height: 500px`（通过 class 或 style）
- 操作按钮: 边距保持
- 页面外层 `p-5` → `p-6`
- 高度约束 `h-[calc(100vh-100px)]` 保持

### T9: Blacklist.vue

- 页面外层 `p-5` → `p-6`
- 表格: 添加 `stripe` 和 `highlight-current-row`
- 对话框 `width="450px"` → `width="500px"`
- el-empty: 已有 description，保持

### T10: EffectPreview.vue

- 页面外层 `p-5` → `p-6`
- Canvas 容器: 添加 `border border-gray-700 rounded-lg overflow-hidden`

## 验证策略

由于是纯 CSS/样式变更，验证方式：

1. **编译验证**: `pnpm run dev` 确保无编译错误
2. **视觉检查**: 打开应用检查所有页面渲染效果
3. **功能检查**: 确认所有交互功能正常

## 风险与回滚

- **风险**: Element Plus 深色菜单模式下子菜单可能样式异常
- **回滚**: 所有变更仅影响 Vue 模板的 class/style 属性，Git revert 即可
- **兼容性**: 仅使用 Tailwind CSS 类和 Element Plus 原生属性，无第三方依赖变更