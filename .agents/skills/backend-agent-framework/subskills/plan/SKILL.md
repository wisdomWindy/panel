---
name: backend-agent-framework-plan
description: Stage subskill for planning. Convert the approved backend spec into execution tasks, dependencies, and verification strategy.
---

# Plan Subskill

## 触发场景

- 当主 `backend-agent-framework` 将请求路由到 `stage=plan` 时使用。
- 适用于把已批准 spec 转成执行任务、依赖关系、验证策略的场景。

## 必要输入

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/spec/clarifications.md`
- `docs/requests/<request-id>/requirements/requirement-map.md`（如存在）
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/artifacts/code-context.md`（如存在）

## 执行步骤

1. 先读：
   - `docs/requests/<request-id>/state.json`
   - `docs/requests/<request-id>/spec/spec.md`
   - `docs/requests/<request-id>/spec/clarifications.md`
   - `docs/requests/<request-id>/requirements/requirement-map.md`（如存在）
   - `docs/requests/<request-id>/requirements/modules/` 下相关模块文件（如存在）
   - `docs/requests/<request-id>/artifacts/code-context.md`（如存在）
   - `../../references/state-machine.md`
   - `../../references/templates/plan.md`
   - `../../references/templates/code-context.md`
   - `../../references/policies/design-patterns.md`
   - `../../references/policies/code-graph.md`
   - `../../references/policies/doc-writing.md`
   - `../../references/policies/clean-code.md`
   - `../../references/policies/spec-constraints.md`
   - `../../references/policies/testing.md`
   - `../../references/policies/mysql-enterprise.md`
   - `../../references/policies/redis-enterprise.md`
   - `../../references/policies/kafka-enterprise.md`
   - `../../references/policies/grpc-enterprise.md`
   - `../../references/policies/k8s-enterprise.md`
2. 进入本阶段前，要求 `approvals.spec_approved=true`；默认允许由上游 spec 阶段在同一 active run 中隐式设置该标记。
3. 产出：
   - `plan/plan.md`
   - `plan/task-board.md`
4. 默认使用中文写计划工件。
5. 所有任务都必须回溯到已批准 spec。
6. 保持 spec 颗粒度；若 spec 太粗，必须回退到 spec。
7. 把任务拆到 `execute` 无需猜测的粒度：
   - handler/controller
   - service/domain flow
   - repository/store
   - job/consumer/subscriber
   - contract and compatibility handling
   - migration / rollback
   - observability
   - verification
8. 如果任何产品细节仍未知，显式写成 blocking clarification 或 approved assumption。
9. 对可测试行为按 TDD 导向拆任务，并在实现前定义验证路径。
10. 对接口对接工作，拆成：
   - contract-source 确认
   - compatibility / version strategy
   - request or handler layer
   - service/domain flow
   - persistence / migration
   - observability
   - verification
11. 对发布性工作，必须拆出：
   - rollout / gray release strategy
   - migration sequencing
   - monitoring window and success signals
   - rollback trigger and rollback operator steps
12. 对安全敏感工作，必须拆出：
   - auth/authz verification
   - secret/credential handling checks
   - PII or sensitive-data exposure checks
   - abuse or replay regression checks
13. 若使用 MySQL、Redis、Kafka、gRPC 或 Kubernetes，必须拆出对应中间件或运行时专项任务：
   - MySQL：索引/SQL 变更、事务与锁分析、migration rehearsal、回滚限制验证
   - Redis：key/TTL 设计、缓存一致性策略、容量与热点风险、降级与失效验证
   - Kafka：topic/partition/key 设计、consumer retry/DLQ、幂等消费、积压与回溯验证
   - gRPC：proto/IDL 变更、兼容性校验、deadline/retry 所有权、metadata/auth 透传、错误码与流式验证
   - Kubernetes：workload 与配置变更、probe/drain 验证、资源与扩缩容策略、灰度发布与回滚演练
14. 把 clean-code guardrails、pattern decisions、side-effect boundaries、ownership 写进计划。
15. 既有代码影响面较大时，优先用 code graph 明确 impact scope，并更新 `artifacts/code-context.md`。
16. 区分串行任务与可并行任务，仅在确实值得时建议 workflow-style parallel execution。
17. `plan/plan.md` 必须使用利于人眼快速扫描的结构。
18. 每个任务都必须附一个 Mermaid 流程图。
19. 保留 `state.json.loop`，不重置、不改写。
20. 本阶段不写实现代码。
21. 默认自动推进：如果用户没有明确要求在 plan 后停下审批，且不存在会影响执行顺序或实现正确性的真实阻塞决策，则直接设置 `approvals.plan_approved=true` 并切到 `stage=execute`。
22. 只有在以下情况才请求用户审批并停在当前 gate：
   - 用户明确要求 plan checkpoint
   - 存在无法通过上下文恢复的关键产品/架构/发布决策
   - 存在高风险迁移、回滚或外部协作条件，继续执行会造成明显误实施风险

## 输出格式

- 必须产出：
  - `docs/requests/<request-id>/plan/plan.md`
  - `docs/requests/<request-id>/plan/task-board.md`
  - `docs/requests/<request-id>/artifacts/code-context.md`（如需要结构分析）

## 验收标准

- `plan/plan.md` 已存在并覆盖任务、依赖、影响面、测试、回滚。
- 每个任务都有对应流程图。
- `plan/task-board.md` 已存在并反映执行单元。
- 所有验收标准都已映射到执行任务和验证任务。
- 接口、兼容性、迁移、观测项已拆解为非黑盒任务。
- rollout、灰度、回滚和安全验证项已拆解为非黑盒任务。
- 若使用 MySQL、Redis、Kafka、gRPC 或 Kubernetes，专项风险与验证任务已拆解为非黑盒任务。
- plan 已被隐式或显式审批。
- `state.json.approvals.plan_approved=true`，`state.json.stage=execute`，且保留原有 `loop`。

## 安全边界

- 不能写代码。
- 不能创建 spec 未授权的任务。
- 在默认自动推进模式下，不能因为缺少手动继续指令而停在 plan；只有真实 checkpoint gate 才能阻止进入 execution。
- 对可测试行为，不能把测试设计拖到实现之后。
- 不能把迁移、兼容性、回滚和观测面留空。
