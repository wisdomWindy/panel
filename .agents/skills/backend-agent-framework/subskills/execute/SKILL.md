---
name: backend-agent-framework-execute
description: Stage subskill for execution. Implement only approved backend tasks and keep execution records aligned with the plan.
---

# Execute Subskill

## 触发场景

- 当主 `backend-agent-framework` 将请求路由到 `stage=execute` 时使用。
- 适用于按已批准 spec 和 plan 落地后端实现的阶段。

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
   - `../../references/policies/backend-architecture.md`
   - `../../references/policies/backend-components.md`
   - `../../references/policies/testing.md`
   - `../../references/policies/mysql-enterprise.md`
   - `../../references/policies/redis-enterprise.md`
   - `../../references/policies/kafka-enterprise.md`
   - `../../references/policies/grpc-enterprise.md`
   - `../../references/policies/k8s-enterprise.md`
2. 执行前要求 `approvals.plan_approved=true`。
3. 仅实现已批准任务。
4. 把 spec 与 plan 视为唯一实现合同。
5. 默认使用中文更新 `execution/changelog.md` 与其他执行工件；代码标识、协议名、字段名、错误码、路径等源标识保持原文。
6. 将 `plan/task-board.md` 视为当前 execute 阶段的完整待办清单：只要仍有已批准且非阻塞的任务未完成，就继续在同一 active run 中执行，不得只完成一个任务就停下等待手动继续。
7. 如果 plan 缺少关键接口、状态、流程、迁移、回滚或观测要求，停止执行并回退到 `plan`。
8. 如果 spec 与 plan 在行为颗粒度或产品含义上冲突，停止执行并回退修复工件。
9. 对可测试行为按 red -> green -> refactor 执行。
10. 实现过程中应用 clean-code 规则和已批准的 pattern 决策。
11. 对接口、事件和存储契约：
   - 优先复用权威契约来源
   - 明确兼容性边界
   - 保持入站与出站契约语义一致
12. 对迁移、幂等、重试、超时、外部依赖隔离和观测点要显式落地，不得隐含。
13. 对安全敏感面：
   - auth/authz 逻辑与权限边界要显式落地
   - secret、token、credential 不得泄漏到错误位置、日志或不受控存储
   - PII / 敏感数据暴露面要显式控制
   - abuse / replay / privilege escalation 风险点要按 plan 落地防护
14. 若实现涉及 MySQL、Redis、Kafka、gRPC 或 Kubernetes，必须显式落实对应专项约束：
   - MySQL：事务边界、索引与 SQL 路径、锁冲突面、migration 可执行性
   - Redis：key/TTL、失效策略、一致性边界、热点与穿透防护
   - Kafka：producer key/partition 语义、consumer retry/DLQ、重复消费防护、积压处理
   - gRPC：proto 契约复用、兼容性保真、deadline/metadata 透传、错误码映射、流式与取消语义
   - Kubernetes：probe/lifecycle 语义、资源限制、配置与 secret 边界、graceful shutdown、rollout 与回滚路径
15. 既有代码影响较大时，优先用 code graph 确认 impact scope / callers / ownership boundaries。
16. 如执行中发现新的结构信息，更新 `artifacts/code-context.md`。
17. 持续更新：
   - `execution/changelog.md`
   - `plan/task-board.md`
18. 只有当全部已批准任务完成，或者遇到真实 blocker / requirement_changed / plan gap 时，才允许离开 execute。
19. 保留 `state.json.loop`，不重置、不改写。
20. 如果需求变化或计划暴露出 gap，立即停下并回退到 `spec` 或 `plan`。
21. 如果是从 verify / review 失败回流而来，继续当前活动 workflow run。
22. 本阶段不宣称完成。

## 输出格式

- 必须产出：
  - `docs/requests/<request-id>/execution/changelog.md`
  - 与计划任务对应的代码改动
  - 更新后的 `plan/task-board.md`
  - 更新后的 `artifacts/code-context.md`（如结构理解有变化）

## 验收标准

- 已批准的实现任务已完成。
- `execution/changelog.md` 已记录关键决策与偏差。
- 对可测试行为，已记录 test-first 执行步骤或合理例外。
- 契约、迁移、幂等、重试和观测约束已在改动中得到体现或显式记录。
- 安全约束与 rollout / rollback 实施约束已在改动中得到体现或显式记录。
- 若使用 MySQL、Redis、Kafka、gRPC 或 Kubernetes，对应中间件约束已在改动中得到体现或显式记录。
- `plan/task-board.md` 已反映完成状态。
- 不存在仍可继续执行却被无故暂停的已批准任务。
- `state.json.stage=verify`，且保留原有 `loop`。

## 安全边界

- 不能发明新需求。
- 不能猜 plan 未定义的产品行为。
- 不能在 spec / plan 冲突时擅自选一种解释实现。
- 不能无说明地跳过 TDD、兼容性、迁移或回滚约束。
- 不能在仍有未完成且非阻塞的已批准任务时因为阶段总结而停止。
