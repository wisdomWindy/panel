# Task Board

| ID | 任务 | 状态 | Ready | 模式 | 前置 | 规格/规则 | 范围 | 文件/符号 | 质量与评审合同 | 状态/交互 | 来源 | 测试 | 假设/待确认 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | 回归测试与排行榜模板修复 | complete | yes | serial/TDD | spec approved | F1/F2, A1-A4 | 排行榜页 | `rankings/index.vue`, `rankings-page.test.ts` | Level 0 direct code；Tailwind utility；单 empty owner；无新依赖/抽取；pre-review clean | empty/loading/data owner 不变；滚动不隐藏 | source-backed U1-U4 | RED/GREEN complete；full checks/runtime 交 verify | SSR 被 Element Plus CSS 外部加载阻断，采用既有 SFC 契约测试并由浏览器补运行证据 |
