/**
 * Pinia 实例
 * 在此文件顶层创建并导出 pinia 实例，确保在任何 store composable 被引用之前 Pinia 已初始化。
 */

import { createPinia } from 'pinia';

export const pinia = createPinia();
