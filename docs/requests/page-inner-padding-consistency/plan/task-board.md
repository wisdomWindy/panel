# 任务板：page-inner-padding-consistency

| ID | 任务 | 状态 | Ready | 前置 | 文件/Owner | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | 根级边距合同 RED | done | ready | approved spec | `shell-components-contract.test.ts` | focused Vitest 失败 |
| T2 | 删除四页重复根 padding | done | ready | T1 RED | 四个 route `index.vue` | focused GREEN + `rg` |
| T3 | 工程、运行时与评审 | done | ready | T2 GREEN | scoped tests/type/lint/build/runtime/diff | 四路由、两视口 |

全部任务串行，S1 local / scoped context / compact artifacts；无并行 agent、API、类型、状态或业务语义工作。
