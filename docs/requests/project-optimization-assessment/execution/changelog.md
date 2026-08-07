# 项目优化评估执行记录

## Delivery Unit

- 标识：`project-optimization-assessment`
- 批准规格：`spec/spec.md`
- 批准计划：`plan/plan.md`
- 性质：仓库评估；不修改生产代码或产品行为。

## 执行摘要

已完成 workspace、脚本、CI、TypeScript、构建、测试、Electron IPC、安全边界和文档一致性评估。结论按影响、紧迫度、投入排序；“确认问题”“风险信号”“待验证项”分开标注。详细证据见 `artifacts/code-context.md`。

## 优先级建议

### P0-1 立即轮换并移除仓库中的明文平台凭据

- 分类：**确认问题**。
- 证据：`packages/bilibili-live-tool/src/renderer/stores/config.ts:55-57` 硬编码 access key；同文件 `saveConfig()` 将完整配置写入 localStorage。AI 配置也将 API key 随配置持久化。
- 影响：已提交的 secret 应视为已泄露；renderer/localStorage 可读性放大泄露面。仅删除 Git 当前版本不能使旧 secret 失效。
- 建议动作：
  1. 立即在 Bilibili 开放平台轮换 key，并检查使用/审计记录。
  2. 删除所有真实默认凭据，改为空值或开发环境显式注入。
  3. secret 由 main 进程持有，使用操作系统 keychain（如 keytar）或至少受控的 main-process secret store；renderer 仅传递引用/状态。
  4. 配置导出、日志和错误对象默认脱敏；添加 secret scanning（如 gitleaks）。
- 投入：S～M。
- 验证：仓库历史扫描不再发现有效 secret；旧 key 调用失败、新 key 正常；renderer DevTools/localStorage/导出文件不出现 secret。
- 依赖/风险：需先轮换再清理代码；若要清理 Git 历史需协调所有协作者重新同步。

### P0-2 收紧 preload/IPC 与文件系统权限边界

- 分类：**确认问题**（任意 channel 与任意路径能力存在）；可利用性取决于 renderer 内容注入，属于**风险信号**。
- 证据：`src/preload/index.ts` 暴露 `invoke(channel: string)`、`on(channel: string)`；`src/main/ipc-handlers.ts:161-176` 对 renderer 传入路径直接执行 read/readdir/cp/rm，含递归强制删除。主进程约有 66 个 handlers。
- 影响：renderer 一旦发生 XSS、依赖污染或恶意插件执行，可调用未预期 channel，并对用户权限范围内文件进行读写删除。
- 建议动作：
  1. 移除公开通用 invoke/on/off，改为按业务命名的最小 API 和固定 allowlist。
  2. 建立共享 `IpcContract`（channel、参数、返回、事件）并在 main 边界做 runtime schema 校验。
  3. 文件操作仅允许 app-owned 根目录或由原生 dialog 授权的路径；`resolve/realpath` 后校验归属，拒绝 symlink/path traversal 和根目录删除。
  4. 校验 `event.senderFrame` 来源，敏感操作增加确认/能力 token；配置 CSP，并显式评估 `sandbox: true`。
- 投入：M～L。
- 验证：契约测试覆盖未知 channel、越界路径、`..`、symlink、错误 sender；均拒绝；正常录制/插件目录流程通过。
- 依赖/风险：需先盘点 66 个 handler 和现有调用方，分批迁移以避免破坏桌面功能。

### P0-3 把插件执行从 renderer 的 `new Function` 中隔离或暂时降权

- 分类：**确认问题**（执行方式）；“沙箱”可逃逸/利用的具体路径为**风险信号**。
- 证据：`src/renderer/plugins/plugin-sandbox.ts` 使用 `new Function` 执行插件源码；`plugin-manager.ts` 将其用于实际插件加载。该代码与 renderer 共享 JavaScript realm，并非安全边界。
- 影响：不可信插件可影响 renderer；结合通用 IPC/文件 API，风险可跨到主进程能力。当前“沙箱”命名可能造成错误安全预期。
- 建议动作：在完成隔离前仅允许内置/签名/用户明确授信插件；将执行迁至独立 utility process/worker/独立受限 renderer，使用消息协议和 capability-based API，设置超时、资源上限、签名/哈希和权限审计。禁止把 `vm` 或 Function 参数遮蔽当作完整安全方案。
- 投入：L。
- 验证：恶意插件测试尝试访问 `window/globalThis/electronAPI`、无限循环、网络和文件系统；宿主保持可用且越权失败。
- 依赖/风险：与 P0-2 联动；这是架构项，建议先禁用不可信来源，再逐步重构。

### P1-1 恢复一个可复现、单一来源且 fail-fast 的构建链

- 分类：**确认问题**。
- 证据：`vite.config.ts` 已通过 `vite-plugin-electron` 构建 main/preload；`scripts/build.js`、`scripts/dev.js` 又用 `npx tsc` 构建。两者用 `shell: true`，Node 24 触发 DEP0190。dev 未等待 preload build，固定 1 秒后启动 Electron。
- 影响：配置漂移、错误二进制入口、启动竞态、锁文件外工具解析；发布可能在某一环境偶发失败。
- 建议动作：
  1. 选定 Vite/electron plugin 为唯一 dev/build owner，或明确拆成独立 tsconfig + 本地 binary；删除重复路径。
  2. 用 `pnpm exec`/workspace script，禁止 `npx` 和不必要的 shell；监听 `error/close` 并 fail-fast。
  3. dev 通过插件 onstart 或 wait-on 等 readiness 信号启动 Electron，统一清理子进程。
  4. 根 package 声明 `packageManager`，CI 使用同一 pnpm major 和 frozen lockfile。
- 投入：M。
- 验证：全新 checkout 的 Linux CI 能安装、type-check、build；Windows/macOS smoke package；任一前置步骤失败时整体非零退出且不继续。
- 依赖/风险：Electron postinstall 在当前环境两次超时，应先保证镜像/代理/缓存可用。

### P1-2 修复 TypeScript 基线，并按 Electron 进程拆分类型边界

- 分类：**确认问题**。
- 证据：`pnpm type-check` 报 5 个错误，包括 renderer logger 对 main 路径的错误导入、`any` 索引固定 map、未使用函数。当前主 tsconfig 同时覆盖 main/preload/renderer。
- 影响：当前 strict TypeScript 门禁不可用；跨进程导入容易把 Node/Electron 实现泄漏给 renderer，并掩盖真正 IPC 契约。
- 建议动作：
  1. 先清零现有 5 个错误，禁止降低 strict 或扩大 skip。
  2. 建立 `tsconfig.main.json`、`tsconfig.preload.json`、`tsconfig.renderer.json` 和共同 base；各自限定 `lib/types/module`。
  3. 将共享 DTO/IPC 类型放在不依赖 main 或 DOM 的 contract 模块；renderer 不直接导入 main 实现。
  4. 生成类型与 type-check/build 使用同一配置来源。
- 投入：M。
- 验证：三个 scoped type-check 全绿；架构 lint/依赖测试拒绝 renderer→main 私有导入。
- 依赖/风险：与 IPC 契约治理可一起设计，但可先以最小修复恢复红绿反馈。

### P1-3 重建 monorepo 测试闭环并接入主应用 CI

- 分类：**确认问题**。
- 证据：根 tests 约 18 个，多数导入已不存在的 `../src/...`；少量使用 `node:test`，其余自制 runner；package 无 `test` script 或明确 runner。现有 CI 只部署 homepage。
- 影响：迁移后测试实际不可运行，回归保护不可依赖；主应用提交缺少 lint/type/test/build 门禁。
- 建议动作：
  1. 选择一个 runner（优先 Vitest，与 Vite/Vue 栈一致）并统一测试 API、退出码、coverage。
  2. 将测试归属到主 package 或修正 workspace alias；按 unit / contract / integration / Electron smoke 分层。
  3. 优先覆盖 P0 边界：IPC schema/path policy、secret redaction、plugin capability；再覆盖录制和 Bilibili parser。
  4. 新建 PR CI：frozen install → lint → scoped type-check → unit/contract test → build；homepage workflow 保留部署职责。
- 投入：M。
- 验证：`pnpm test` 在 clean checkout 单命令运行；故意破坏断言会阻断 CI；报告稳定产出。
- 依赖/风险：先修路径和 runner，再谈覆盖率目标；不要把现有文件数当成有效覆盖率。

### P1-4 修复 IPC 事件取消订阅语义和生命周期清理

- 分类：**确认问题**。
- 证据：preload `on` 注册的是包装 callback，`off` 却移除原 callback，无法匹配；`recording/index.vue` mounted 注册热键事件但无 unmounted 清理；recording store 初始化即注册三个监听器。
- 影响：页面反复进入或 HMR 时可能积累监听器，导致重复操作、重复提示和内存泄漏。
- 建议动作：让 `onXxx` 返回 disposer，或维护 callback→wrapper 映射；Vue 在 `onUnmounted` 调 disposer；store 提供幂等 setup/dispose，并在 app 生命周期明确 owner。
- 投入：S。
- 验证：挂载/卸载多次后 listener count 回到基线；一次主进程事件只触发一次处理；添加回归测试。
- 依赖/风险：最好与 typed IPC facade 一并完成，亦可独立先修。

### P2-1 统一工具版本、安装诊断和 CI 缓存策略

- 分类：**确认问题 + 环境阻塞**。
- 证据：Node 版本约束为 24；workflow 用 pnpm 9，本地为 10，根无 `packageManager`。两次 frozen install 均卡在 Electron postinstall，导致 eslint/vitepress bins 未完整创建。
- 影响：开发者环境与 CI 行为可能不一致；基础命令失败易被误判为源码问题。
- 建议动作：声明精确 `packageManager` 并用 Corepack；CI `pnpm install --frozen-lockfile`；记录 Electron 下载镜像/代理/缓存及超时诊断；可把纯静态检查与需要 Electron binary 的 package smoke 分开。
- 投入：S。
- 验证：clean container 和开发机按 README 一次安装成功；工具 bin 可用；CI 缓存 miss/hit 都成功。

### P2-2 修正文档、脚本和发布产物的一致性

- 分类：**确认问题**。
- 证据：README 快速开始在根执行不存在的 `pnpm run build:vite`；目录仍描述根 `src/`；语言扩展路径同样过期；README 声称 `BilibiliLiveTool.exe`，builder 配置 `executableName: start`。
- 影响：新贡献者首个操作失败，发布物定位和故障排查成本升高。
- 建议动作：README 以 workspace 根脚本为准，更新 monorepo 结构和产物矩阵；在 CI 增加文档命令/链接 smoke 或从 package metadata 生成关键片段；补开发、测试、打包、secret 配置说明。
- 投入：S。
- 验证：在 clean checkout 逐条执行 Quick Start；三平台产物名称与文档一致；内部路径检查通过。

### P2-3 以职责和指标驱动拆分热点，而非按行数机械重构

- 分类：**风险信号/待验证项**。
- 证据：若干 SFC/service/manager 为 400～647 行；全源码约 83 个 `any`、144 个 console 调用。与此同时，路由已全量 lazy-load，ECharts/Three 已按需和分包。
- 影响：热点文件可能混合 UI、I/O、状态和领域规则，但仅凭行数无法证明缺陷；盲目拆分会增加跳转成本。
- 建议动作：优先对 gift-sound、plugin、Bilibili client、database 建立职责/依赖图和测试后再拆；以函数复杂度、变更频率、bug 密度和 bundle/startup profile 作为门槛。不要把现有 lazy-loading 重做一遍。
- 投入：M～L（按模块分期）。
- 验证：目标模块测试先行；复杂度和循环依赖下降；bundle/startup/内存有基线与前后数据；功能等价。

## 建议实施顺序

1. **当天止血**：轮换 secret；限制插件来源；建立安全公告/审计记录。
2. **第 1 个迭代**：恢复 install/type-check/test/CI 基线；修 IPC disposer；统一构建入口。
3. **第 2 个迭代**：typed IPC + 路径授权 + sender/schema 校验；拆分 tsconfig。
4. **后续专项**：插件进程隔离、热点模块重构与性能基准。

## 保留的正向实践

- pnpm catalog 已统一主要依赖版本。
- Electron 已启用 `contextIsolation`、禁用 `nodeIntegration`。
- renderer 路由均懒加载；ECharts/Three 已做按需/分包。
- TypeScript strict 已开启，应修复门禁而不是放宽规则。

## Self-healing Execution Notes

- iteration：3；初次 execute，非 verify/review 回流。
- 新证据：安装可复现卡在 Electron postinstall；type-check 已从环境阻塞推进到 5 个真实错误；识别明文 secret、插件执行和 IPC 文件权限三个 P0 风险。
- 剩余假设：未在完整安装后运行 lint/homepage build/package；未进行 Electron GUI、外部 API、三平台打包和恶意插件动态验证。
- verify 聚焦：逐条抽查 P0/P1 路径与措辞强度；确认所有失败按环境/代码分类。

## Task-board Updates

- T-01：完成。
- T-02：完成；成功/失败/未验证原因均已记录。
- T-03：完成；关键上下文已读取并分类。
- T-04：完成；建议已按 P0/P1/P2 排序。

## Changed Files and Owned Symbols

- 仅新增/更新请求工件：`artifacts/code-context.md`、`execution/changelog.md`、`plan/task-board.md`、`state.json`。
- 无生产 symbol、调用方、测试或产品行为变更。

## Introduced-content Provenance Ledger

- 请求工件均为技术评估内容；`product-content delta: none`。
- 来源：批准计划 T-01～T-04；未新增字段、控件、状态、默认值、权限、API 语义或用户可见产品行为。

## Implementation / Engineering Notes

- expert frontend engineering：仅评估现有状态；未改变用户旅程、state owner 或数据生命周期。
- frontend architecture：无 architecture-design 工件；记录了 IPC、process tsconfig、plugin isolation 候选，不作为已批准实现设计。
- production code quality：不适用，无生产代码变更。
- human review readiness：diff 限于请求目录；无格式化生产文件、debug code 或用户既有变更归因。
- TDD：不适用；本请求无行为变更，以可复现命令和路径抽查替代。
- change-chain/removal cleanup：不适用；未修改或移除既有代码。
- functional programming/styling/API adapter：不适用；仅审查。
- TypeScript context：读取主 package `tsconfig.json`、`tsconfig.eslint.json`；实际运行 scoped type-check；未全仓扫描声明。
- architecture conformance：不适用；未实施架构改动。

## 偏差、阻塞与回滚触发器

- 偏差：依赖安装未在 300 秒内完成 Electron postinstall；因此 lint、homepage build 和 package build 不宣称源码失败。
- 阻塞：完整桌面运行、外部服务和跨平台打包未验证，不影响静态评估交付。
- 回滚：若路径或命令证据失实，仅修正/删除当前请求目录工件；不得修改生产树。

## Verification Handoff

- 抽查 `stores/config.ts`、`preload/index.ts`、`ipc-handlers.ts`、`plugin-sandbox.ts`、build/dev scripts。
- 复核 type-check 的 5 个错误和 tests 旧路径。
- 复核 Git diff 仅位于 `docs/requests/project-optimization-assessment/`。
