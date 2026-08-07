# Code Context — 项目优化评估

## 评估边界

- workspace：根 package + `packages/bilibili-live-tool` + `packages/bilibili-live-tool-homepage`。
- 应用边界：Electron main / preload / Vue renderer；官网为 VitePress。
- 实际变更边界：仅当前请求目录；未修改生产代码、测试和根工程配置。
- 证据时间：2026-03-23；Node `v24.14.1`，pnpm `10.33.0`。

## 工程基线

- 根 `package.json` 提供 dev/build/lint/type-check/homepage 脚本，但无正式 `test` 脚本。
- 仅发现 `.github/workflows/deploy-homepage.yml`；未发现主应用 lint、type-check、test 或 build CI。
- 主应用使用 Electron 30、Vue 3、TypeScript、Vite；`vite.config.ts` 已用 `vite-plugin-electron` 构建 main/preload。
- `scripts/build.js`、`scripts/dev.js` 另行以 `npx tsc` 编译 main/preload，形成重复构建路径。
- 主应用 TypeScript/Vue 源码约 23k 行；代表性热点为 `gift-sound/index.vue` 647 行、`bilibili/client.ts` 461 行、plugin loader/manager 约 458 行、`main/database.ts` 419 行。行数仅作为职责集中信号。

## 命令证据

| 命令 | 结果 | 归因 |
| --- | --- | --- |
| `pnpm install --frozen-lockfile` | 两次均在 Electron postinstall 阶段超时/中止；包链接不完整 | 环境/安装阻塞 |
| `pnpm lint:check` | `eslint: command not found` | 安装不完整，非源码 lint 结论 |
| `pnpm type-check` | 运行成功进入 TypeScript，报 5 个错误 | 确认代码/配置问题 |
| `pnpm homepage:build` | `vitepress: command not found` | 安装不完整，非官网源码结论 |
| `node --test --experimental-strip-types tests/*.test.ts` | 旧 `../src/...` 导入失败 | 测试迁移/runner 闭环问题 |
| `pnpm build`（先前探测） | `shell:true` 触发 DEP0190；依赖缺失时 `npx tsc` 命中错误包 | 构建脚本可复现风险 |

Type-check 的确认错误：

1. renderer logger 文件导入不存在/跨边界错误的 `../../main/modules/logger/logger.js`（2 处）。
2. `log-query.ts` 使用 `any` 索引固定 level map（2 处）。
3. `bilibili-gift-service.ts` 存在未使用函数（1 处）。

## 关键链路与风险证据

### Electron / IPC

- `src/main/index.ts` 已设置 `contextIsolation: true`、`nodeIntegration: false`，这是正向基线。
- `src/preload/index.ts` 同时暴露任意字符串 `invoke/on/off` 和文件系统便捷 API。
- `src/main/ipc-handlers.ts` 注册任意路径的 read/list/copy/delete；未看到路径根目录约束或 sender 校验。
- preload 的 `on` 用包装函数注册，但 `off` 用原 callback 移除，引用不一致；renderer recording 页面也没有 unmount cleanup。
- main 及模块共约 66 个 `ipcMain.handle`，适合建立共享 channel map 与 schema，而非继续字符串分散。

### 凭据与插件

- `src/renderer/stores/config.ts` 包含明文 `accessKeyId` / `accessKeySecret` 默认值，并将完整配置写入 `localStorage`。
- AI store 同样将包含 `apiKey` 的配置写入 `localStorage`。
- `plugin-sandbox.ts` 在 renderer 中使用 `new Function` 执行插件代码；该机制不构成安全隔离，插件代码与 renderer 同权限运行。应按不可信代码风险处理，而非仅称“沙箱”。

### 测试与类型

- 根 `tests/` 约 18 个 `.test.ts`，多数仍从旧根 `src/...` 导入，生产代码已迁至 package。
- 测试风格混杂：少量 `node:test`，多数为自制 assert/console runner；package 无 test runner 依赖和 test script。
- 单一 tsconfig 同时覆盖 main/preload/renderer；renderer 代码直接尝试导入 main logger 类型，暴露进程边界与类型 owner 不清。

### 构建、CI 与文档

- `vite-plugin-electron` 与自定义 tsc 构建重复；dev 脚本未 await preload build，且以固定 1 秒启动 Electron，存在 readiness 竞态。
- `spawn(..., { shell: true })` 搭配参数在 Node 24 触发 DEP0190；使用 `npx` 还可能脱离 lockfile 解析错误工具。
- GitHub Actions 固定 pnpm 9，而本地使用 pnpm 10；根 package 未声明 `packageManager`，可复现性不足。
- README 在 monorepo 迁移后仍引用根 `src/`，根快速开始使用不存在的 `pnpm run build:vite`，并声称 Windows 产物名与 builder 的 `executableName: start` 不一致。

## 正向实践

- workspace 已使用 pnpm catalog 统一主要依赖版本。
- renderer 路由全部懒加载，ECharts/Three 有分包与按模块引入，暂不建议无基准地继续做性能重构。
- Electron 已启用 context isolation 并禁用 renderer Node integration。
- TypeScript 已启用 `strict`，当前 type-check 能暴露真实边界问题。
