# 执行变更记录

## 2026-06-12

### T1: 移除 Login.vue 根容器 p-10 类

- **文件**: `src/renderer/views/Login.vue`
- **变更**: 删除根 `<div>` class 中的 `p-10`，保留 `h-screen w-screen flex flex-col items-center justify-center`
- **TDD**: 不适用（纯样式变更，非可测试行为）