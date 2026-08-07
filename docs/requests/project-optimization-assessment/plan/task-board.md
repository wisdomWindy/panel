# Task Board

| ID | 任务 | 状态 | execute-readiness | 模式 | 前置条件 | 规格映射 | 范围 / 目标 | 来源与 product delta | 测试切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T-01 | 工程基线与结构事实 | completed | ready | 串行（内部只读可并行） | spec approved | function-complete 1 | workspace、packages、TS、CI、README | source-backed + code-fact-backed；none | 配置路径与脚本可定位 |
| T-02 | 质量与构建命令 | completed | ready | 串行 | T-01 | function-complete 2 | lint/type-check/test/build | source-derived + code-fact-backed；none | 退出码、关键输出、未运行原因 |
| T-03 | 风险驱动源码抽样 | completed | ready | 串行（搜索可并行） | T-01、T-02 | function-complete 3～5 | main/preload/renderer、依赖、发布 | source-derived + code-fact-backed；none | 上下文读取与分类证据 |
| T-04 | 排序建议与执行记录 | completed | ready | 串行 | T-01～T-03 | function-complete 6、AC-01～AC-05 | `execution/changelog.md` | source-backed + source-derived；none | AC 映射与引用抽查 |

## 执行说明

- speed profile / context budget：`S3 broad` / full；采用风险驱动抽样，不扫描 `node_modules`。
- 模式约束：Level 0 direct code；不引入脚本框架、manager 或共享抽象。
- API/类型：不修改合同；审查时读取 governing tsconfig 与 scoped IPC declarations。
- 关键状态：确认问题、风险信号、待验证项必须分开。
- 整洁性：实际 diff 仅限本请求目录，不格式化或修改生产文件。
- 待确认项：none；外部运行时无法验证项降级标注。
- 自愈：execute → verify → review；失败按 owning stage 回流并遵守 7/2/3 上限。
