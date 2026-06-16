# 插件系统规格说明

## 背景与目标

Bilibili 直播助手需要支持第三方插件扩展功能，以建立插件生态，满足不同主播的个性化需求。插件系统需要提供标准化的接口、安全的隔离机制、完善的生命周期管理，使第三方开发者能够轻松开发插件，用户能够方便地管理插件。

## 范围内

### 核心功能
1. **插件接口规范**：定义插件结构、生命周期钩子、API 接口
2. **插件加载器**：从本地目录加载插件，支持 JavaScript/TypeScript 模块
3. **插件管理器**：管理插件的安装、启用、禁用、卸载
4. **插件 API 暂露**：暂露核心 API 给插件使用（弹幕、礼物、特效、配置等）
5. **插件沙箱**：隔离插件执行环境，防止插件异常影响主应用
6. **插件配置管理**：插件可定义自己的配置项，用户可在界面中配置
7. **插件管理界面**：设置页面新增「插件管理」tab，展示已安装插件列表
8. **插件开发文档**：提供插件开发指南和 API 文档
9. **示例插件**：提供至少一个示例插件，展示插件开发方法

### 插件能力
- 监听弹幕事件
- 监听礼物事件
- 监听舰长事件
- 发送弹幕
- 读取/修改配置
- 注册自定义特效
- 注册菜单项
- 注册设置页面

## 范围外

1. **插件市场**：本期不实现在线插件市场，仅支持本地安装
2. **插件自动更新**：本期不实现插件自动更新，需手动更新
3. **插件付费**：本期不实现插件付费功能
4. **插件签名验证**：本期不实现插件签名验证，后续版本添加
5. **多进程隔离**：本期不实现多进程隔离，使用渲染进程内沙箱

## 用户流程

### 插件安装流程
1. 用户下载插件包（.zip 或 .js 文件）
2. 用户打开设置页面 → 插件管理 tab
3. 点击「安装插件」按钮
4. 选择插件文件或拖拽到指定区域
5. 系统验证插件格式和兼容性
6. 安装成功，插件出现在已安装列表
7. 用户启用插件

### 插件使用流程
1. 插件启用后，自动注册其功能
2. 用户在对应功能区域看到插件添加的菜单项/特效/配置
3. 插件监听事件并执行相应逻辑
4. 插件异常时显示错误提示，不影响主应用

### 插件管理流程
1. 用户打开设置页面 → 插件管理 tab
2. 查看已安装插件列表
3. 切换插件启用/禁用状态
4. 查看插件详情（版本、作者、描述）
5. 配置插件设置项
6. 卸载不需要的插件

## 页面与模块设计

### 插件管理页面
- **路径**：`/settings` → 插件管理 tab
- **组件**：`PluginManager.vue`
- **功能**：
  - 已安装插件列表（表格形式）
  - 插件启用/禁用开关
  - 插件详情查看
  - 插件配置入口
  - 安装插件按钮
  - 卸载插件按钮

### 插件目录结构
```
plugins/
├── plugin-manager.ts      # 插件管理器
├── plugin-loader.ts       # 插件加载器
├── plugin-api.ts          # 插件 API 暂露
├── plugin-sandbox.ts      # 插件沙箱
├── types.ts               # 类型定义
└── index.ts               # 入口文件
```

### 插件包结构
```
my-plugin/
├── manifest.json          # 插件清单
├── index.js               # 插件入口
├── styles/                # 样式文件（可选）
└── assets/                # 资源文件（可选）
```

### 插件清单格式 (manifest.json)
```json
{
  "id": "my-plugin",
  "name": "我的插件",
  "version": "1.0.0",
  "description": "插件描述",
  "author": "作者名",
  "main": "index.js",
  "minAppVersion": "1.0.0",
  "permissions": ["danmaku", "gift", "config"],
  "configSchema": {
    "enabled": {
      "type": "boolean",
      "default": true,
      "description": "是否启用"
    }
  }
}
```

## API 与数据契约

### 插件接口定义

```typescript
interface Plugin {
  // 插件元信息
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;

  // 生命周期钩子
  onLoad?: () => Promise<void> | void;
  onEnable?: () => Promise<void> | void;
  onDisable?: () => Promise<void> | void;
  onUnload?: () => Promise<void> | void;

  // 事件监听
  onDanmaku?: (danmaku: DanmakuEvent) => void;
  onGift?: (gift: GiftEvent) => void;
  onCaptain?: (captain: CaptainEvent) => void;

  // 配置
  config?: Record<string, any>;
  configSchema?: Record<string, ConfigField>;
}
```

### 插件 API 暂露

```typescript
interface PluginAPI {
  // 弹幕相关
  sendDanmaku(content: string, options?: DanmakuOptions): Promise<void>;
  onDanmaku(callback: (danmaku: DanmakuEvent) => void): () => void;

  // 礼物相关
  onGift(callback: (gift: GiftEvent) => void): () => void;

  // 配置相关
  getConfig<T>(key: string): T;
  setConfig<T>(key: string, value: T): void;
  onConfigChange(callback: (key: string, value: any) => void): () => void;

  // 特效相关
  registerEffect(effect: EffectDefinition): () => void;

  // UI 相关
  registerMenuItem(menu: MenuItemDefinition): () => void;
  registerSettingPanel(panel: SettingPanelDefinition): () => void;

  // 日志
  log(level: 'info' | 'warn' | 'error', message: string): void;
}
```

### 插件清单数据结构

```typescript
interface PluginManifest {
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  main: string;
  minAppVersion: string;
  permissions: PluginPermission[];
  configSchema?: Record<string, ConfigField>;
}

type PluginPermission = 
  | 'danmaku'    // 弹幕读写
  | 'gift'       // 礼物监听
  | 'captain'    // 舰长监听
  | 'config'     // 配置读写
  | 'effect'     // 特效注册
  | 'ui';        // UI 扩展

interface ConfigField {
  type: 'string' | 'number' | 'boolean' | 'select' | 'color';
  default: any;
  description: string;
  options?: { label: string; value: any }[];
  min?: number;
  max?: number;
}
```

## 边缘情况

1. **插件格式错误**：manifest.json 格式不正确或缺少必填字段
   - 处理：显示错误提示，拒绝安装

2. **插件版本不兼容**：插件要求的最低应用版本高于当前版本
   - 处理：显示警告，允许用户选择是否强制安装

3. **插件权限不足**：插件请求的权限未在 manifest 中声明
   - 处理：拒绝 API 调用，记录错误日志

4. **插件异常崩溃**：插件代码抛出未捕获异常
   - 处理：捕获异常，显示错误提示，禁用插件，不影响主应用

5. **插件冲突**：多个插件注册相同的菜单项或特效
   - 处理：按加载顺序显示，后加载的插件覆盖先加载的

6. **插件资源加载失败**：插件的样式或资源文件不存在
   - 处理：跳过缺失资源，记录警告日志

7. **插件配置损坏**：插件配置文件格式错误或损坏
   - 处理：使用默认配置，提示用户重新配置

8. **插件卸载残留**：插件卸载后配置或数据未清理
   - 处理：卸载时清理插件配置和数据

## 验收标准

### 插件加载
- [ ] 能够从 `plugins/` 目录加载插件
- [ ] 能够正确解析 manifest.json
- [ ] 能够验证插件格式和兼容性
- [ ] 加载失败时显示错误提示

### 插件生命周期
- [ ] 插件安装后出现在已安装列表
- [ ] 插件启用时调用 onEnable 钩子
- [ ] 插件禁用时调用 onDisable 钩子
- [ ] 插件卸载时调用 onUnload 钩子
- [ ] 插件状态变更后持久化

### 插件 API
- [ ] 插件能够发送弹幕
- [ ] 插件能够监听弹幕事件
- [ ] 插件能够监听礼物事件
- [ ] 插件能够读取配置
- [ ] 插件能够修改配置
- [ ] 插件能够注册自定义特效
- [ ] 插件能够注册菜单项

### 插件沙箱
- [ ] 插件异常不影响主应用稳定性
- [ ] 插件无法访问未授权的 API
- [ ] 插件异常时显示错误提示

### 插件管理界面
- [ ] 设置页面显示「插件管理」tab
- [ ] 已安装插件列表正确显示
- [ ] 插件启用/禁用开关正常工作
- [ ] 插件详情显示正确
- [ ] 安装插件功能正常
- [ ] 卸载插件功能正常

### 插件开发文档
- [ ] 文档包含插件开发指南
- [ ] 文档包含 API 参考
- [ ] 文档包含示例代码
- [ ] 文档包含常见问题解答

## 风险

1. **安全风险**：恶意插件可能访问用户数据或执行危险操作
   - 缓解：实现权限控制和沙箱隔离

2. **稳定性风险**：插件异常可能导致主应用崩溃
   - 缓解：实现异常捕获和插件隔离

3. **性能风险**：大量插件可能影响应用性能
   - 缓解：实现插件懒加载和性能监控

4. **兼容性风险**：插件可能与主应用版本不兼容
   - 缓解：实现版本检查和兼容性验证

5. **维护风险**：插件 API 变更可能导致插件失效
   - 缓解：保持 API 向后兼容，提供迁移指南
