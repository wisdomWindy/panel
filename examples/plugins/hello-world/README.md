# Hello World 示例插件

这是一个简单的示例插件，展示插件开发的基本方法。

## 功能特性

- 监听弹幕事件
- 监听礼物事件
- 自动回复新用户
- 感谢大额礼物
- 支持配置自定义

## 安装方法

1. 将 `hello-world` 文件夹复制到应用的插件目录
2. 在应用中打开「设置」→「插件管理」
3. 找到「Hello World 示例插件」并启用

## 配置说明

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| greeting | string | 你好！欢迎来到直播间！ | 欢迎语 |
| autoReply | boolean | true | 是否自动回复 |

## 开发说明

### 插件结构

```
hello-world/
├── manifest.json    # 插件清单
├── index.js         # 插件入口
└── README.md        # 说明文档
```

### 生命周期

插件支持以下生命周期钩子：

- `onLoad`: 插件加载时调用
- `onEnable`: 插件启用时调用
- `onDisable`: 插件禁用时调用
- `onUnload`: 插件卸载时调用

### API 使用

插件可以使用以下 API：

#### 弹幕相关

```javascript
// 发送弹幕
sendDanmaku('你好！');

// 监听弹幕
const cleanup = onDanmaku((danmaku) => {
  console.log(danmaku.content);
});

// 取消监听
cleanup();
```

#### 礼物相关

```javascript
// 监听礼物
const cleanup = onGift((gift) => {
  console.log(gift.giftName);
});

// 取消监听
cleanup();
```

#### 配置相关

```javascript
// 获取配置
const value = getConfig('greeting');

// 设置配置
setConfig('greeting', '新的欢迎语');

// 监听配置变更
const cleanup = onConfigChange((key, value) => {
  console.log(key, value);
});

// 取消监听
cleanup();
```

#### 日志相关

```javascript
// 记录日志
log('info', '信息日志');
log('warn', '警告日志');
log('error', '错误日志');
```

## 权限说明

本插件需要以下权限：

- `danmaku`: 弹幕读写权限
- `gift`: 礼物监听权限
- `config`: 配置读写权限

## 注意事项

1. 插件代码运行在沙箱环境中，无法访问主应用的全局对象
2. 插件异常不会影响主应用的稳定性
3. 插件配置存储在应用的统一配置系统中
4. 插件卸载时会自动清理所有事件监听器

## 常见问题

### Q: 如何调试插件？

A: 在插件代码中使用 `log()` 函数记录日志，日志会显示在应用的日志监控页面。

### Q: 如何更新插件？

A: 下载新版本插件文件，覆盖原有插件目录，然后重新启用插件。

### Q: 插件无法启用怎么办？

A: 检查插件清单文件是否正确，查看日志是否有错误信息。

## 示例代码

更多示例代码请参考 `index.js` 文件中的注释。
