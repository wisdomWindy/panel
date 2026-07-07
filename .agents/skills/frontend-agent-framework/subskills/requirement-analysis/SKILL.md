---
name: frontend-agent-framework-requirement-analysis
description: Stage subskill for requirement analysis. Turn intake artifacts into an explicit scope, risk, ambiguity, and splitting-strategy analysis before requirement splitting.
---

# Requirement Analysis Subskill

## 触发场景

- 当主 `frontend-agent-framework` 将 PRD 驱动需求路由到 `stage=requirement-analysis` 时使用。
- 适用于在正式模块拆分前，先把需求范围、非目标、依赖、风险、歧义和拆分依据显式固化的场景。

## 必要输入

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/state.json`
- 原始 PRD 文档或稳定 PRD 引用

## 执行步骤

1. 先读：
   - `docs/requests/<request-id>/state.json`
   - `docs/requests/<request-id>/request.md`
   - `docs/requests/<request-id>/artifacts/prd-snapshot.md`
   - 原始 PRD 文档或稳定 PRD 引用
   - `../../references/framework-overview.md`
   - `../../references/state-machine.md`
   - `../../references/templates/request.md`
   - `../../references/templates/requirement-analysis.md`
   - `../../references/policies/doc-writing.md`
2. 仅用于 PRD 驱动需求；bugfix 输入默认不走本阶段，除非主 skill 明确把缺陷提升为等价需求分析问题。
3. 先消费 intake 工件；如果 `artifacts/prd-snapshot.md` 相比原始 PRD 丢失了关键业务约束、流程、字段、状态或依赖关系，必须回看原始 PRD 或稳定引用。
4. 产出：
   - `analysis/requirement-analysis.md`
5. 在本阶段明确写清：
   - confirmed scope
   - out-of-scope / non-goals
   - source-backed behavior inventory
   - workflow / dependency assessment
   - ambiguity / clarification register
   - risk / constraint assessment
6. 需求分析必须以“理解需求”为目标，而不是提前输出实现方案、文件结构、代码抽象或测试设计。
7. 对上游已经明确给出的字段、表格、展示、交互、状态、流程、依赖、审批、外部约束，不得在分析阶段压缩成过于宽泛的口号式总结。
8. 如果范围内存在多个页面、流程、表格、表单或跨模块规则，必须明确记录它们为什么应该在下一阶段被拆开，或为什么应该保持在一个模块内。
9. 可以在分析阶段给出 `page-design` / `architecture-design` 的 request-level signals，但不能替代 `requirement-splitting` 对具体模块路由的正式判定。
10. 如果发现源文档存在歧义、冲突、遗漏或无从判断的业务语义，必须记录为开放问题；不能靠常识补全。
11. 保留 `state.json.loop`，不重置、不改写。
12. 本阶段不写 `requirements/requirement-map.md`，不写 `spec/plan/execution/verification/review` 工件，不写代码。
13. 完成后把 `stage` 设为 `requirement-splitting`。

## 输出格式

- 必须产出：
  - `docs/requests/<request-id>/analysis/requirement-analysis.md`
- 最终交付物应包含：
  - 需求理解摘要
  - confirmed scope
  - out-of-scope / non-goals
  - 关键行为簇梳理
  - 依赖与约束判断
  - 歧义与开放问题
  - 风险判断
  - 后续 requirement-splitting 的拆分依据
  - `page-design` / `architecture-design` 信号

## 验收标准

- `analysis/requirement-analysis.md` 已存在。
- 当前请求范围与非目标已被显式写清。
- 上游明确行为约束没有被压缩成无法驱动后续拆分的粗摘要。
- 已记录后续拆分的依据与不应合并的边界。
- 已记录开放问题、依赖与风险。
- `state.json.stage=requirement-splitting`。
- `state.json` 保留原有 `loop`。

## 安全边界

- 不能把本阶段退化成 intake 摘要复述。
- 不能跳过范围判断、非目标判断和拆分依据判断。
- 不能把需求分析直接写成实现设计。
- 不能在本阶段生成正式模块工件。
- 不能写 spec、plan 或代码。
- 不能在存在歧义时用实现常识替代显式记录。
