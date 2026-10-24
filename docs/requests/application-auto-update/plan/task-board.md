# 任务看板

| ID | 任务 | 状态 | 依赖 | 验证 |
| --- | --- | --- | --- | --- |
| T1 | 新增自动检查、全局弹窗、强制关闭限制、手动检查的失败契约测试 | completed | spec + architecture | focused Vitest red: 4 expected failures |
| T2 | `useReleaseRuntime` 启动后自动检查并隔离失败 | completed | T1 | runtime tests: 4 passed |
| T3 | `App.vue` 全局更新对话框和 store actions | completed | T1, T2 | shell contract: 4 passed; type-check passed |
| T4 | 强制更新禁用所有普通关闭通道 | completed | T1 | release pages test: 3 passed |
| T5 | 设置 About 手动检查、loading 和结果反馈 | completed | T1, T3 | settings contract: 4 passed; type-check passed |
| T5B | 原生检查传播瞬时失败，启动 runtime 单独静默处理 | completed | ADR-05, T2, T5 | Rust mapper test passed; runtime/settings: 8 passed |
| T6 | 完整验证、引用链复查、diff 自审与工件记录 | completed | T2-T5B | focused 23/23; type/lint/build; release 12/12; foundation 20/20; Rust 113/113 |

## 执行约束

- 串行执行，因为所有任务共享 release store/runtime 状态语义。
- 每完成一项即更新状态，不得在末尾批量标记。
- 不得覆盖 `settings-shell-contract.test.ts` 中已有的未提交断言或其他用户改动。
