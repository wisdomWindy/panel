# 规格说明书: UI 样式与布局优化

## 概述

对 Bilibili 直播助手全部页面的视觉样式和布局进行统一优化，建立一致的设计语言，使页面布局间距合理、样式美观。

## 范围

### 涉及文件

| 文件 | 类型 |
|------|------|
| `src/renderer/App.vue` | 主布局 |
| `src/renderer/components/StatCard.vue` | 通用组件 |
| `src/renderer/views/Login.vue` | 登录页 |
| `src/renderer/views/Dashboard.vue` | 仪表盘 |
| `src/renderer/views/Connection.vue` | 连接配置 |
| `src/renderer/views/DanmakuPool.vue` | 弹幕池管理 |
| `src/renderer/views/DanmakuList.vue` | 弹幕列表 |
| `src/renderer/views/Settings.vue` | 设置 |
| `src/renderer/views/Blacklist.vue` | 黑名单管理 |
| `src/renderer/views/EffectPreview.vue` | 特效预览 |

### 不变范围

- 所有业务逻辑代码（script 部分）不修改
- API 调用、数据流、状态管理不修改
- 组件交互行为不修改
- Element Plus 组件的功能性属性不修改

## 设计系统

### 色彩体系

```
Primary: #409EFF → #337ECC (Element Plus 默认蓝)
Background: #F0F2F5 (页面背景，替换 #E5E7EB/gray-100)
Card Background: #FFFFFF
Sidebar Background: #304156 (深色侧边栏，替换 gray-50)
Header Background: linear-gradient(135deg, #1D3557 → #457B9D) (渐变深蓝)
Text Primary: #303133
Text Secondary: #909399
Text Muted: #C0C4CC
Border: #EBEEF5
Success: #67C23A
Warning: #E6A23C
Danger: #F56C6C
```

### 间距体系

统一使用 Element Plus 原生间距或 Tailwind 语义化间距：

| Token | 值 | 用途 |
|-------|-----|------|
| xs | 4px / p-1 | 紧凑间距 |
| sm | 8px / p-2 | 小间距 |
| md | 16px / p-4 | 默认 padding |
| lg | 20px / p-5 | 卡片 padding、行间距 |
| xl | 24px / p-6 | 页面边缘 padding |

页面间 el-row 间距统一使用 `:gutter="20"`。

### 阴影层次

```css
/* 卡片阴影 */
box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);

/* 悬停阴影 */
box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
```

### 圆角

- 卡片: `border-radius: 8px`
- 按钮保持 Element Plus 默认
- 输入框保持 Element Plus 默认

## 页面规格

### 1. App.vue - 主布局框架

**当前问题**: header 简单单一蓝色，侧边栏灰白色背景，缺乏层次感。

**改进措施**:
- Header: 使用 `background: linear-gradient(135deg, #1D3557, #457B9D)` 替换 `bg-blue-500`。标题字号 `text-xl`，加 `font-semibold`，`tracking-wide`
- Sidebar: 使用深色背景 `#304156`，白色文字菜单。不再使用 `bg-gray-50`
- `el-menu` 使用深色模式：`background-color="#304156" text-color="#bfcbd9" active-text-color="#409EFF"`
- Main 区域：背景 `#F0F2F5`，padding 改为 `p-6`
- 整体 `min-height: 100vh`

**验收标准**:
- Header 显示渐变深蓝色调，标题文字清晰居中
- 侧边栏深色背景，菜单项白色文字，激活项蓝色高亮
- 主内容区背景浅灰色 `#F0F2F5`，与卡片形成对比层次

### 2. Login.vue - 登录页

**当前问题**: 背景 `bg-gray-100` 太灰，视觉单调，无品牌元素。

**改进措施**:
- 背景改为 `#F0F2F5`
- 标题区增加 B 站主题装饰（微渐变文字或彩色下划线），标题色从 `text-blue-500` 改为 brand gradient
- 表单外层包裹一个白色卡片容器：`max-w-md bg-white rounded-lg shadow-lg p-8`
- 减少顶部标题 margin 从 `mb-8` 到 `mb-6`
- 底部说明文字颜色 `#909399`
- 整体布局使用 flex 垂直水平居中

**验收标准**:
- 登录卡片白色背景、圆角、柔和阴影
- 标题有明显的品牌色调
- 表单区域视觉突出，背景与卡片形成层次

### 3. Dashboard.vue - 仪表盘

**当前问题**: 间距不一致混用 `mt-5`，统计区使用 `el-statistic` 样式单调，图表卡片缺少视觉层次。

**改进措施**:
- 所有 `class="mt-5"` 替换为 `class="mt-4"` (统一 16px)
- 统计算子区域：使用自定义统计卡片（保持 el-statistic 结构但用卡片包裹），统计项增加图标颜色
- 图表卡片内的图表容器增加 `min-height: 300px`
- "系统概览" 卡片中 el-statistic 每项外包裹 `bg-gray-50 rounded-lg p-4` 背景块
- 最近活动区域：卡片悬停加背景色过渡 `hover:bg-blue-50 transition-colors duration-200`
- 图表标题使用 `font-semibold text-gray-700`

**验收标准**:
- 所有 `el-row` 间距统一为 `:gutter="20"` + `mt-4`
- 统计数字卡片有浅灰背景块分隔
- 最近活动列表项hover有背景色变化

### 4. Connection.vue - 连接配置

**当前问题**: 表单行间距不够明显，`label-width="120px"` 偏窄。

**改进措施**:
- label-width 改为 `"140px"`
- 表单内添加 `.el-form-item { margin-bottom: 22px }` 级别的间距
- 连接信息描述列表保留 element-plus 原生样式
- 两个卡片间距统一 `mt-5`
- 操作按钮组间距保持 `ml-2`

**验收标准**:
- 表单标签对齐，项目间有明显但不过大的间距
- 描述列表信息清晰可读

### 5. DanmakuPool.vue - 弹幕池管理

**当前问题**: 表格编辑控件密集，控制面板样式简单。

**改进措施**:
- 表格列宽优化：内容列加 `min-width="180"`，间隔列 `width="120"`
- 自动发送控制面板使用 `el-descriptions` 或卡片分组展示状态信息
- 操作按钮间距统一 `gap-1`
- 添加弹幕对话框 `width="480px"`

**验收标准**:
- 表格在正常窗口宽度下无横向滚动
- 控制面板信息分布清晰

### 6. DanmakuList.vue - 弹幕列表

**当前问题**: 弹幕卡片边框颜色单一 `border-l-4 border-blue-500`，统计行数值为硬编码 0。

**改进措施**:
- 弹幕卡片增加圆角 `rounded-lg`，阴影 `shadow-sm`，背景改为白色 `bg-white` 替代 `bg-gray-50`
- 已发送弹幕使用 `border-l-green-400 bg-green-50`，接收弹幕使用 `border-l-blue-400 bg-blue-50`
- 卡片hover效果：`hover:shadow-md transition-shadow duration-200`
- 统计行保持 `el-statistic`，背景包裹在 `p-4 bg-gray-50 rounded-lg` 中
- 最新弹幕默认滚动到可见位置

**验收标准**:
- 弹幕卡片视觉层次分明，收到/已发送区分明显
- 统计数字区布局干净

### 7. Settings.vue - 设置

**当前问题**: 左侧菜单和右侧表单匹配度不够，表单分隔不够清晰。

**改进措施**:
- 右侧表单卡片加 `min-height: 500px` 避免页面高度跳变
- 每个 tab 内容区顶部显示 tab 标题（已有）
- 角色回复区的 `el-divider` 增加间距
- 操作按钮组置于卡片底部可浮动
- 表单全局 label-width 保持 `"150px"`

**验收标准**:
- 切换 Tab 时表单区域高度稳定
- 角色回复三个区域分隔清晰
- 保存按钮区始终可见

### 8. Blacklist.vue - 黑名单管理

**当前问题**: 表格组件样式基础。

**改进措施**:
- 表格增加 `highlight-current-row` 属性
- 空状态 `el-empty` 居中显示，添加 `description="暂无黑名单记录"`
- 添加对话框宽度 `"500px"`
- 表格 `stripe` 属性增加斑马纹

**验收标准**:
- 表格有斑马纹、行悬停高亮
- 空状态提示友好

### 9. EffectPreview.vue - 特效预览

**当前问题**: 参数面板与 Canvas 区域的视觉比例关系不够均衡。

**改进措施**:
- 预览区 Canvas 背景保持 `bg-gray-900`（暗色适合查看特效）
- 参数表单增加分组标题
- 按钮组 `flex gap-2` 保持
- Canvas 容器增加 `border border-gray-700 rounded-lg overflow-hidden`

**验收标准**:
- 参数面板和预览区的比例均衡（8:16 的 el-col 分栏）
- Canvas 区有明显的边界

### 10. StatCard.vue - 统计卡片组件

**当前问题**: 样式过于简单，无颜色区分，无悬停效果。

**改进措施**:
- 将背景改为白色 `bg-white`，增加 `shadow-sm rounded-lg p-5`
- 增加 hover 效果 `hover:shadow-md hover:scale-[1.02] transition-all duration-300`
- label 颜色改为 `text-gray-500` 字体 `text-xs uppercase tracking-wider`
- value 颜色改为 `text-gray-800`，字号 `text-3xl font-bold`
- 支持通过 slot 传递内容（已有）
- 增加 `valueClass` prop 支持传入不同的颜色类

**验收标准**:
- 统计卡片有白色背景、圆角、阴影
- 悬停有轻微放大和阴影增强效果
- label 和 value 层级清晰

## 验收标准总结

### 全局
- [ ] 所有页面背景统一为 `#F0F2F5`
- [ ] Header 使用渐变深蓝色
- [ ] 侧边栏使用深色背景
- [ ] 卡片间距统一为 `gutter="20"` + `mt-4`
- [ ] 所有功能保持正常工作

### 组件
- [ ] StatCard 有白色背景、阴影、hover 效果
- [ ] 弹幕卡片样式层次分明

### 各页面
- [ ] 登录页卡片化设计
- [ ] 仪表盘图表间距合理
- [ ] 设置页 Tab 切换高度稳定
- [ ] 黑名单表格有斑马纹
- [ ] 特效预览布局均衡

## 风险评估

- **低风险**: 纯 CSS/样式修改，不影响业务逻辑
- **Element Plus 兼容性**: 深色菜单模式下需确保子菜单也正确渲染
- **响应式**: 保持现有桌面端布局，不增加移动端适配需求