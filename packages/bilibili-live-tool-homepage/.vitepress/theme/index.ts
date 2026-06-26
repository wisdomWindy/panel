import DefaultTheme from 'vitepress/theme';
import './style.css';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import Icon from './Icon.vue';

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
      app.component(key, component);
    }
    app.component('Icon', Icon);
  }
};
