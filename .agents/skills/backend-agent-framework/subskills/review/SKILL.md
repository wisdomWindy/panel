---
name: backend-agent-framework-review
description: Stage subskill for review. Review correctness, standards compliance, and residual backend risk before completion.
---

# Review Subskill

## 触发场景

- 当主 `backend-agent-framework` 将请求路由到 `stage=review` 时使用。
- 适用于在 completion 前对代码、spec、plan、verification 输出做最终审查的阶段。

## 必要输入

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/verification/verification.md`
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/artifacts/code-context.md`（如存在）

## 执行步骤

1. 先读：
   - `docs/requests/<request-id>/state.json`
   - `docs/requests/<request-id>/spec/spec.md`
   - `docs/requests/<request-id>/plan/plan.md`
   - `docs/requests/<request-id>/verification/verification.md`
   - `docs/requests/<request-id>/artifacts/code-context.md`（如存在）
   - `../../references/state-machine.md`
   - `../../references/templates/review.md`
   - `../../references/templates/release-readiness.md`
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
2. 先读变更后的代码，再开始评审。
3. 产出 `review/review.md`。
4. 同时产出 `release/release-readiness.md`。
5. 默认使用中文写工件；代码标识、协议名、字段名、错误码、路径等源标识保持原文。
6. 把评审结论分成：
   - 阻塞问题
   - 非阻塞问题
   - 已接受风险
   - 后续事项
7. 既看功能正确性，也看 clean-code、后端边界、契约忠实度、数据所有权、事务与一致性、重试与补偿、副作用扩散、可观测性和迁移风险。
8. 若使用 MySQL、Redis、Kafka、gRPC 或 Kubernetes，必须显式审查对应中间件架构与企业规范是否被满足。
9. 必须显式审查安全面，并写出 `通过|失败|不适用`：
   - authn / authz
   - secret / credential handling
   - PII / sensitive-data exposure
   - abuse / replay / privilege escalation surfaces
10. 检查引入或修改的 pattern 是否真的必要、是否与问题匹配。
11. 依据 spec 中声明的设计约束审查实现，不做脱离工件的主观审美评审。
12. 检查 spec 与 plan 是否保持同一 function-complete behavior granularity；若有漂移，按 blocker 处理。
13. 在 `release/release-readiness.md` 中明确写出：
   - rollout 策略
   - 迁移顺序
   - 监控观察窗
   - 回滚触发条件与执行路径
   - 剩余发布风险
   - `结果: 通过|失败`
14. 明确写出：
   - `整洁代码评估: 通过|失败`
   - `设计模式评估: 通过|失败`
   - `安全评估: 通过|失败|不适用`
15. 如果结论依赖真实依赖方向、ownership boundary、side-effect spread、abstraction fan-out，优先使用 code graph 证据。
16. 如果 review 扩展或推翻了早先结构理解，更新 `artifacts/code-context.md`。
17. 保留 `state.json.loop`，不重置、不改写。
18. 若仍有 blocker，记录后交回主 skill 回流到 `execute`。
19. 在 blocker 未清零前，不允许标记 complete。

## 输出格式

- 必须产出：
  - `docs/requests/<request-id>/review/review.md`
  - `docs/requests/<request-id>/release/release-readiness.md`

## 验收标准

- `review/review.md` 已存在并按严重级别记录发现项。
- 已明确说明变更区域是否仍有 clean-code blocker。
- 已明确说明 pattern choice 是合理 / 过度设计 / 不必要。
- `review/review.md` 已明确写出 `整洁代码评估: 通过|失败` 与 `设计模式评估: 通过|失败`。
- `review/review.md` 已明确写出 `安全评估: 通过|失败|不适用`。
- `release/release-readiness.md` 已明确写出 `结果: 通过|失败`。
- 已明确说明 spec 与 plan 是否保持所需行为颗粒度对齐。
- 若使用 MySQL、Redis、Kafka、gRPC 或 Kubernetes，已明确说明对应中间件企业规范是否满足。
- 所有 blocker 都已清零，否则必须回流执行。
- 通过时 `state.json.stage=complete`，且保留原有 `loop`。

## 安全边界

- 不能把 review 压缩成一句批准语。
- 不能因为“功能能跑”就忽略严重的可维护性问题。
- 不能把 clean-code 或 design-pattern 合规性当成默认成立而不写 `通过|失败`。
- 在 blocker 未解决前，不能宣称 merge-ready。
