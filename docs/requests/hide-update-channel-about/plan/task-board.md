# Task Board

| ID | 任务 | 状态 | Execute readiness | 模式 | 文件/符号 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | 移除关于页更新通道展示并闭包清理 | complete | passed | 串行 | settings page/test + zh/en locale | RED/GREEN + refs + type/lint/build/diff |

## Constraints

- Source：U1；Level 0 direct code；S0 narrow context。
- 只删除展示及其孤立文案，保留 release channel 数据与更新检查。
- 无产品假设、API 变化、组件抽取、样式新增或并行工作单元。
