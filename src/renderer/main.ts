import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import ElementPlus from 'element-plus';
import zhCn from 'element-plus/dist/locale/zh-cn.mjs';
import en from 'element-plus/dist/locale/en.mjs';
import './styles/tailwind.css';
import 'element-plus/dist/index.css';
import router from './router';
import { pinia } from './stores';
import App from '@/App.vue';
import zhCN from './locales/zh-CN.json';
import enUS from './locales/en-US.json';

// 获取存储的语言配置，默认中文
const storedLocale = localStorage.getItem('locale') || 'zh-CN';

const i18n = createI18n({
  legacy: false,
  locale: storedLocale,
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  }
});

// Element Plus locale 配置
const elementPlusLocales: Record<string, any> = {
  'zh-CN': zhCn,
  'en-US': en
};

const app = createApp(App);

app.use(pinia);
app.use(router);
app.use(i18n);
app.use(ElementPlus, {
  locale: elementPlusLocales[storedLocale] || zhCn
});

app.mount('#app');
