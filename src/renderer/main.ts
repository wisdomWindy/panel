import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import ElementPlus from 'element-plus';
import './styles/tailwind.css';
import 'element-plus/dist/index.css';
import router from './router';
import { pinia } from './stores';
import App from '@/App.vue';
import zhCN from './locales/zh-CN.json';
import enUS from './locales/en-US.json';
import { getStoredLocale, localeState } from './i18n-runtime';

const storedLocale = getStoredLocale();

const i18n = createI18n({
  legacy: false,
  locale: storedLocale,
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  }
});

const app = createApp(App);

app.use(pinia);
app.use(router);
app.use(i18n);
app.use(ElementPlus);

localeState.value = storedLocale;
(i18n.global.locale as { value: string }).value = storedLocale;

app.mount('#app');
