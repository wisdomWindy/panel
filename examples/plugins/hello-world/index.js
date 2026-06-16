/**
 * Hello World 示例插件
 * 展示插件开发的基本方法
 */

// 插件主模块
const HelloWorldPlugin = {
  // 插件元信息（从 manifest.json 读取）
  id: 'hello-world',
  name: 'Hello World 示例插件',
  version: '1.0.0',
  description: '一个简单的示例插件，展示插件开发的基本方法',
  author: 'Bilibili 直播助手团队',

  // 插件配置
  config: {
    greeting: '你好！欢迎来到直播间！',
    autoReply: true,
  },

  /**
   * 生命周期钩子：插件加载时调用
   */
  onLoad() {
    console.log('[Hello World] 插件已加载');
  },

  /**
   * 生命周期钩子：插件启用时调用
   */
  onEnable() {
    console.log('[Hello World] 插件已启用');

    // 注册弹幕事件监听
    this.cleanupDanmaku = onDanmaku((danmaku) => {
      console.log(`[Hello World] 收到弹幕: ${danmaku.content}`);

      // 自动回复逻辑
      if (this.config.autoReply) {
        // 检查是否是新用户（简化逻辑）
        if (danmaku.content.includes('新人') || danmaku.content.includes('第一次')) {
          sendDanmaku(this.config.greeting);
        }
      }
    });

    // 注册礼物事件监听
    this.cleanupGift = onGift((gift) => {
      console.log(`[Hello World] 收到礼物: ${gift.giftName} x${gift.count}`);

      // 感谢礼物
      if (gift.count >= 10) {
        sendDanmaku(`感谢 ${gift.username} 送的 ${gift.giftName} x${gift.count}！`);
      }
    });

    // 监听配置变更
    this.cleanupConfig = onConfigChange((key, value) => {
      console.log(`[Hello World] 配置变更: ${key} = ${value}`);
      this.config[key] = value;
    });

    log('info', 'Hello World 插件已启用');
  },

  /**
   * 生命周期钩子：插件禁用时调用
   */
  onDisable() {
    console.log('[Hello World] 插件已禁用');

    // 清理事件监听
    if (this.cleanupDanmaku) {
      this.cleanupDanmaku();
    }
    if (this.cleanupGift) {
      this.cleanupGift();
    }
    if (this.cleanupConfig) {
      this.cleanupConfig();
    }

    log('info', 'Hello World 插件已禁用');
  },

  /**
   * 生命周期钩子：插件卸载时调用
   */
  onUnload() {
    console.log('[Hello World] 插件已卸载');
    log('info', 'Hello World 插件已卸载');
  },

  /**
   * 弹幕事件处理
   */
  onDanmaku(danmaku) {
    // 这个方法会被 onEnable 中的监听器调用
    // 可以在这里添加更复杂的弹幕处理逻辑
  },

  /**
   * 礼物事件处理
   */
  onGift(gift) {
    // 这个方法会被 onEnable 中的监听器调用
    // 可以在这里添加更复杂的礼物处理逻辑
  },

  // 清理函数
  cleanupDanmaku: null,
  cleanupGift: null,
  cleanupConfig: null,
};

// 导出插件模块
module.exports = HelloWorldPlugin;
