# 多语言支持 - 执行计划

## 任务拆解

### T1: 集成 vue-i18n 框架

**描述**: 在 main.ts 中集成 vue-i18n 国际化框架，配置中文和英文语言包。

**关联规格**: spec/spec.md - vue-i18n 配置

**执行内容**:
- 在 main.ts 中导入 vue-i18n
- 创建 i18n 实例，配置 legacy: false（Composition API 模式）
- 配置默认语言（zh-CN）
- 配置 fallbackLocale
- 注册 i18n 插件

**验收标准**:
- vue-i18n 框架集成完成
- 应用启动时加载语言包
- `$t()` 函数可用

**执行模式**: 串行

---

### T2: 完善中文语言包

**描述**: 补充中文语言包中缺失的 key，确保覆盖所有页面。

**关联规格**: spec/spec.md - 语言包结构

**执行内容**:
- 检查现有 zh-CN.json
- 补充录制模块（recording）翻译
- 补充更新模块（update）翻译
- 确保所有页面文本都有对应 key

**验收标准**:
- 中文语言包覆盖所有页面
- 无缺失 key

**执行模式**: 串行

---

### T3: 完善英文语言包

**描述**: 补充英文语言包中缺失的 key，确保覆盖所有页面。

**关联规格**: spec/spec.md - 语言包结构

**执行内容**:
- 检查现有 en-US.json
- 补充录制模块（recording）翻译
- 补充更新模块（update）翻译
- 确保所有页面文本都有对应 key

**验收标准**:
- 英文语言包覆盖所有页面
- 无缺失 key

**执行模式**: 串行（依赖 T2）

---

### T4: 配置 Element Plus 多语言

**描述**: 配置 Element Plus 组件库的多语言支持。

**关联规格**: spec/spec.md - Element Plus 多语言配置

**执行内容**:
- 在 main.ts 中导入 Element Plus locale
- 配置中文 locale（zhCn）
- 配置英文 locale（en）
- 与 vue-i18n 同步切换

**验收标准**:
- Element Plus 组件语言与应用语言同步
- 日期选择器、分页等组件显示正确语言

**执行模式**: 串行（依赖 T1）

---

### T5: 添加语言配置持久化

**描述**: 在 electron-store 中添加语言配置，支持持久化用户语言偏好。

**关联规格**: spec/spec.md - 语言配置持久化

**执行内容**:
- 在 database.ts 中添加 locale 配置字段
- 实现读取语言配置 API
- 实现保存语言配置 API
- 在 stores/config.ts 中添加 locale 状态

**验收标准**:
- 语言配置可持久化
- 启动时可读取语言配置

**执行模式**: 串行

---

### T6: 设置页面添加语言切换

**描述**: 在设置页面中添加语言切换下拉菜单。

**关联规格**: spec/spec.md - 设置页面语言切换

**执行内容**:
- 在 Settings.vue 中添加语言切换表单项
- 使用 el-select 组件
- 实现语言切换逻辑
- 切换时更新 vue-i18n locale
- 切换时保存到 electron-store

**验收标准**:
- 设置页面显示语言切换选项
- 切换语言后界面实时更新
- 语言选择持久化

**执行模式**: 串行（依赖 T4, T5）

---

### T7: 页面组件 i18n 改造

**描述**: 将所有页面组件中的硬编码文本替换为 i18n key。

**关联规格**: spec/spec.md - 页面组件 i18n 改造

**执行内容**:
- 改造 App.vue（导航菜单）
- 改造 Dashboard.vue
- 改造 Connection.vue
- 改造 DanmakuPool.vue
- 改造 DanmakuList.vue
- 改造 GiftList.vue
- 改造 Settings.vue
- 改造 Logs.vue
- 改造 Blacklist.vue
- 改造 EffectPreview.vue
- 改造 UpdateLog.vue
- 改造 PluginManager.vue
- 改造 Recording.vue
- 改造所有组件文件

**验收标准**:
- 所有页面文本使用 `$t()` 函数
- 无硬编码中文/英文文本
- 语言切换后所有文本正确更新

**执行模式**: 串行（依赖 T2, T3）

---

### T8: 端到端测试与验证

**描述**: 执行端到端测试，验证所有验收标准。

**关联规格**: spec/spec.md - 验收标准

**执行内容**:
- 验证 AC-01 到 AC-10 所有验收条件
- 记录测试结果到 verification/verification.md

**验收标准**: 所有验收条件通过

**执行模式**: 串行（依赖 T1-T7）

---

## 依赖关系

```
T1 (vue-i18n 集成) → T4 (Element Plus 多语言) → T6 (设置页面语言切换)
T2 (中文语言包) → T3 (英文语言包) → T7 (页面 i18n 改造)
T5 (语言配置持久化) → T6 (设置页面语言切换)
T7 (页面 i18n 改造) → T8 (端到端验证)
```

## 并行执行建议

当前任务集规模适中（8 个任务），任务间依赖关系紧密，不建议启用 workflow-style 并行执行。所有任务按串行顺序执行。

**推荐执行顺序**: T1 → T2 → T3 → T4 → T5 → T6 → T7 → T8

## 受影响文件或模块

| 文件/模块 | 变更类型 | 关联任务 |
|-----------|----------|----------|
| `src/renderer/main.ts` | 修改 | T1, T4 |
| `src/renderer/locales/zh-CN.json` | 修改 | T2 |
| `src/renderer/locales/en-US.json` | 修改 | T3 |
| `src/main/database.ts` | 修改 | T5 |
| `src/renderer/stores/config.ts` | 修改 | T5 |
| `src/renderer/views/Settings.vue` | 修改 | T6 |
| `src/renderer/App.vue` | 修改 | T7 |
| `src/renderer/views/Dashboard.vue` | 修改 | T7 |
| `src/renderer/views/Connection.vue` | 修改 | T7 |
| `src/renderer/views/DanmakuPool.vue` | 修改 | T7 |
| `src/renderer/views/DanmakuList.vue` | 修改 | T7 |
| `src/renderer/views/GiftList.vue` | 修改 | T7 |
| `src/renderer/views/Logs.vue` | 修改 | T7 |
| `src/renderer/views/Blacklist.vue` | 修改 | T7 |
| `src/renderer/views/EffectPreview.vue` | 修改 | T7 |
| `src/renderer/views/UpdateLog.vue` | 修改 | T7 |
| `src/renderer/views/PluginManager.vue` | 修改 | T7 |
| `src/renderer/views/Recording.vue` | 修改 | T7 |
| `src/renderer/components/*.vue` | 修改 | T7 |

## 测试策略

### 单元测试

| 测试目标 | 测试内容 | 测试框架 |
|----------|----------|----------|
| i18n 配置 | 语言包加载、key 映射 | Vitest |
| 语言切换 | locale 切换、持久化 | Vitest |

### 集成测试

| 测试目标 | 测试内容 |
|----------|----------|
| 语言切换流程 | 选择语言 → 界面更新 → 持久化 |
| 启动加载流程 | 读取配置 → 设置语言 → 显示文本 |

### 端到端测试

验证所有 AC-01 到 AC-10 验收条件。

## 回滚说明

如果实现过程中发现问题：

1. **T1 失败**: 检查 vue-i18n 版本兼容性
2. **T4 失败**: 检查 Element Plus locale 配置
3. **T6 失败**: 回退到 T5，检查配置存储
4. **T7 失败**: 回退到 T2/T3，检查语言包
5. **T8 验证失败**: 回退到对应任务修复问题

每个任务完成后应提交代码，便于回滚。
