import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import ElementPlus from 'element-plus';
import './styles/tailwind.css';
import 'element-plus/dist/index.css';
import 'element-plus/theme-chalk/dark/css-vars.css';
import router from './router';
import { pinia } from './stores';
import App from '@/App.vue';
import zhCN from './locales/zh-CN.json';
import enUS from './locales/en-US.json';
import { getStoredLocale, localeState } from './i18n-runtime';
import './platform/desktop/build-channel';
import EffectWindowRoot from './views/captain-effect/EffectWindowRoot.vue';
import { memberChannel } from '@member-channel';

const prefersDark = window.matchMedia?.('(prefers-color-scheme: dark)').matches ?? false;
document.documentElement.classList.toggle('dark', prefersDark);
document.documentElement.style.colorScheme = prefersDark ? 'dark' : 'light';

const storedLocale = getStoredLocale();

const i18n = createI18n({
  legacy: false,
  locale: storedLocale,
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': {
      ...zhCN,
      shell: {
        ...zhCN.shell,
        navigation: { ...zhCN.shell.navigation, ...memberChannel.navigationMessages['zh-CN'] }
      }
    },
    'en-US': {
      ...enUS,
      shell: {
        ...enUS.shell,
        navigation: { ...enUS.shell.navigation, ...memberChannel.navigationMessages['en-US'] }
      }
    }
  }
});

const isEffectSurface = new URLSearchParams(window.location.search).get('surface') === 'effect';

if (isEffectSurface) {
  createApp(EffectWindowRoot).mount('#app');
} else {
  const app = createApp(App);
  app.use(pinia);
  app.use(router);
  app.use(i18n);
  app.use(ElementPlus);
  localeState.value = storedLocale;
  (i18n.global.locale as { value: string }).value = storedLocale;
  app.mount('#app');
}
