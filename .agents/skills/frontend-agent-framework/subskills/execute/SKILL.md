---
name: frontend-agent-framework-execute
description: Stage subskill for execution. Implement only approved tasks and keep execution records aligned with the plan.
---

# Execute Subskill

## 触发场景

- 当主 `frontend-agent-framework` 将请求路由到 `stage=execute` 时使用。
- 适用于按已批准 spec 和 plan 落地代码实现的阶段。

## 必要输入

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/plan/task-board.md`
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/artifacts/code-context.md`（如存在）

## 执行步骤

1. 先读：
   - `docs/requests/<request-id>/state.json`
   - `docs/requests/<request-id>/spec/spec.md`
   - `docs/requests/<request-id>/plan/plan.md`
   - `docs/requests/<request-id>/plan/task-board.md`
   - `docs/requests/<request-id>/artifacts/code-context.md`（如存在）
   - `../../references/state-machine.md`
   - `../../references/templates/code-context.md`
   - `../../references/policies/api-contracts.md`
   - `../../references/policies/clean-code.md`
   - `../../references/policies/code-graph.md`
   - `../../references/policies/design-patterns.md`
   - `../../references/policies/spec-constraints.md`
   - `../../references/policies/frontend-architecture.md`
   - `../../references/policies/frontend-components.md`
   - `../../references/policies/testing.md`
2. 执行前要求 `approvals.plan_approved=true`。
3. 仅实现已批准任务。
4. 把 spec 与 plan 视为唯一实现合同。
5. 如果 plan 缺少关键字段、列、交互、状态或 loading 结束条件，停止执行并回退到 `plan`。
6. 如果 spec 与 plan 在行为颗粒度或产品含义上冲突，停止执行并回退修复工件。
7. 对可测试行为按 red -> green -> refactor 执行：
   - 先写失败测试
   - 再写最小通过实现
   - 最后重构并保持测试通过
8. 实现过程中应用 clean-code 规则和已批准的 pattern 决策。
9. 对 API 集成工作：
   - 服务端有 TS contract types 时优先复用
   - 非 TS 契约时保留后端字段名并用 TS 类型表达
   - proto 优先复用生成类型，否则从 proto 导出 TS-facing types
   - request transport / contract handling / semantic normalization 保持在 request layer 或 adapter boundary
10. 既有代码影响较大时，优先用 code graph 确认 impact scope / callers / ownership boundaries。
11. 如执行中发现新的结构信息，更新 `artifacts/code-context.md`。
12. 默认串行执行，除非计划明确标注某些 work unit 可并行。
13. 持续更新：
   - `execution/changelog.md`
   - `plan/task-board.md`
14. 保持执行过程可观察、可追问、可重定向。
15. 保留 `state.json.loop`，不重置、不改写。
16. 如果需求变化或计划暴露出 gap，立即停下并回退到 `spec` 或 `plan`。
17. 如果是从 verify / review 失败回流而来，继续当前活动 workflow run，不当作全新任务重来。
18. 本阶段不宣称完成。

## 输出格式

- 必须产出：
  - `docs/requests/<request-id>/execution/changelog.md`
  - 与计划任务对应的代码改动
  - 更新后的 `plan/task-board.md`
  - 更新后的 `artifacts/code-context.md`（如结构理解有变化）
- 最终交付物应包含：
  - 实现变更
  - 执行记录
  - 任务状态更新
  - TDD 执行证据或合理例外记录
  - 必要的 clean-code 重构
  - API / graph / adapter 相关实现与记录

## 验收标准

- 已批准的实现任务已完成。
- `execution/changelog.md` 已记录关键决策与偏差。
- 对可测试行为，已记录 test-first 执行步骤或合理例外。
- 改动区域可读性仍然达标，必要的 clean-code 重构已完成或显式记录。
- 引入的 pattern 仍能回溯到已批准问题陈述。
- `plan/task-board.md` 已反映完成状态。
- `state.json.stage=verify`，且保留原有 `loop`。
- 已留下足够证据，使 `verify` 能立即接续。

## 安全边界

- 不能发明新需求。
- 不能猜 plan 未定义的产品行为。
- 不能在 spec / plan 冲突时擅自选一种解释实现。
- 不能无批准地本地重写服务端 TS contract types。
- 不能在无 adapter boundary 的情况下改名非 TS 后端字段。
- 不能跳过任务跟踪。
- 不能宣称请求完成。
- 对可测试行为，不能无说明地跳过 TDD。
- 不能把 spec / plan 当作可选参考。
- 不能在已触碰区域保留明显误导命名、隐藏副作用、重复业务规则而不处理。
- 不能在编码时私自发明新 pattern layer。
- 不能对 trivial / unplanned work 擅自启用 workflow-style parallel execution。
- 不能把关键进展与上下文切换藏在仓库工件之外。
