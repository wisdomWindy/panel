---
name: frontend-agent-framework-intake
description: Stage subskill for intake. Create the request workspace from a PRD-driven source or stable direct-change instruction and write only intake-stage artifacts.
---

# Intake Subskill

## 触发场景

- 当主 `frontend-agent-framework` 将一个新的 PRD 驱动需求路由到 `intake` 阶段时使用。
- 当主 `frontend-agent-framework` 将一个稳定的直接前端修改、清理、删除或调整请求路由到 `intake` 阶段时使用。
- 适用于需要从可读 PRD 文档、稳定 PRD 引用或可执行的用户直接指令创建标准请求工作区的场景。

## 必要输入

- 可读的 PRD 文档、稳定 PRD 引用，或稳定的直接变更指令。
- 可接受输入包括：
  - Feishu/Lark PRD 链接
  - 文档 ID
  - 本地 PRD 文件路径
  - 明确目标页面、组件、文件、行为、清理项、删除项或可观察结果的用户直接指令
- 足够的需求上下文，用于推导稳定的 `request-id`。

## 执行步骤

1. 先读：
   - `../../references/framework-overview.md`
   - `../../references/state-machine.md`
   - `../../references/authoring-guide.md`
   - `../../references/templates/request.md`
   - `../../references/policies/policy-index.md`
   - 按 `policy-index.md` 的 `intake` 阶段映射读取本次输入适用的 policy 文件；默认至少读取 `source-grounding.md`、`doc-writing.md`、`workflow-efficiency.md`；当请求包含创建 / 初始化项目、应用、包、前端业务面、micro frontend、组件库、SDK package 或 buildable frontend entry 等 greenfield 信号时，必须读取 `project-scaffold.md`
2. 读取 PRD 文档、解析稳定 PRD 引用，或归一化稳定直接变更指令，再开始创建下游工件。
3. 创建 `docs/requests/<request-id>/` 作为标准请求工作区。
4. 只写 intake 阶段工件，不写 `spec/spec.md`，不写实现代码。
5. 在 `request.md` 中写入：
   - 初始目标
   - 初始 done signal
   - 初始触发条件
   - 初始上下文来源
   - 预期人工交接点
6. 在 `artifacts/prd-snapshot.md` 中做结构化提取，而不是薄摘要；直接变更请求也使用该文件作为下游可消费的 source snapshot。必须建立 verbatim source evidence index，保留需求原文、稳定章节锚点或用户直接指令原话；归纳摘要不能成为独立授权来源。
7. 如果 PRD 或直接变更指令已明确给出字段、表格、展示、交互、校验、loading、流程分支、修改目标或删除目标等细节，必须将这些显式约束规范化写入 `artifacts/prd-snapshot.md`。
8. 如果上游来源结构不一致或信息不完整，区分“已确认事实”和“开放问题”，不要混成模糊总结。
9. 对直接变更请求，本阶段是其 source-grounding 与人工确认前置 gate：用户直接指令是主来源，必须建立 closed-world product scope allowlist，逐项引用用户原话。仓库事实只能用于定位影响面、引用链和必要清理闭包，不能扩展为相邻重构、额外优化、未请求功能或新的产品语义。
10. 对直接变更请求，如果目标、范围、预期结果、禁止解释、关键规则、歧义内容、模糊内容、来源依据或可观察完成条件仍需用户确认，必须停在 intake 并记录阻塞原因；不能把确认推迟到 `spec` 或 `plan`。
11. 对创建 / 初始化项目、应用、包、前端业务面、micro frontend、组件库、SDK package 或 buildable frontend entry 的请求，必须在 `request.md` 和 `artifacts/prd-snapshot.md` 记录 `greenfield_scaffold_sensitive=true` 或等价说明，并写清：
   - 触发 greenfield / scaffold-sensitive 的用户原话或 PRD 来源
   - 项目类型、目标运行面、仓库 / package 归属、技术栈约束和部署目标中已知的信息
   - 来源是否指定脚手架、starter、模板、生成器、同类项目基线或 monorepo command
   - 需要在 requirement-analysis 或 spec 前完成的脚手架候选发现范围
   - 若存在多个脚手架候选且技术栈 / 部署 / 维护后果不同，必须作为前置人工确认项，不能推迟到 execute
12. 初始化 `state.json.speed_profile`，按 `workflow-efficiency.md` 选择最低安全档位：
   - `S0 trivial`：单点局部修改 / 删除 / 文案 / 显式清理，影响闭包清楚
   - `S1 local`：单页面、单组件、单 hook / composable 或单请求链路
   - `S2 scoped`：多文件、完整功能链、API / adapter / validation / removal cleanup / reuse decision
   - `S3 broad`：PRD、多模块、跨系统、迁移、架构敏感或需求边界不清
   对 `S0` / `S1` 记录 compact artifact 与 scoped context；若发现风险扩大，后续阶段必须升级 profile。
13. 初始化 `state.json.loop`，并把 `stage` 设为：
   - PRD 驱动需求：`requirement-analysis`
   - 直接变更请求：若页面布局、样式或交互结构是主要工作则为 `page-design`；否则为 `spec`
   - 对直接变更请求，代码结构、文件关系、函数结构、数据结构或类型策略是否重要，只记录为 `architecture_design_required` / 架构路由信号；必须等 framework-approved `spec` 形成行为合同后，再进入 `architecture-design`
14. 对 PRD 驱动需求，将 `state.json.module_flow` 初始化为 `null`；先由 `requirement-analysis` 固化范围、风险与拆分依据，再由 `requirement-splitting` 负责补全顺序执行队列。
15. 对直接变更请求，将 `state.json.module_flow` 初始化为 `null`，并在 `request.md` 和 `artifacts/prd-snapshot.md` 中记录为什么该请求不需要 PRD-only requirement-analysis / requirement-splitting。
16. 参考样例 `docs/requests/prd-material-delivery-config-center/` 时，只复用结构和质量基线：
   - 可参考目录结构
   - 必须重写 `README.md`、`request.md`、`state.json`、`artifacts/prd-snapshot.md`
   - 不得沿用样例中的 `spec/plan/execution/verification/review` 业务内容

## 输出格式

- 必须产出：
  - `docs/requests/<request-id>/request.md`
  - `docs/requests/<request-id>/artifacts/prd-snapshot.md`
  - `docs/requests/<request-id>/state.json`
- 最终交付物应包含：
  - 请求来源摘要
  - 规范化 PRD / 直接变更快照
  - 当上游存在时，对字段、表格、展示、交互、状态、流程约束的结构化提取
  - 初始请求状态

## 验收标准

- 请求工作区已创建。
- `request.md` 与 `state.json` 已记录来源链接。
- `request.md` 已记录明确目标、初始 done signal、触发条件、最小上下文、人工交接点。
- `state.json.speed_profile` 已记录最低安全档位、原因、context budget 和 artifact density。
- `artifacts/prd-snapshot.md` 已生成。
- `artifacts/prd-snapshot.md` 没有把上游明确行为约束压缩成过粗摘要。
- `artifacts/prd-snapshot.md` 已按请求适用区分 source-backed、code-fact-backed、confirmed-decision、source-derived、missing-source 和 out-of-scope 内容。
- `artifacts/prd-snapshot.md` 已保留 verbatim source evidence index；直接变更请求已建立 closed-world product scope allowlist，且归纳摘要、代码事实或相邻模块没有被当成产品授权来源。
- 如果请求创建 / 初始化项目、应用、包或前端业务面，`request.md` 和 `artifacts/prd-snapshot.md` 已记录 scaffold-sensitive 信号、项目类型、已知技术 / 部署约束、来源指定脚手架或候选发现范围；没有把脚手架选择留到 execute。
- 直接变更请求如需人工确认，或存在会影响 downstream 的歧义 / 模糊内容，已在 intake 阶段解决或阻塞，没有把确认推迟到 `spec` / `plan`。
- 直接变更请求没有把仓库相邻行为、样例内容或常规做法扩展成本次范围。
- `state.json` 已初始化完成，且：
  - PRD 驱动需求进入 `requirement-analysis`
  - 直接变更请求进入请求级 `page-design` 或 `spec`
- `state.json.loop` 已按框架规定形状初始化。
- `state.json.module_flow=null`；PRD 驱动需求等待 `requirement-analysis` 与 `requirement-splitting` 继续生成下游状态，直接变更请求保持请求级交付单元。

## 安全边界

- 不能在本阶段规划实现任务。
- 不能写 `spec` 工件。
- 不能写代码。
- 不能宣称请求已具备实现准备度。
- 不能把直接变更指令扩展成用户没有要求的相邻优化、重构或行为新增。
- 不能把样例请求中的下游阶段内容当作当前请求的有效内容直接复用。
- 不能把创建项目 / 应用 / 包 / 前端业务面的请求当成普通代码新增；必须先标记 scaffold-sensitive 并把脚手架发现交给上游工件承接。
