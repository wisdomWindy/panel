---
name: backend-agent-framework-spec
description: Stage subskill for specification. Turn intake artifacts into the formal backend engineering spec and collect clarifications.
---

# Spec Subskill

## 触发场景

- 当主 `backend-agent-framework` 将请求路由到 `stage=spec` 时使用。
- 适用于需要把 intake 结果转成正式后端工程合同的场景。

## 必要输入

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/requirements/requirement-map.md`（PRD 驱动需求必需）
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/artifacts/code-context.md`（如需要结构分析）
- `docs/requests/<request-id>/design/architecture-design.md`（如存在 architecture-design 阶段）

## 执行步骤

1. 先读：
   - `docs/requests/<request-id>/state.json`
   - `docs/requests/<request-id>/request.md`
   - `docs/requests/<request-id>/artifacts/prd-snapshot.md`
   - `docs/requests/<request-id>/requirements/requirement-map.md`（如存在）
   - `docs/requests/<request-id>/requirements/modules/` 下相关模块文件（如存在）
   - `docs/requests/<request-id>/artifacts/code-context.md`（如存在）
   - `docs/requests/<request-id>/design/architecture-design.md`（如存在）
   - `../../references/state-machine.md`
   - `../../references/templates/spec.md`
   - `../../references/templates/code-context.md`
   - `../../references/policies/api-contracts.md`
   - `../../references/policies/clean-code.md`
   - `../../references/policies/code-graph.md`
   - `../../references/policies/design-patterns.md`
   - `../../references/policies/doc-writing.md`
   - `../../references/policies/backend-architecture.md`
   - `../../references/policies/backend-components.md`
   - `../../references/policies/spec-constraints.md`
   - `../../references/policies/mysql-enterprise.md`
   - `../../references/policies/redis-enterprise.md`
   - `../../references/policies/kafka-enterprise.md`
   - `../../references/policies/grpc-enterprise.md`
   - `../../references/policies/k8s-enterprise.md`
2. 对 PRD 驱动需求，必须先消费 `requirements/requirement-map.md` 与对应模块工件；如果存在 architecture-design，则再把它视为上游输入。
3. 把 `artifacts/prd-snapshot.md` 当作结构化抽取来使用；如果它或 requirement-splitting 工件比原始需求源更薄，必须回看原始 PRD/稳定引用。
4. 产出：
   - `spec/spec.md`
   - `spec/clarifications.md`
5. 默认使用中文写工件；代码标识、协议名、字段名、错误码、路径等源标识保持原文。
6. 把行为定义写到 downstream 可执行、可审查、可验证的粒度。
7. 明确把行为写到与 `plan` 对齐的 function-complete granularity：
   - 入站接口要写路径/方法/协议、鉴权、限流、幂等、错误语义
   - 工作流要写触发、步骤、分支、状态流转、补偿、重试、超时
   - 数据模型要写字段、约束、兼容性、迁移要求
   - 出站依赖要写契约来源、调用语义、失败处理、重试边界
   - 观测性要写日志、指标、告警、trace 或审计要求
   - 安全性要写认证、授权、敏感字段、secret 使用边界、滥用面与保护策略
   - 发布性要写 rollout 顺序、灰度/全量策略、迁移执行顺序、监控观察窗与回滚触发条件
8. 如果上游需求源已经给出这些细节，这些信息必须进入 spec，不能因为 intake 摘要粗化而丢失。
9. 如果 requirement-splitting 已经按模块拆分了内容，spec 必须逐模块承接这些功能单元。
10. 如果仍有未知信息，写入 `spec/clarifications.md`，作为显式 clarification 或 approved assumption。
11. 把可维护性、抽象边界、side effect、duplication ownership、命名与模式决策写成 spec constraints。
12. 如涉及接口、事件或存储契约：
   - 识别权威 contract source
   - 记录 direct contract ownership 还是 adapter boundary
   - 记录兼容性、版本化、迁移和回滚要求
13. 对安全敏感需求，明确记录：
   - authn / authz 约束
   - secret 或 token 边界
   - PII / 敏感数据处理要求
   - abuse / replay / privilege escalation 风险与防护
14. 如涉及 MySQL、Redis、Kafka、gRPC 或 Kubernetes，必须把对应中间件或运行时约束写入 spec，包括：
   - MySQL：事务边界、索引策略、锁与并发风险、读写路径、迁移与回滚限制
   - Redis：key 设计、TTL、缓存一致性、穿透/击穿/雪崩防护、分布式锁或限流边界
   - Kafka：topic/partition/key 语义、consumer group、顺序与重试、幂等与重复消费、死信与回溯策略
   - gRPC：proto 权威来源、字段与枚举兼容性、deadline/timeout、metadata/auth 透传、错误码与流式语义
   - Kubernetes：workload 类型、probe 策略、资源与扩缩容、配置与 secret 边界、rollout/drain/rollback 策略
15. 如涉及既有代码结构调整：
   - 优先用 code graph 识别真实边界、依赖方向和 impact scope
   - 如结构理解加深，更新 `artifacts/code-context.md`
16. 不写实现代码。
17. 默认自动推进：如果用户没有明确要求在 spec 后停下审批，且不存在会影响后续实现的真实阻塞决策，则直接把 `approvals.spec_approved=true` 并切到 `stage=plan`。
18. 只有在以下情况才请求用户审批并停在当前 gate：
   - 用户明确要求 spec checkpoint
   - 存在无法通过仓库上下文恢复的关键产品决策
   - 存在高风险范围/契约/迁移问题，继续推进会造成明显误实现风险

## 输出格式

- 必须产出：
  - `docs/requests/<request-id>/spec/spec.md`
  - `docs/requests/<request-id>/spec/clarifications.md`

## 验收标准

- `spec/spec.md` 已存在，且覆盖 scope、flows、modules、contracts、edge cases、acceptance criteria、risks。
- 行为颗粒度已达到与 `plan` 对齐的 function-complete granularity。
- spec 已记录 responsibility boundaries、side-effect placement、duplication ownership、complexity guardrails、pattern use / rejection。
- spec 已记录安全约束与发布/灰度/回滚约束。
- 若使用 MySQL、Redis、Kafka、gRPC 或 Kubernetes，spec 已记录对应企业级架构与运行约束。
- `spec/clarifications.md` 已记录开放问题与已定决策。
- 若存在 requirement-splitting 或 architecture-design 工件，spec 已完整承接其显式行为约束。
- spec 已被隐式或显式审批。
- `state.json.approvals.spec_approved=true`，`state.json.stage=plan`，且保留原有 `loop`。

## 安全边界

- 不能写代码。
- 不能在 spec 不存在前生成实现任务。
- 在默认自动推进模式下，不能因为缺少手动继续指令而停在 spec；只有真实 checkpoint gate 才能阻止进入 `plan`。
- 不能把行为写得含糊到需要实现阶段去猜。
- 不能把重大抽象/协作问题留给下游自己决定。
