# 直接指令规范化快照

## verbatim source evidence index

| ID | 来源 | 原文 | 授权范围 |
| --- | --- | --- | --- |
| SRC-001 | 当前用户指令 | `根据当前项目提出优化建议` | 审查当前仓库并提出优化建议 |

## source summary

用户授权对当前项目进行优化评估。该授权允许读取仓库、运行非破坏性检查并形成建议，不等同于授权修改生产代码或新增产品能力。

## key business goals

- `source-backed`：针对“当前项目”给出优化建议。
- `source-derived`：建议应以当前仓库事实为依据，否则无法证明其针对性。
- `confirmed-decision`：无。

## greenfield scaffold sensitivity

不适用：不是创建或初始化项目、应用、包或前端业务面。

## explicit behavior constraints

- `source-backed`：输出优化建议。
- `source-derived`：建议需可理解、可排序、可落地。
- `missing-source`：用户未指定只关注性能、UI、架构或构建，因此允许覆盖工程质量的主要维度，但不得扩展为新产品需求。

## closed-world product scope allowlist

- 允许：针对现有仓库事实提出工程、架构、质量、测试、构建、依赖与文档优化建议。
- 允许：为建议说明优先级、收益、风险、工作量和验证方式。
- 不允许：直接新增或修改用户可见功能、字段、文案、交互、权限、导航、接口语义或业务流程。
- 不允许：未经用户确认直接实施生产代码重构。

## forms, tables, displays, and interactions extracted from source

不适用：源指令没有授权具体页面或交互改动。

## workflow and state rules extracted from source

- 起点：收到当前项目优化评估请求。
- 处理：读取项目事实、运行非破坏性质量检查、归纳证据。
- 终点：交付分级建议并明确证据和验证方式。
- 人工交接：由用户选择后续实施项。

## relevant modules or pages

由评估证据确定；初始范围包括 workspace、两个 package、Electron 主应用、官网、测试与 CI。

## notable open questions

- 无阻塞问题。用户未限定优化维度，可在不新增产品语义的前提下做仓库级工程评估。

## grounding classification rules for downstream artifacts

- 仓库文件、配置与命令结果只能作为 `code-fact-backed` 工程证据。
- 通用最佳实践若没有当前仓库触发信号，只能列为低置信候选或不列入建议。
- 任何后续代码实施需另行建立变更请求和对应 source allowlist。
