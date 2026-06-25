# 源需求快照

## source summary

- 用户要求“把 packages 中子项目都用到的依赖设置为公共依赖”；
- 在澄清后，用户明确选择方案 2：不挪动依赖边界，只通过 workspace 机制统一版本；
- 技术落点确定为 `catalog` 方案。

## key business goals

1. 把真实重复依赖的版本集中维护，减少多处手工同步；
2. 保持每个子项目自己的依赖归属与可读性；
3. 不把当前任务扩展成全仓库依赖重构。

## relevant modules or pages

- `pnpm-workspace.yaml`
- `packages/bilibili-live-tool/package.json`
- `packages/bilibili-live-tool-homepage/package.json`
- `pnpm-lock.yaml`

## notable open questions from the upstream PRD

1. 是否要把所有依赖都迁入 catalog
   - 当前答案：否，只处理当前真实跨包重复依赖。
2. 是否要把依赖抬到根 `package.json`
   - 当前答案：否，用户已明确选择 `catalog` 而非根公共依赖。
3. 是否要覆盖运行时依赖和开发依赖两类
   - 当前答案：按“真实重复依赖”处理；当前发现的重复项均为 dev 侧样式链路依赖。
