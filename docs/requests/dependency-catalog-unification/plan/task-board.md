# 依赖 catalog 统一任务看板

| 任务 ID | 任务名称 | 状态 | 执行模式 | 执行说明 | 触发/前置条件 | 关联规格区域 | 整洁性约束或注意点 | 模式约束或抽象边界 |
|---|---|---|---|---|---|---|---|---|
| T1 | 在 workspace 层建立公共依赖 catalog | 已完成 | 串行 | 修改 `pnpm-workspace.yaml` 新增公共版本目录 | spec 已批准 | spec 4.1 / 5.1 / 5.4 / 6 / 7.1 / AC-01 / AC-03 | catalog 只放真实重复依赖 | 不改依赖归属 |
| T2 | 把子项目重复依赖改为 `catalog:` 引用 | 已完成 | 串行 | 修改两个子项目 `package.json` | T1 完成 | spec 4.2 / 5.2 / 5.3 / 7.2 / AC-02 / AC-03 | 只改 4 个重复依赖 | 不把单项目依赖误迁入 catalog |
| T3 | 更新 lockfile 并验证解析链路 | 已完成 | 串行 | 运行 `pnpm install` 与 `pnpm homepage:build` | T1 / T2 完成 | spec 8 / 9 / 10 / 11 / AC-04 / AC-05 | 不夹带无关版本升级 | 仅验证，不扩 scope |
