# 代码上下文

## Context Requirement

需要确认共享边距 owner 与四个页面重复 padding 的关系。bugfix-intake 首次需要；定向检索足够，code graph 仅为可选。

## Graph Availability Check

- 状态：missing / not required for this S1 local class-only change。
- 检测方式：沿 `App.vue -> router-view -> route SFC` 定向读取与 `rg` 检索。
- 迭代：1。

## Installation Or Bootstrap Record

- attempted: no
- result: not_needed
- 原因：没有符号调用链、跨模块状态或依赖方向变化。

## Fallback Record

- used: yes
- 方法：读取 `App.vue`、四个目标页面、正常页面与 shell contract。
- 置信度：高；所有间距类都在模板根节点可直接审计。

## Relevant Entrypoints And Functional Chain

用户导航到目标路由 -> `App.vue` `<main class="... p-3 sm:p-5 ...">` 提供共享边距 -> `<router-view>` 渲染页面 -> 目标页面根 `<section>` 再叠加 `p-[18px] xl:p-6` 或 `px-7 py-6`。本次 owner 是四个 route 根节点的重复 padding；shell、子组件、store、IPC 和业务状态均为禁止修改的邻近链路。

## Key Symbols And Modules

- `App.vue` `<main>`：唯一共享页面外侧间距 owner，保持不变。
- 四个 route `index.vue` 根 `<section>`：删除重复 padding。
- `shell-components-contract.test.ts`：锁定 shell 与 route 间距所有权。

## Dependency And Side-Effect Boundaries

仅 Vue 模板静态 class；无 import/export、请求、store、事件、watch、computed、IPC 或副作用变化。

## Impact Scope And Follow-Up

- 预计改动：四个根 class 与一个合同测试。
- 验证：RED/GREEN、相关测试、type/lint/build、桌面与窄屏路由 computed padding/标题位置。
- 回归邻居：其他仍自带 padding 的页面不在 U1 点名范围内，不顺带修改。
