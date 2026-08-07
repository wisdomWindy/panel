# 当前项目优化评估规格

## delivery unit identifier

`project-optimization-assessment`（请求级、非拆分交付单元）

## background and goals

基于 `SRC-001` 对当前仓库做工程评估，输出有事实依据、可排序、可落地的优化建议。交付物是评估报告，不包含生产代码修改。

## in scope

1. monorepo 根配置、package scripts、依赖目录与锁文件。
2. Electron 主应用的 main / preload / renderer 边界、TypeScript 与构建配置。
3. 官网 package 的工程闭环。
4. 测试、lint、type-check、build 与 CI 质量链路。
5. 文档与仓库实际结构的一致性。
6. 对关键代码链路做有界抽样，以验证架构、类型、安全、性能和可维护性信号。

## out of scope

- 修改生产代码、测试、配置或产品行为。
- 新增产品功能、页面、字段、交互或 API 语义。
- 无证据的全量安全审计、性能基准或运行时兼容性承诺。
- 对每个源文件逐行审查。

## trigger and start conditions

用户直接提出 `根据当前项目提出优化建议`；仓库可读且允许执行非破坏性命令。

## requirement split summary

不适用：direct assessment request；请求级单一交付单元，不进入 PRD-only requirement splitting。

## source grounding and traceability

| Spec item | 标签 | 产品语义 | 原始依据 | 下游义务 |
| --- | --- | --- | --- | --- |
| 对当前项目提出建议 | source-backed | 否 | SRC-001 | 报告只讨论当前仓库 |
| 建议需引用具体证据 | source-derived | 否 | “当前项目”要求针对性；所有合理解释均要求查看当前项目 | 每项建议记录文件/命令证据 |
| 建议需有优先级与验证方式 | source-derived | 否 | 优化建议若不可排序/验证无法指导落地 | 报告给出优先级、动作、验证 |
| 不直接修改生产代码 | source-derived | 否 | 用户只要求“提出建议”，未授权实施 | execute 仅形成评估工件 |

Closed-world allowlist：仅允许输出已有工程面的改进建议与实施参考；不授权任何新产品字段、文案、控件、状态、权限、导航、接口、埋点或业务验收行为。

## workflow efficiency profile

- `S3 broad`：跨 package、进程边界、工具链与测试体系。
- artifact density：full。
- context breadth：full，但采用“配置基线 → 命令信号 → 风险驱动代码抽样”，避免无目的全仓逐文件扫描。
- 若发现必须运行应用、访问外部服务或获取凭据才能证实的问题，标记为待验证，不扩大权限。

## self-healing completion bar

本次不修改实现代码，但交付前必须检查：建议与证据一致、命令结果如实记录、建议无重复/矛盾、优先级与风险匹配、无产品范围扩张；verification 与 review 必须分别给出 framework-required verdicts，remaining blockers 为 none。

## user intent contract

- literal request：根据当前项目提出优化建议。
- practical goal：获得能指导团队排期的针对性清单。
- success criteria：每项高/中优先建议至少包含证据、影响、动作和验证。
- forbidden interpretations：泛化最佳实践堆砌；直接重构；把猜测写成已确认问题；只做文字润色而不检查项目。
- acceptable approaches：静态审查、非破坏性质量命令、风险驱动抽样。
- verification / review：抽查引用可定位；命令状态可复现；事实与建议强度匹配。

## user flow

不适用：不修改用户可见流程。评估流程为“收集事实 → 分类风险 → 排序建议 → 用户选择后续实施项”。

## page and module design

不适用：不设计页面或生产模块。

## UI / UX design contract

不适用：不修改 UI / UX。

## frontend styling constraints

不适用：不修改样式。

## field display semantics and interaction visual-state contracts

不适用：不触碰业务展示字段或交互状态。

## function-complete behavior breakdown

1. **工程基线**：识别 workspace、package、运行时版本、主要脚本、构建与发布配置。
2. **质量命令**：运行或解释 lint、type-check、test、build 的实际可用性；命令不存在或失败本身作为证据。
3. **结构评估**：检查 Electron 安全边界、前后端契约/IPC、状态与服务分层、官网独立质量闭环。
4. **规模与热点抽样**：通过文件规模、重复/宽类型/调试输出/TODO 等信号定位高风险文件，再读取关键上下文。
5. **依赖与交付**：检查依赖版本策略、CI、打包元数据、文档命令与仓库一致性。
6. **建议报告**：按 P0/P1/P2 或等价优先级输出事实、影响、动作、投入与验证。

## expert frontend engineering constraints

不改变用户旅程或状态流；只在仓库事实显示风险时提出对应改进，并区分“确认问题”“风险信号”“需运行时验证”。

## frontend architecture design routing inputs

`architecture-design_required: false`。本次不批准模块边界变更，只把潜在架构改进列为后续候选；具体实施须另开请求并重新做架构设计。

## production code quality constraints

不适用：本次不添加或修改生产代码。

## human review readiness constraints

只新增本请求的 `docs/requests/project-optimization-assessment/` 工件；不得混入或修复当前工作树中已有的 `pi-code-agent.zip` 删除状态。报告需标记实际运行命令、失败与未运行项。

## functional-programming constraints

不适用：不修改规则、转换或副作用编排。

## architecture reuse and shared ownership constraints

不适用：只识别候选，不在本请求中抽取或迁移共享逻辑。

## locality and file-extraction constraints

不适用：不新增生产 helper / hook / mapper / utility。

## Vue component extraction constraints

不适用：不修改 Vue SFC 边界。

## design constraints

- 结论强度必须匹配证据强度。
- 优先解决会阻断质量反馈、发布可信度或安全边界的问题，再考虑结构美化。
- 同一根因不拆成多个重复建议刷高数量。
- 投入评估只给相对等级，除非有足够证据支持精确工期。

## project bootstrap and scaffold decision

不适用：不是 greenfield 请求。

## change axes and pattern decision

- concrete problem：生成一次性的仓库工程评估报告。
- candidate signals：无生产运行时选择、适配、创建、状态或副作用协调变更。
- depth：Level 0。
- decision：direct code / none。
- syntax shape：Markdown 工件与简单命令证据。
- rationale：引入评估框架类、manager 或新工具层会增加无收益结构。
- later signal：若团队要周期性自动执行本评估，再单独设计可复用审计脚本或 CI workflow。

## code context and impact assumptions

- 以实际仓库为准，不以 README 结构描述替代代码事实。
- 使用定向搜索、文件统计、配置读取及现有命令；结构影响分析以 import/调用与进程边界抽样为主，不修改代码，故无需安装 code graph。

## API and data contracts

不适用：不新增或修改后端 / IPC 合同；可将现有合同风险作为建议证据。

## frontend / server responsibility boundary

Electron main/preload/renderer 边界仅作为工程审查对象；本请求不重新分配职责。

## context and dependency sources

根 package、workspace、两个 package、TypeScript/Vite/ESLint/CI 配置、测试目录、Electron 入口和 preload、代表性 renderer services/stores/views，以及命令输出。

## edge cases

- 命令因环境、缺少脚本或依赖失败：记录原始失败类别，不误判为代码行为失败。
- 已有未提交改动：不触碰、不归因于本次请求。
- 文档与代码不一致：按事实记录，避免推测历史原因。
- 未经运行时验证的性能/安全结论：降级为风险建议。

## acceptance criteria

- AC-01：报告至少覆盖工程闭环、架构/安全、测试、依赖/构建、文档五类；无事实信号的类别可明确“未发现高优先项”。
- AC-02：每个 P0/P1 建议引用至少一个可定位仓库证据或命令结果。
- AC-03：每项建议说明影响、建议动作、相对投入和验证方式。
- AC-04：报告明确区分确认问题、风险信号和待验证项。
- AC-05：不修改生产代码，不新增产品语义。
- AC-06：verification 与 review 工件记录所有适用 verdict 为 pass，且 state 达到 complete。

## human review and handoff

用户收到报告后选择实施批次；每个实际代码批次需建立新的变更请求，不能直接把建议视为已批准实现合同。

## risks

- 仓库规模较大，抽样可能遗漏低频问题；通过质量命令和热点信号降低风险。
- Electron 运行时与外部 Bilibili 服务未必可在当前环境完整验证；相应结论必须标记限制。
- 当前工作树存在用户已有删除项，必须避免误操作。
