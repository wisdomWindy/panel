# Code Context

## Context Requirement

需要定位主题类和空状态的渲染所有权。缺陷范围局限于单个 Vue 页面，结构图仅为偏好而非必要。

## Graph Availability Check

- 状态：missing / 未发现仓库已配置的 code graph 命令。
- 检测方式：检查仓库工作流与目标页引用后，使用定向 `rg` 搜索。
- 迭代：bugfix-intake，iteration 1。

## Installation Or Bootstrap Record

- attempted：no。
- result：not_needed。
- 原因：目标页、主题入口和空状态所有权均由单文件与直接引用完整覆盖，S1 流程不需要扩大工具面。

## Fallback Record

- used：yes。
- method：定向 `rg` 与完整文件读取。
- confidence：高；两个症状均有直接模板证据。

## Relevant Entrypoints

- 用户入口：应用壳导航 `/rankings`。
- 主题入口：`useApplicationShellRuntime` -> 根 `.dark` 类；`main.ts` 导入 Element Plus dark variables。
- 页面入口：`views/rankings/index.vue`。

## Functional Chain Under Change

1. 用户切换深色主题，现有 runtime 为 `document.documentElement` 设置 `.dark`。
2. `App.vue` 应用壳响应 `.dark`，但排行榜根 section 固定 `bg-white`，导致浅色表面覆盖深色壳。
3. 空数据时 `el-table` 自带 empty slot 文案；页面又在表格后渲染一个自定义“暂无数据”。
4. `el-table height="100%"` 占满滚动容器，第二个空状态位于其后，向下滚动才出现。
5. 修复所有权：只调整排行榜模板类并删除重复自定义空状态；现有表格继续拥有空状态。

## Key Symbols And Modules

- `rows`：当前榜单切片，不修改。
- `el-table`：唯一空状态 owner。
- 排行榜根 section/header/footer：深色样式适配位置。

## Dependency And Side-Effect Boundaries

不修改 Pinia store、查询副作用、IPC、路由或主题状态；仅模板表现层变化。

## Impact Scope

- 预计修改 `views/rankings/index.vue`。
- 新增同目录页面契约测试。
- 回归敏感区：浅色模式、表格滚动高度、数据行和页脚可读性。

## Open Follow-Up Checks

- 自动化验证只有一个空状态 owner。
- 浏览器/安装应用中检查深色页无白色大块且无重复空状态。

