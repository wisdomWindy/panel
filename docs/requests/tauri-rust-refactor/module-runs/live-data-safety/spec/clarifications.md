# Live Data And Safety Clarifications

## Confirmed Decisions

### Q1 Blacklist Import And Export Schema

- question: 黑名单导入导出的 CSV 列如何定义？
- answer: 用户于 2026-09-14 回复“按建议继续”。
- final decision: 导入 UTF-8 CSV `uid,username,reason`；导出追加 `created_at`。
- affected spec area: LDS-07、Blacklist、Export、CSV tests。
- grounding status and required source: `confirmed-decision`; owner: user。

### Q2 Sensitive-Word Hit Action

- question: PRD 同时出现屏蔽/替换/仅记录与“不进入展示/TTS/AI”，最终 UI 是否提供 action selector？
- answer: 用户于 2026-09-14 回复“按建议继续”。
- final decision: 每次命中都阻断展示、TTS 和 AI，并保留过滤日志；不增加 action selector。
- affected spec area: LDS-04、event pipeline、sensitive form、negative tests。
- grounding status and required source: `confirmed-decision`; owner: user。

### Q3 Captain Contribution Ranking

- question: 舰长贡献榜按等级、次数还是金额？
- answer: 用户于 2026-09-14 回复“按建议继续”。
- final decision: 按本范围内累计舰长首开/续费人民币金额降序。
- affected spec area: LDS-11、ranking DTO/aggregator/display/CSV。
- grounding status and required source: `confirmed-decision`; owner: user。

### Q4 Dashboard Point Detail

- question: 点击/悬浮图表时间点显示哪些明细？
- answer: 用户于 2026-09-14 回复“按建议继续”。
- final decision: 显示该时间点的五项看板指标。
- affected spec area: LDS-09、TrendPoint、chart tooltip/detail。
- grounding status and required source: `confirmed-decision`; owner: user。

### Q5 Unspecified System Copy

- question: PRD 未给出的空态、确认和成功文案是否允许补齐？
- answer: 用户于 2026-09-14 回复“按建议继续”。
- final decision: 可以使用简洁中文系统文案；PRD 已给出的原文必须逐字保留，不添加营销、帮助或升级文案。
- affected spec area: closed-world copy allowlist、UI boundary states。
- grounding status and required source: `confirmed-decision`; owner: user。

## Source-Derived Technical Decisions

### C1 Technical Query And Event Names

- question: PRD 未命名但页面完整运行必需的 list/detail/import/progress Commands/Events 如何命名？
- answer: 使用本 spec API 表中的闭合 snake_case 名称，保持 gateway 与 Rust serde 一一对应。
- final decision: 这些名称是可逆、非产品可见的技术合同，不新增行为；architecture-design 可在不改变 payload/语义的前提下收敛名称。
- affected spec area: API And Data Contracts。
- grounding status and required source: `source-derived` from PRD 的 Tauri Command/Event 边界和必需页面流程；无需新的产品 source。

### C2 Sensitive Import Serialization

- question: PRD 要求 txt/csv/json，但未定义字段序列化。
- answer: 所有格式必须无损保留已批准的 word/category/match/status，禁止引入默认业务值。
- final decision: CSV/JSON 使用四字段；TXT 每行一个 JSON object。该选择只定义 round-trip codec，不改变词规则；若产品方未来提供格式规范，回滚 spec/adapter。
- affected spec area: Sensitive Words、codec tests。
- grounding status and required source: safe `source-derived` technical serialization; no product behavior expansion。

### C3 External Bilibili Production Contract

- question: 正式凭证下发、端点配置和完整签名/错误映射合同在哪里？
- answer: PRD 未提供，现有 Electron 实现只能作为迁移代码事实，不能升级为产品/部署权威。
- final decision: Rust 定义 typed transport port；生产配置缺失时返回 `external_service_unavailable`/credential unavailable，测试使用 injected fake。不得声明生产连接验收通过。
- affected spec area: Room transport、security、handoff、risk。
- grounding status and required source: `missing-source` external integration; required source: 部署方提供正式开放平台配置/合同。该缺口不阻断本地能力，但阻断生产 live-connect claim。

## Open Questions

- 无会改变当前本地模块 scope、产品行为、字段、权限、流程或验收的开放问题。
- 外部 Bilibili 生产合同保持待补；到达后只回滚 Room infrastructure adapter 的 spec/architecture/plan，不改变已批准领域和 UI 语义。
