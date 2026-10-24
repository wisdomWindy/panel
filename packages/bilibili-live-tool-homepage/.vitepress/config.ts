import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Bilibili 直播助手',
  description: '智能弹幕与互动工具 - 提升直播互动体验',
  base: '/panel/',
  head: [
    ['link', { rel: 'icon', href: '/panel/favicon.ico' }]
  ],
  themeConfig: {
    logo: '/logo.svg',
    nav: [
      { text: '首页', link: '/' },
      { text: '下载', link: '/download/' },
      { text: '功能介绍', link: '/features/' },
      { text: '使用指南', link: '/guide/' },
      { text: '更新日志', link: '/changelog' }
    ],
    sidebar: {
      '/features/': [
        {
          text: '功能介绍',
          items: [
            { text: '概览', link: '/features/' },
            { text: '弹幕管理', link: '/features/danmaku' },
            { text: '礼物音效', link: '/features/gift-sound' },
            { text: '录制功能', link: '/features/recording' },
            { text: 'AI 对话', link: '/features/ai' }
          ]
        }
      ],
      '/guide/': [
        {
          text: '使用指南',
          items: [
            { text: '快速开始', link: '/guide/' },
            { text: '安装配置', link: '/guide/installation' },
            { text: '连接直播间', link: '/guide/connection' },
            { text: '功能设置', link: '/guide/settings' },
            { text: '常见问题', link: '/guide/faq' }
          ]
        }
      ]
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/wisdomWindy/panel' }
    ],
    footer: {
      message: '基于 MIT 许可发布',
      copyright: 'Copyright © 2026 Bilibili 直播助手'
    },
    search: {
      provider: 'local'
    }
  }
})
