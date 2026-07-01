---
name: backend-agent-framework-verify
description: Stage subskill for verification. Validate the backend implementation against acceptance criteria and record durable evidence.
---

# Verify Subskill

## 触发场景

- 当主 `backend-agent-framework` 将请求路由到 `stage=verify` 时使用。
- 适用于对实现结果做验收、留证据、判断是否可进入 review 的阶段。

## 必要输入

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/execution/changelog.md`
- `docs/requests/<request-id>/state.json`

## 执行步骤

1. 先读：
   - `docs/requests/<request-id>/state.json`
   - `docs/requests/<request-id>/spec/spec.md`
   - `docs/requests/<request-id>/plan/plan.md`
   - `docs/requests/<request-id>/execution/changelog.md`
   - `../../references/state-machine.md`
   - `../../references/templates/verify.md`
   - `../../references/policies/testing.md`
   - `../../references/policies/doc-writing.md`
   - `../../references/policies/api-contracts.md`
   - `../../references/policies/mysql-enterprise.md`
   - `../../references/policies/redis-enterprise.md`
   - `../../references/policies/kafka-enterprise.md`
   - `../../references/policies/grpc-enterprise.md`
   - `../../references/policies/k8s-enterprise.md`
2. 产出 `verification/verification.md`。
3. 默认使用中文写工件；代码标识、协议名、字段名、错误码、路径等源标识保持原文。
4. 将每条验收标准映射到：
   - 验证方法
   - 结果
   - 证据
5. 验证实现是否符合 approved spec，而不是 undocumented intent。
6. 验证 spec 与 plan 是否在实现范围内保持同一 function-complete granularity。
7. 验证可测试行为是否被测试覆盖，若无则检查是否有明确例外说明。
8. 如涉及后端接口、事件或存储契约，验证契约一致性、错误路径、兼容性、幂等、重试、超时、迁移结果与回滚可行性。
9. 必须显式验证安全面，并记录 `通过|失败|不适用`：
   - authentication
   - authorization
   - secret / credential handling
   - PII / sensitive-data exposure
   - abuse / replay / privilege escalation surfaces
10. 必须结合 plan 验证 rollout readiness 所需证据是否齐备，包括：
   - migration sequencing evidence
   - monitoring and alert readiness
   - rollback trigger and operator path
11. 若使用 MySQL、Redis、Kafka、gRPC 或 Kubernetes，必须显式验证对应专项证据：
   - MySQL：migration rehearsal、索引/执行计划、事务与锁风险、回滚边界
   - Redis：key/TTL 正确性、一致性验证、热点/击穿/雪崩防护、降级策略
   - Kafka：topic/key/partition 语义、consumer retry/DLQ、重复消费防护、积压与回溯策略
   - gRPC：proto 兼容性、deadline/timeout 行为、metadata/auth 透传、错误码映射、流式取消与背压语义
   - Kubernetes：probe/readiness 结果、rollout 与 rollback 证据、graceful shutdown/drain、资源与 secret 处理
12. 如 spec 包含 trigger/context/handoff 假设，也要验证实现是否遵守。
13. 在 `verification/verification.md` 中写出明确的 `规格约束符合性: 通过|失败` 与 `安全验证: 通过|失败|不适用`。
14. 保留 `state.json.loop`，不重置、不改写。
15. 若验证失败，明确记录失败原因，交回主 skill 回流到 `execute`。
16. 本阶段不能在没有验证工件的情况下宣称完成。

## 输出格式

- 必须产出：
  - `docs/requests/<request-id>/verification/verification.md`
  - `docs/requests/<request-id>/verification/evidence/` 下的可选证据文件

## 验收标准

- 每条验收标准都已被检查。
- 每个验证项都有附件或引用证据。
- `verification/verification.md` 已明确写出 `规格约束符合性: 通过|失败`。
- `verification/verification.md` 已明确写出 spec-plan 粒度对齐结论。
- 契约一致性、错误路径、幂等、迁移和回滚面已被验证或明确记录为不适用。
- `verification/verification.md` 已明确写出 `安全验证: 通过|失败|不适用`。
- rollout readiness 所需证据已被验证或明确记录缺口。
- 若使用 MySQL、Redis、Kafka、gRPC 或 Kubernetes，对应中间件专项证据已被验证或明确记录缺口。
- 若存在失败，要么已修复，要么已明确回流执行。
- 通过时 `state.json.stage=review`，且保留原有 `loop`。

## 安全边界

- 不能在没有证据时标记完成。
- 不能忽略失败的 acceptance criteria。
- 不能把 `规格约束符合性` 当作默认成立而不写 `通过|失败`。
- 不能在 request/response handling、event contract、schema compatibility 与 approved contract source 冲突时仍判定验证通过。
