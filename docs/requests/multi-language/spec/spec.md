# 多语言支持 - 功能规格

## 背景与目标

Bilibili 直播助手当前界面文本为中文硬编码，无法支持其他语言用户。本功能旨在：

1. 集成 vue-i18n 国际化框架
2. 完善中文和英文语言包
3. 所有界面文本使用 i18n key
4. 设置页面提供语言切换功能
5. 语言选择持久化

## 范围内

- vue-i18n 框架集成和配置
- 中文语言包（zh-CN.json）完善
- 英文语言包（en-US.json）完善
- 设置页面语言切换功能
- 语言配置持久化（electron-store）
- 所有页面组件 i18n 改造
- Element Plus 多语言配置

## 范围外

- 自动检测系统语言（后续版本）
- 其他语言支持（日语、韩语等）
- RTL 语言支持
- 语言包动态加载

## 用户流程

### 流程 1：切换语言

```
用户打开设置页面
  → 选择「语言设置」
  → 从下拉菜单选择语言（中文/English）
  → 界面文本实时切换
  → 语言选择自动保存
```

### 流程 2：启动加载语言

```
应用启动 → main.ts 初始化
  → 读取 electron-store 中的语言配置
  → 初始化 vue-i18n 并设置语言
  → 加载对应语言包
  → 界面显示对应语言文本
```

## 页面与模块设计

### vue-i18n 配置（main.ts）

```typescript
import { createI18n } from 'vue-i18n';
import zhCN from './locales/zh-CN.json';
import enUS from './locales/en-US.json';

const i18n = createI18n({
  legacy: false,
  locale: storedLocale || 'zh-CN',
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  }
});

app.use(i18n);
```

### 语言包结构

```json
{
  "app": { "title": "..." },
  "nav": { "dashboard": "...", ... },
  "common": { "save": "...", ... },
  "danmaku": { ... },
  "effect": { ... },
  "ai": { ... },
  "dashboard": { ... },
  "blacklist": { ... },
  "connection": { ... },
  "settings": { ... },
  "logs": { ... },
  "plugins": { ... },
  "gift": { ... },
  "recording": { ... },
  "update": { ... }
}
```

### 设置页面语言切换

在 Settings.vue 中添加语言切换：

```vue
<el-form-item :label="$t('settings.language')">
  <el-select v-model="currentLocale" @change="handleLocaleChange">
    <el-option label="中文" value="zh-CN" />
    <el-option label="English" value="en-US" />
  </el-select>
</el-form-item>
```

### 语言配置持久化

在 stores/config.ts 中添加语言配置：

```typescript
interface Config {
  // ... 现有配置
  locale: 'zh-CN' | 'en-US';
}
```

## API 与数据契约

### vue-i18n API

```typescript
// 在组件中使用
const { t } = useI18n();

// 模板中使用
{{ $t('key') }}

// 带参数
{{ $t('dashboard.giftMessage', { giftName: '小心心', count: 5 }) }}
```

### 语言配置存储

```typescript
// 读取语言配置
const locale = await window.electron.ipcRenderer.invoke('db:getConfig', 'locale');

// 保存语言配置
await window.electron.ipcRenderer.invoke('db:setConfig', { locale: 'en-US' });
```

## 边界情况

1. **语言包缺失 key**: 使用 fallbackLocale（zh-CN）的对应文本
2. **语言包格式错误**: 启动时验证，错误时使用默认语言
3. **存储读取失败**: 使用默认语言（zh-CN）
4. **快速切换语言**: 使用防抖避免频繁切换
5. **Element Plus 语言**: 同步配置 Element Plus 的 locale

## 验收标准

| 编号 | 验收条件 | 可测试性 |
|------|----------|----------|
| AC-01 | vue-i18n 框架集成完成 | 代码检查 |
| AC-02 | 中文语言包覆盖所有页面 | 手动验证 |
| AC-03 | 英文语言包覆盖所有页面 | 手动验证 |
| AC-04 | 设置页面可切换语言 | 手动验证 |
| AC-05 | 语言切换后界面文本实时更新 | 手动验证 |
| AC-06 | 语言选择持久化到 electron-store | 自动化测试 |
| AC-07 | 启动时加载用户语言偏好 | 手动验证 |
| AC-08 | 语言包缺失 key 时使用 fallback | 自动化测试 |
| AC-09 | Element Plus 组件语言同步 | 手动验证 |
| AC-10 | 新增功能预留 i18n 接口 | 代码检查 |

## 风险

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 语言包不完整 | 部分文本显示中文 | 定期检查补充 |
| i18n key 命名不一致 | 维护困难 | 制定命名规范 |
| Element Plus 语言不同步 | 组件显示不一致 | 同步配置 |
| 性能影响 | 切换卡顿 | 使用懒加载 |
