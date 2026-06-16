# 更新日志功能 - 功能规格

## 背景与目标

Bilibili 直播助手当前缺少应用内的版本更新日志查看能力。用户需要离开应用才能了解产品迭代内容。本功能旨在：

1. 在应用内提供完整的更新日志查看页面
2. 应用启动时自动检测新版本（可在设置中关闭）
3. 版本变更后首次启动展示"新版本特性"弹窗
4. 检测到新版本时在状态栏显示提示图标

## 范围内

- 更新日志查看页面（`UpdateLog.vue`）
- 新版本特性弹窗（`WhatsNewModal.vue`）
- 更新检测服务（`update-service.ts`）
- 更新状态管理（`stores/update.ts`）
- 设置页面新增更新检查开关
- 状态栏更新提示图标
- CHANGELOG.md 解析
- 版本号比较逻辑（Semantic Versioning）

## 范围外

- 应用内自动安装更新（仅跳转下载页）
- 增量更新机制
- 更新包下载进度显示
- 多语言更新日志
- 更新日志编辑功能

## 用户流程

### 流程 1：主动查看更新日志

```
用户点击菜单「帮助 → 更新日志」
  → 路由跳转到 /update-log
  → 页面加载并解析 CHANGELOG.md
  → 渲染版本列表（时间倒序）
  → 同时异步检查最新版本
  → 显示当前版本和最新版本信息
```

### 流程 2：新版本特性弹窗

```
应用启动 → main.ts 初始化更新模块
  → 读取 localStorage 中的 lastWhatsNewVersion
  → 比较当前版本与 lastWhatsNewVersion
  → 版本不同 → 弹出 WhatsNewModal
  → 用户点击「知道了」→ 关闭弹窗
  → 用户勾选「不再提醒此版本」→ 写入 localStorage
  → 记录当前版本到 lastWhatsNewVersion
```

### 流程 3：新版本检测与提示

```
应用启动 → 异步调用 updateService.checkForUpdates()
  → 请求 GitHub Releases API
  → 比较远程版本与本地版本
  → 远程版本更新 → 更新 Pinia store 状态
  → App.vue 状态栏显示红点
  → 用户点击红点 → 跳转 /update-log
  → 网络不可用 → 静默跳过，不显示错误
```

## 页面与模块设计

### UpdateLog.vue

**路由**: `/update-log`
**菜单入口**: 侧边栏新增「更新日志」菜单项，或「帮助」子菜单

**页面结构**:
- 头部区域：当前版本号 + 最新版本号 + 前往下载按钮
- 内容区域：版本卡片列表，每个卡片包含：
  - 版本号（el-tag 样式）
  - 发布日期
  - 变更分类列表（新功能/改进/修复/破坏性变更）

**数据来源**:
- 当前版本：`package.json` 的 `version` 字段
- 更新日志：项目根目录 `CHANGELOG.md`
- 最新版本：GitHub Releases API（异步）

### WhatsNewModal.vue

**触发条件**: 应用启动时，当前版本 !== localStorage.getItem('lastWhatsNewVersion')

**Props**:
- `visible`: boolean - 控制弹窗显示
- `version`: string - 当前版本号
- `changelog`: string - 当前版本的更新内容

**Events**:
- `close`: 用户点击「知道了」或关闭按钮
- `dontRemind`: 用户勾选「不再提醒此版本」

**持久化**:
- `localStorage.whatsNewDontRemind`: string[] - 不再提醒的版本列表
- `localStorage.lastWhatsNewVersion`: string - 最后一次展示的版本

### update-service.ts

**职责**:
- 解析 CHANGELOG.md 为结构化数据
- 检查 GitHub Releases 获取最新版本
- 比较版本号（Semantic Versioning）

**接口**:
```typescript
interface UpdateService {
  // 解析 CHANGELOG.md
  parseChangelog(): Promise<ChangelogEntry[]>;
  
  // 检查更新
  checkForUpdates(): Promise<UpdateCheckResult>;
  
  // 比较版本号
  compareVersions(current: string, latest: string): number;
}

interface ChangelogEntry {
  version: string;
  date: string;
  changes: {
    type: 'added' | 'changed' | 'fixed' | 'removed' | 'security';
    items: string[];
  }[];
}

interface UpdateCheckResult {
  hasUpdate: boolean;
  currentVersion: string;
  latestVersion: string;
  releaseUrl: string;
  publishedAt: string;
}
```

### stores/update.ts

**职责**: 管理更新相关的状态

**State**:
```typescript
interface UpdateState {
  currentVersion: string;
  latestVersion: string | null;
  hasUpdate: boolean;
  releaseUrl: string;
  isChecking: boolean;
  lastCheckTime: Date | null;
  changelog: ChangelogEntry[];
}
```

**Actions**:
- `checkForUpdates()`: 触发更新检查
- `parseChangelog()`: 解析更新日志
- `setCurrentVersion(version)`: 设置当前版本

## API 与数据契约

### CHANGELOG.md 格式

遵循 [Keep a Changelog](https://keepachangelog.com/) 规范：

```markdown
# Changelog

## [0.2.0] - 2026-06-20

### Added
- 新增更新日志查看功能
- 新增版本自动检测

### Changed
- 优化弹幕发送性能

### Fixed
- 修复连接断开后无法重连的问题

## [0.1.0] - 2026-06-01

### Added
- 初始发布版本
- 支持弹幕自动发送
- 支持礼物感谢
```

### GitHub Releases API

**端点**: `GET https://api.github.com/repos/{owner}/{repo}/releases/latest`

**响应关键字段**:
```json
{
  "tag_name": "v0.2.0",
  "published_at": "2026-06-20T10:00:00Z",
  "html_url": "https://github.com/{owner}/{repo}/releases/tag/v0.2.0",
  "body": "更新日志内容..."
}
```

### 版本号比较规则

遵循 Semantic Versioning (MAJOR.MINOR.PATCH):
- MAJOR: 不兼容的 API 修改
- MINOR: 向下兼容的功能性新增
- PATCH: 向下兼容的问题修正

比较逻辑：
```typescript
function compareVersions(v1: string, v2: string): number {
  const parts1 = v1.replace('v', '').split('.').map(Number);
  const parts2 = v2.replace('v', '').split('.').map(Number);
  
  for (let i = 0; i < 3; i++) {
    if (parts1[i] > parts2[i]) return 1;
    if (parts1[i] < parts2[i]) return -1;
  }
  return 0;
}
```

## 边界情况

1. **CHANGELOG.md 不存在或为空**: 显示"暂无更新日志"占位文本
2. **CHANGELOG.md 格式不符合规范**: 尝试宽松解析，失败时显示原始 Markdown 内容
3. **网络不可用**: 更新检查静默跳过，不显示错误提示
4. **GitHub API 限流**: 静默跳过，下次启动重试
5. **版本号格式错误**: 降级为字符串比较
6. **localStorage 不可用**: 弹窗每次启动都显示（降级行为）
7. **多个版本未查看**: 只展示当前版本的更新日志，不累积

## 验收标准

| 编号 | 验收条件 | 可测试性 |
|------|----------|----------|
| AC-01 | 菜单栏「帮助 → 更新日志」可打开更新日志页面 | 手动点击验证 |
| AC-02 | 设置页面「关于 → 查看更新日志」可打开更新日志页面 | 手动点击验证 |
| AC-03 | 更新日志页面按时间倒序展示版本记录 | 自动化测试 |
| AC-04 | 每个版本记录包含版本号、日期和变更分类 | 自动化测试 |
| AC-05 | 启动时自动检查更新，不阻塞主界面加载（< 3秒） | 性能测试 |
| AC-06 | 设置中可开启/关闭自动检查更新 | 手动验证 |
| AC-07 | 版本变更后首次启动展示"新版本特性"弹窗 | 自动化测试 |
| AC-08 | 弹窗「知道了」按钮可关闭弹窗 | 自动化测试 |
| AC-09 | 弹窗「不再提醒此版本」勾选后该版本不再弹出 | 自动化测试 |
| AC-10 | 检测到新版本时状态栏显示红点提示 | 自动化测试 |
| AC-11 | 点击红点跳转到更新日志页面 | 手动验证 |
| AC-12 | 点击「前往下载」跳转到对应版本下载页 | 手动验证 |
| AC-13 | 离线状态下无错误提示或弹窗 | 自动化测试 |
| AC-14 | CHANGELOG.md 解析失败时显示友好提示 | 自动化测试 |
| AC-15 | 版本号比较正确处理 Semantic Versioning | 单元测试 |

## 风险

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| CHANGELOG.md 格式不一致 | 解析失败，页面无内容 | 宽松解析 + 降级显示原始 Markdown |
| GitHub API 限流 | 无法检测更新 | 静默跳过，缓存上次检查结果 |
| 版本号格式不规范 | 比较逻辑错误 | 降级为字符串比较 |
| localStorage 被清除 | 弹窗重复显示 | 接受降级行为，每次启动都显示 |
| 网络代理环境 | GitHub API 请求失败 | 静默跳过，不阻塞用户操作 |
