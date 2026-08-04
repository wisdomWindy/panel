# 插件开发指南

本文档介绍如何为 Bilibili 直播助手开发插件。

## 目录

- [简介](#简介)
- [快速开始](#快速开始)
- [插件结构](#插件结构)
- [插件清单](#插件清单)
- [生命周期](#生命周期)
- [API 参考](#api-参考)
- [权限系统](#权限系统)
- [配置管理](#配置管理)
- [最佳实践](#最佳实践)
- [常见问题](#常见问题)

## 简介

插件系统允许第三方开发者扩展 Bilibili 直播助手的功能。通过插件，您可以：

- 监听和响应弹幕事件
- 监听和响应礼物事件
- 注册自定义特效
- 扩展用户界面
- 读写配置数据

## 快速开始

### 1. 创建插件目录

在应用的插件目录下创建一个新的文件夹，用于存放您的插件：

```
plugins/
└── my-plugin/
    ├── manifest.json
    └── index.js
```

### 2. 编写插件清单

创建 `manifest.json` 文件，定义插件的基本信息：

```json
{
  "id": "my-plugin",
  "name": "我的插件",
  "version": "1.0.0",
  "description": "这是一个示例插件",
  "author": "Your Name",
  "main": "index.js",
  "minAppVersion": "1.0.0",
  "permissions": ["danmaku", "gift"]
}
```

### 3. 编写插件代码

创建 `index.js` 文件，实现插件功能：

```javascript
const MyPlugin = {
  id: 'my-plugin',
  name: '我的插件',
  version: '1.0.0',
  description: '这是一个示例插件',
  author: 'Your Name',

  onLoad() {
    console.log('插件已加载');
  },

  onEnable() {
    console.log('插件已启用');
    
    // 监听弹幕
    onDanmaku((danmaku) => {
      console.log('收到弹幕:', danmaku.content);
    });
  },

  onDisable() {
    console.log('插件已禁用');
  },

  onUnload() {
    console.log('插件已卸载');
  },
};

module.exports = MyPlugin;
```

### 4. 安装插件

将插件文件夹复制到应用的插件目录，然后在应用中启用插件。

## 插件结构

一个完整的插件包含以下文件：

```
my-plugin/
├── manifest.json    # 插件清单（必需）
├── index.js         # 插件入口文件（必需）
├── styles/          # 样式文件（可选）
└── assets/          # 资源文件（可选）
```

### manifest.json

插件清单文件，定义插件的元信息、权限和配置。

### index.js

插件入口文件，导出插件主模块。插件主模块是一个对象，包含以下属性和方法：

- 属性：`id`、`name`、`version`、`description`、`author`
- 方法：`onLoad`、`onEnable`、`onDisable`、`onUnload`
- 事件处理：`onDanmaku`、`onGift`、`onCaptain`

## 插件清单

`manifest.json` 文件定义了插件的基本信息：

```json
{
  "id": "my-plugin",
  "name": "我的插件",
  "version": "1.0.0",
  "description": "这是一个示例插件",
  "author": "Your Name",
  "main": "index.js",
  "minAppVersion": "1.0.0",
  "permissions": ["danmaku", "gift", "config"],
  "configSchema": {
    "greeting": {
      "type": "string",
      "default": "你好！",
      "description": "欢迎语"
    }
  }
}
```

### 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | string | 是 | 插件唯一标识，使用小写字母、数字和连字符 |
| name | string | 是 | 插件显示名称 |
| version | string | 是 | 插件版本号，建议使用 Semantic Versioning |
| description | string | 是 | 插件描述 |
| author | string | 是 | 插件作者 |
| main | string | 是 | 插件入口文件路径 |
| minAppVersion | string | 是 | 支持的最低应用版本 |
| permissions | array | 否 | 所需权限列表 |
| configSchema | object | 否 | 配置项定义 |

## 生命周期

插件支持以下生命周期钩子：

### onLoad

插件加载时调用，用于初始化插件资源。

```javascript
onLoad() {
  console.log('插件已加载');
  // 初始化资源
}
```

### onEnable

插件启用时调用，用于注册事件监听器、启动定时器等。

```javascript
onEnable() {
  console.log('插件已启用');
  // 注册事件监听器
  // 启动定时器
}
```

### onDisable

插件禁用时调用，用于清理事件监听器、停止定时器等。

```javascript
onDisable() {
  console.log('插件已禁用');
  // 清理事件监听器
  // 停止定时器
}
```

### onUnload

插件卸载时调用，用于释放所有资源。

```javascript
onUnload() {
  console.log('插件已卸载');
  // 释放所有资源
}
```

## API 参考

### 弹幕相关

#### onDanmaku(callback)

监听弹幕事件。

- `callback`: 回调函数，接收弹幕事件对象
- 返回值：取消监听的函数

```javascript
const cleanup = onDanmaku((danmaku) => {
  console.log('用户:', danmaku.username);
  console.log('内容:', danmaku.content);
  console.log('时间:', danmaku.timestamp);
});

// 取消监听
cleanup();
```

弹幕事件对象属性：

| 属性 | 类型 | 说明 |
|------|------|------|
| id | string | 弹幕 ID |
| userId | string | 用户 ID |
| username | string | 用户名 |
| content | string | 弹幕内容 |
| timestamp | number | 时间戳 |
| userType | string | 用户类型：normal、captain、admin、owner |
| badge | object | 徽章信息 |
| medal | object | 勋章信息 |

### 礼物相关

#### onGift(callback)

监听礼物事件。

- `callback`: 回调函数，接收礼物事件对象
- 返回值：取消监听的函数

```javascript
const cleanup = onGift((gift) => {
  console.log('用户:', gift.username);
  console.log('礼物:', gift.giftName);
  console.log('数量:', gift.count);
});

// 取消监听
cleanup();
```

礼物事件对象属性：

| 属性 | 类型 | 说明 |
|------|------|------|
| id | string | 礼物 ID |
| giftId | string | 礼物类型 ID |
| giftName | string | 礼物名称 |
| userId | string | 用户 ID |
| username | string | 用户名 |
| count | number | 数量 |
| price | number | 价格 |
| timestamp | number | 时间戳 |
| isCombo | boolean | 是否连击 |
| comboId | string | 连击 ID |

### 舰长相关

#### onCaptain(callback)

监听舰长事件。

- `callback`: 回调函数，接收舰长事件对象
- 返回值：取消监听的函数

```javascript
const cleanup = onCaptain((captain) => {
  console.log('用户:', captain.username);
  console.log('等级:', captain.level);
  console.log('首次:', captain.isFirstTime);
});

// 取消监听
cleanup();
```

舰长事件对象属性：

| 属性 | 类型 | 说明 |
|------|------|------|
| userId | string | 用户 ID |
| username | string | 用户名 |
| level | number | 等级：1=舰长，2=提督，3=总督 |
| giftName | string | 礼物名称 |
| giftId | string | 礼物 ID |
| count | number | 数量 |
| timestamp | number | 时间戳 |
| isFirstTime | boolean | 是否首次上舰 |
| isRenewal | boolean | 是否续费 |

### 配置相关

#### getConfig(key)

获取配置值。

- `key`: 配置键名
- 返回值：配置值

```javascript
const greeting = getConfig('greeting');
console.log(greeting); // '你好！'
```

#### setConfig(key, value)

设置配置值。

- `key`: 配置键名
- `value`: 配置值

```javascript
setConfig('greeting', '新的欢迎语');
```

#### onConfigChange(callback)

监听配置变更。

- `callback`: 回调函数，接收键名和值
- 返回值：取消监听的函数

```javascript
const cleanup = onConfigChange((key, value) => {
  console.log('配置变更:', key, value);
});

// 取消监听
cleanup();
```

### 特效相关

#### registerEffect(effect)

注册自定义特效。

- `effect`: 特效定义对象
- 返回值：取消注册的函数

```javascript
const cleanup = registerEffect({
  id: 'my-effect',
  name: '我的特效',
  type: 'custom',
  template: {
    text: '感谢 {username}！',
    animation: 'fade',
    colors: {
      primary: '#FFD700',
      secondary: '#FFFFFF',
    },
    duration: 5000,
    fontSize: 24,
  },
});

// 取消注册
cleanup();
```

### UI 相关

#### registerMenuItem(menu)

注册菜单项。

- `menu`: 菜单项定义对象
- 返回值：取消注册的函数

```javascript
const cleanup = registerMenuItem({
  id: 'my-menu',
  label: '我的菜单',
  icon: 'Setting',
  path: '/my-plugin',
  order: 100,
});

// 取消注册
cleanup();
```

#### registerSettingPanel(panel)

注册设置面板。

- `panel`: 设置面板定义对象
- 返回值：取消注册的函数

```javascript
const cleanup = registerSettingPanel({
  id: 'my-settings',
  label: '我的设置',
  icon: 'Setting',
  component: MySettingsComponent,
  order: 100,
});

// 取消注册
cleanup();
```

### 日志相关

#### log(level, message)

记录日志。

- `level`: 日志级别：info、warn、error
- `message`: 日志消息

```javascript
log('info', '信息日志');
log('warn', '警告日志');
log('error', '错误日志');
```

## 权限系统

插件需要声明所需权限才能访问相应的 API。权限在 `manifest.json` 中的 `permissions` 字段定义。

### 权限列表

| 权限 | 说明 | 可用 API |
|------|------|----------|
| danmaku | 弹幕监听 | onDanmaku |
| gift | 礼物监听 | onGift |
| captain | 舰长监听 | onCaptain |
| config | 配置读写 | getConfig、setConfig、onConfigChange |
| effect | 特效注册 | registerEffect |
| ui | UI 扩展 | registerMenuItem、registerSettingPanel |

### 权限检查

插件在调用 API 时会自动检查权限。如果插件未声明相应权限，API 调用将抛出错误。

## 配置管理

插件可以定义自己的配置项，用户可以在应用中配置这些值。

### 定义配置项

在 `manifest.json` 中的 `configSchema` 字段定义配置项：

```json
{
  "configSchema": {
    "greeting": {
      "type": "string",
      "default": "你好！",
      "description": "欢迎语"
    }
  }
}
```

### 配置项类型

| 类型 | 说明 | 属性 |
|------|------|------|
| string | 字符串 | default、description |
| number | 数字 | default、description、min、max |
| boolean | 布尔值 | default、description |
| select | 下拉选择 | default、description、options |
| color | 颜色选择器 | default、description |

### 读写配置

```javascript
// 获取配置
const greeting = getConfig('greeting');

// 设置配置
setConfig('greeting', '新的欢迎语');

// 监听配置变更
onConfigChange((key, value) => {
  console.log('配置变更:', key, value);
});
```

## 最佳实践

### 1. 资源清理

在 `onDisable` 和 `onUnload` 中清理所有资源：

```javascript
onEnable() {
  this.cleanup = onDanmaku((danmaku) => {
    // 处理弹幕
  });
},

onDisable() {
  if (this.cleanup) {
    this.cleanup();
  }
},
```

### 3. 配置监听

监听配置变更，实时更新插件行为：

```javascript
onEnable() {
  this.config = getConfig('greeting');
  
  this.cleanupConfig = onConfigChange((key, value) => {
    if (key === 'greeting') {
      this.config = value;
    }
  });
},
```

### 4. 日志记录

使用 `log()` 函数记录重要信息，便于调试：

```javascript
log('info', '插件已启用');
log('warn', '配置缺失，使用默认值');
log('error', `操作失败: ${error.message}`);
```

### 5. 版本兼容

检查应用版本，确保兼容性：

```javascript
onLoad() {
  // 检查应用版本
  const appVersion = getAppVersion();
  if (compareVersions(appVersion, '1.0.0') < 0) {
    log('warn', '当前应用版本过低，部分功能可能不可用');
  }
},
```

## 常见问题

### Q: 插件无法加载怎么办？

A: 检查以下几点：
1. `manifest.json` 文件格式是否正确
2. 必填字段是否完整
3. `main` 字段指向的入口文件是否存在
4. 插件目录结构是否正确

### Q: 插件 API 调用失败怎么办？

A: 检查以下几点：
1. 插件是否声明了相应权限
2. API 调用参数是否正确
3. 查看日志是否有错误信息

### Q: 如何调试插件？

A: 使用 `log()` 函数记录日志，日志会显示在应用的日志监控页面。也可以在浏览器开发者工具中查看控制台输出。

### Q: 插件如何更新？

A: 下载新版本插件文件，覆盖原有插件目录，然后重新启用插件。建议在更新前先禁用插件。

### Q: 插件配置丢失怎么办？

A: 插件配置存储在应用的统一配置系统中。如果配置丢失，可以尝试：
1. 重新安装插件
2. 手动设置配置
3. 检查应用数据目录是否完整

### Q: 插件与其他插件冲突怎么办？

A: 如果多个插件注册了相同的菜单项或特效，后加载的插件会覆盖先加载的。建议：
1. 使用唯一的 ID
2. 检查是否有冲突
3. 联系插件开发者协调

### Q: 如何卸载插件？

A: 在应用的「插件管理」页面，找到要卸载的插件，点击「卸载」按钮。卸载会删除插件文件和配置。

## 示例插件

参考 `examples/plugins/hello-world/` 目录下的示例插件，了解插件开发的完整示例。

## API 文档

详细的 API 文档请参考 `src/renderer/plugins/types.ts` 文件中的类型定义。

## 获取帮助

如果您在插件开发过程中遇到问题，可以：

1. 查看本文档的常见问题部分
2. 参考示例插件代码
3. 在 GitHub 上提交 Issue
4. 加入开发者社区讨论
