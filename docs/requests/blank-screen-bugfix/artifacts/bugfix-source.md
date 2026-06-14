# Bugfix 根因分析: 应用启动后界面空白

**分析日期**: 2026-06-12

---

## 根因分类: 时序问题

**代码位置**: `src/renderer/services/danmaku-service.ts:281`

### 根因分析

`danmaku-service.ts` 在第 281 行模块顶层立即执行 `export const danmakuService = new DanmakuService()`。

`DanmakuService` 构造函数中调用了 `useRoomStore()` 和 `useConfigStore()`（第 20-22 行），这两个 Pinia composables 要求运行时有活跃的 Pinia 实例。

ES 模块的加载顺序是自底向上的:
1. `danmaku-service.ts` 模块加载 → 执行 `new DanmakuService()` → 构造函数调用 `useRoomStore()` / `useConfigStore()`
2. 此时 `main.ts` 尚未执行，`createPinia()` + `app.use(pinia)` 还未调用
3. Pinia 抛出: `getActivePinia() was called but there was no active Pinia. Did you forget to install pinia?`
4. 模块加载链失败 → Vue 应用无法挂载到 `#app` → **空白界面**

### 影响评估
- **用户影响**: 所有用户，应用 100% 无法启动
- **数据影响**: 无
- **连锁影响**: 阻塞整个渲染进程

---

## 最小修复方案

**原则**: 将立即实例化的单例改为懒初始化（Lazy Initialization），确保首次实例化发生在 Pinia 已安装的 Vue 组件 setup 上下文中。

### 修改文件 1: `src/renderer/services/danmaku-service.ts`

**修改前** (第 280-281 行):
```ts
// 创建单例实例
export const danmakuService = new DanmakuService();
```

**修改后**:
```ts
let _instance: DanmakuService | null = null;

/**
 * 获取 DanmakuService 单例（懒初始化）
 * 必须在 Pinia 安装之后、组件上下文中首次调用
 */
export function getDanmakuService(): DanmakuService {
  if (!_instance) {
    _instance = new DanmakuService();
  }
  return _instance;
}
```

### 修改文件 2-6: 5 个 Vue 组件引用更新

所有使用 `import { danmakuService } from '../services/danmaku-service'` 的文件改为:
```ts
import { getDanmakuService } from '../services/danmaku-service';
const danmakuService = getDanmakuService();
```

- `src/renderer/views/Login.vue:7`
- `src/renderer/views/Connection.vue:6`
- `src/renderer/views/DanmakuPool.vue:5`
- `src/renderer/views/DanmakuList.vue:5`
- `src/renderer/views/Dashboard.vue:6`
- `src/renderer/views/Settings.vue:6`

### 修复风险评估
- **影响其他调用方**: 无，`getDanmakuService()` 返回同一单例，行为完全兼容
- **副作用**: 每个组件的 `<script setup>` 在 setup 阶段调用 `getDanmakuService()`，首次调用时初始化单例，后续调用返回已有实例
- **数据修复**: 不需要

---

## 验证

- [x] Vite build 成功: `npx vite build --mode development` 无错误
- [x] 386 个模块全部转换成功
- [ ] 运行应用验证界面正常显示 (需要 Electron 环境)