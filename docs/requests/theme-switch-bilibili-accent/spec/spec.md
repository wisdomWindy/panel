# 规格：主题一致性与 bilibili 主色

## 交付单元标识

`theme-switch-bilibili-accent`

## 背景和目标

Tailwind 4 将 `dark:` 编译为系统媒体查询，而应用与 Element Plus 通过根节点 `.dark` 切换，导致主题控制失效和黑白混用。目标是建立单一主题状态源，并将默认主色统一为 `#FB7299`。

## 范围内

- Tailwind dark variant 改为根节点 `.dark` selector。
- light/dark/system 投影一致，system 响应操作系统实时变化并清理监听。
- Element Plus 基础及派生主色均由当前 accent 生成。
- 默认主色从 `#FF6699` 更新为 `#FB7299`；边界 adapter 只迁移精确旧默认。
- 聚焦测试、全量测试、构建和安装版三模式视觉验证。

## 范围外

布局、文案、导航、业务状态、API、媒体效果的 primary/secondary 配色及非旧默认自定义色。

## 触发和启动条件

应用启动或用户在设置页预览/保存主题；system 模式下操作系统配色变化。

## 需求拆分摘要

不适用：S1 非 PRD 单链路。

## 来源锚定与可追溯性

| 规格项 | 标签 | 来源 | 下游义务 |
| --- | --- | --- | --- |
| 三模式即时一致 | source-backed | U1/U2 | CSS selector、projection、runtime test |
| `#FB7299` 完整主色色阶 | confirmed-decision | U3/U4 | projection 与默认值 |
| system 实时监听/清理 | confirmed-decision | U4 | runtime lifecycle |
| 旧默认迁移、自定义保留 | confirmed-decision | U4 | adapter test |
| 不改其他产品语义 | source-backed | U1-U4 closed world | diff audit |

Closed-world allowlist 仅含上述视觉状态与默认值变化。

## 工作流效率配置

`S1 local` compact；只读取主题链与直接测试。若需改变组件边界、后端数据结构或全局设计系统，必须升级回 spec。

## 自愈完成标准

要求测试、类型、lint、Rust 相关测试、Web/NSIS 构建、生成 CSS selector 和安装版三模式 DOM/像素证据全部通过；无监听泄漏、混色、默认蓝色派生色或无关 diff。完成需 self-healing compliance/assessment pass，blockers none。

## 用户意图合同

- literal：修复主题切换、黑白混用并改为 bilibili 主题色。
- practical：设置可相信，整页与控件形成一致视觉系统。
- success：显式主题不受系统值干扰；system 会跟随；主操作色为 `#FB7299` 色系。
- forbidden：仅改局部背景、强制永远深色/浅色、覆盖用户自定义色、用品牌色替换状态语义色。

## 用户流程、页面与 UI/UX 合同

沿用 `page-design.md`：设置页选择 -> preview -> 全应用投影 -> 保存。布局、信息层级、控件和反馈不变；浅/深表面与文字对比一致。

## 前端样式约束

仅 Tailwind 4 `@custom-variant` 配置、既有 utilities 和 Element Plus CSS variables；不新增 scoped CSS/semantic class/inline style object，不隐藏长 class。

## 字段与交互视觉状态

主题 segmented control 与 color picker 语义不变。品牌主色用于 Element Plus primary/default hover/active/light states；成功/警告/危险语义色不变。

## 功能完整行为分解

1. `applyUiProjection` 对 light/dark/system 解析出最终 `isDark`，同步 `.dark`、`color-scheme`、dataset 与所有现有 shell variables。
2. 主色投影至少写入 `--shell-accent`、`--el-color-primary`、`--el-color-primary-rgb`、light-3/5/7/8/9 与 dark-2。
3. runtime 仅在存活期间监听 `prefers-color-scheme: dark`；变化时仅对当前 system 偏好重投影；stop 对称移除。
4. adapter 将精确 `#FF6699` 归一为 `#FB7299`，其他合法颜色原样返回。
5. Rust/UI 默认值统一为 `#FB7299`。
6. 生成 CSS 的 dark utilities 由 `.dark` ancestor 控制，不再包在 dark media query 中。

## 专业前端工程约束

主题偏好仍由 shell store 单一拥有，DOM 是 derived projection；监听由 runtime 管理，无第二份 writable theme、无 watcher fan-out 或持久化副作用。

## 前端架构设计路由输入

`architecture-design_required: false`：owner、依赖方向和公开合同不变，仅修复既有 owner 内逻辑。

## 生产代码质量与函数式约束

复用 `UiPreferences`/`ThemeMode` 与 DOM `MediaQueryList` 类型；主题解析和 accent 归一保持确定性，DOM/listener 副作用留在 projection/runtime。无 `any`、class、cache、debounce 或新依赖。

## 人工评审与局部性

允许改动 Tailwind 入口/config、projection、runtime、adapter、InterfaceSettings 默认、Rust shell default 与对应测试/工件。不得修改页面布局、locale、业务组件或生成声明；不新建单函数 utility/组件。

## 架构复用与模式决策

复用既有 projection/runtime/adapter owner。Pattern decision：Level 1 `adapt lightweight pattern`，用现有 projection 作为 Tailwind/Element Plus adapter，用 runtime lifecycle 管理 observer；拒绝新增 theme service/store/plugin。

## Vue 组件抽取与脚手架

不适用：无 SFC 边界变化，非 greenfield。

## API、前后端责任与 TypeScript 上下文

无后端 API shape 变化；Rust 只更新 domain default。TypeScript 由 package `tsconfig.eslint.json` -> `tsconfig.json` 管理，`strict`、bundler resolution、DOM lib 适用。

## 边界情况

- 显式 light 在系统 dark 下保持浅色；显式 dark 在系统 light 下保持深色。
- system 初始化和后续 change event 均正确。
- runtime 重启不重复注册，stop 后事件不再投影。
- accent 为旧默认、bilibili 默认或任意其他合法十六进制色。

## 验收标准

1. RED 测试证明当前 CSS dark 仍使用 media query、system 无监听、旧默认未迁移且 Element Plus 派生色缺失。
2. projection 测试覆盖 light/dark/system 与完整主色变量。
3. runtime 测试覆盖 media query listener 注册、system-only 重投影与 cleanup。
4. adapter/Rust 测试覆盖新默认、旧默认迁移及自定义色保留。
5. focused/full Vitest、type-check、lint、Rust 相关测试、Web/NSIS build 通过。
6. 生成 CSS 含 `.dark` selector 而非 dark media query；安装版三模式的 shell/Element Plus 表面一致，primary computed color 对应 `#FB7299`。

## 风险

主要风险是 Tailwind v4 config 与 CSS directive优先级、Element Plus 派生变量遗漏及 listener cleanup；均由构建产物和 runtime 测试/实机覆盖。

