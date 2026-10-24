# 执行计划：上舰特效内置模板

## 目标与执行方式

在不改变现有 command/event、持久化 config 和窗口链路的前提下，交付 12 个真实可选、可预览的内置模板。任务紧密关联，采用单路径串行执行，不启用并行代理。

## 变更链审查

- 功能入口：`views/captain-effect/index.vue` 加载 snapshot、选择模板、预览/保存。
- 目录 owner：`src-tauri/src/ipc/media.rs::captain_snapshot`；当前 `Vec::new()` 是根因。
- 合约链：`CaptainEffectPresetDto -> CaptainEffectPreset -> adaptCaptainEffectSnapshot -> media effects store -> page`。
- 渲染链：`CaptainEffectConfig.presetId -> captain_effect_preview/trigger -> CaptainEffectPayload -> CaptainEffectSurface.draw`。
- 邻近但不修改：Pinia store 异步去重、Rust effect queue/window port、音效绑定、窗口尺寸/位置。
- 删除链：移除 `materialsUnavailable` UI 分支后，同步删除中英文 locale 键，用 `rg` 证明无孤儿引用。

## T1：先建立失败测试与类型合同

- 来源：spec “内置模板目录合同”、AC1/AC2；product-content delta：12 项目录及其已批准名称/默认色。
- 目标文件/符号：`ipc/dto.rs::CaptainEffectPresetDto`、`ipc/media_contract_test.rs`、`contracts/media.ts::CaptainEffectPreset`、`adapters.test.ts`、`media-pages-contract.test.ts`。
- 步骤：
  1. 向 Rust/TS 预设类型增加 `primaryColor` / `secondaryColor`。
  2. 添加 Rust 测试，要求 `captain_snapshot` 目录数量为 12、ID 唯一、包含 `golden-premium / 金色尊享`、颜色格式合法。
  3. 更新 adapter fixture 并添加非法目录颜色失败测试。
  4. 更新页面/Canvas 合约测试，先要求选择处理、六种装饰键和旧警告清理。
  5. 运行聚焦 Vitest/Rust 测试，记录预期失败（red）。
- 约束：先合约后实现；无 `any`、无不安全断言；adapter 使用纯验证函数并返回冻结对象。
- 验证：`pnpm exec vitest run src/renderer/modules/media/adapters.test.ts src/renderer/components/media/media-pages-contract.test.ts`；`cargo test ipc::media_contract_test`。

## T2：实现 Rust 目录、adapter 与页面选择

- 来源：spec “目录合同”、“页面行为”、AC1-AC3；ADR-01；product-content delta：同 T1。
- 目标文件/符号：`ipc/media.rs::captain_effect_presets/captain_snapshot`、`adapters.ts::adaptCaptainEffectSnapshot`、`captain-effect/index.vue::handlePresetChange`、`zh-CN.json`、`en-US.json`。
- 步骤：
  1. 在 `ipc/media.rs` 就近创建无 I/O 的 12 项目录函数，snapshot 使用它替代空数组。
  2. 在 adapter 内复用同文件 `hexColor` 纯 helper 校验 config 和 preset 颜色。
  3. 将 `el-select` 切换改为显式 `@change="handlePresetChange"`；handler 仅修改草稿 ID/两个颜色，不触发保存。
  4. 移除 `materialsUnavailable` 警告和 locale 死键。
  5. 运行 T1 聚焦测试至 green，再执行局部 refactor。
- 状态/副作用：目录 snapshot 只读；页面 handler 是唯一可写边界；保存/预览异步 owner 不变。
- 回退条件：如果新字段破坏已有 contract 消费者，回到 architecture-design 重新确认 DTO 边界，不在页面写第二份目录。

## T3：实现 Canvas 多风格渲染

- 来源：spec “Canvas 渲染合同”、AC4/AC5；ADR-02；product-content delta：none（技术渲染落地）。
- 目标文件/符号：`CaptainEffectSurface.vue::draw`，组件内 `CaptainDecoration`、映射与绘制 helper。
- 步骤：
  1. 定义不少于 6 种装饰键的只读映射，覆盖 12 个目录 ID，未知 ID 回退 `classic`。
  2. 用就近纯几何 helper 绘制星芒、颗粒、环、光带、冠冕或晶体装饰；装饰在文字前绘制。
  3. 保留现有 fade、requestAnimationFrame/cancelAnimationFrame、DPI、ResizeObserver、reduced motion 与 aria-live 逻辑。
  4. 运行合约测试和类型检查，修复并重构到 green。
- 性能：每帧固定小量绘制操作，不创建 DOM、不加载资源、不缓存无界对象。

## T4：变更链清理、验证与审阅预检

- 来源：AC1-AC6；product-content delta：none。
- 步骤：
  1. `rg` 重查 `materialsUnavailable`、`captain-default`、所有新 preset ID、`CaptainEffectPreset` 参考链，确认无孤儿符号和缺失链路。
  2. 运行聚焦 Vitest、全部 renderer TypeScript check、ESLint check、Rust IPC/media 聚焦测试。
  3. 检查 `git diff --check` 和 scoped diff，不纳入用户其他未提交改动。
  4. 如本地 web 开发模式可运行，启动未占用端口并做页面烟测；桌面特效窗口的剩余风险明确记录。
- 审阅预检：鲁棒性（未知 ID/非法 DTO）、可维护性（单 owner/无重复目录）、性能/内存（有界每帧工作和完整清理）、技术栈（Vue 3 + Canvas 2D + 现有 Rust IPC，无新依赖）。

## 类型与样式约束

- governing TypeScript：`packages/bilibili-live-tool/tsconfig.json` + `tsconfig.eslint.json`；合约声明：`platform/desktop/contracts/media.ts`。
- 本次不新增 authored CSS；如需页面 class 调整，只使用内联 Tailwind utilities，不藏入常量/computed。
- Vue 不新拆 SFC；不添加 helper/manager/factory 文件。

## 验证证据预期

- Rust：目录数量/内容/序列化测试。
- TypeScript：adapter 冻结和 fail-closed 测试，`tsc --noEmit`。
- UI 合约：选择处理、死警告清理、Canvas 策略覆盖与原生命周期特征。
- 审阅：变更 hunk 到 T1-T4 的映射、无无关格式化、无 debug/dead/stale 痕迹。
