# 验证记录

## 验证结论

- 验证结果：通过
- spec constraint compliance: pass

## 验收项覆盖

| 验收项 | 验证方式 | 结果 | 证据 |
|---|---|---|---|
| AC-01 `pnpm-workspace.yaml` 已新增公共依赖 catalog | 代码检查 | 通过 | `pnpm-workspace.yaml` 已新增 `catalog`，包含 4 个公共依赖项 |
| AC-02 两个子项目共同使用的 4 个重复依赖均改为 `catalog:` 引用 | 代码检查 | 通过 | `packages/bilibili-live-tool/package.json` 与 `packages/bilibili-live-tool-homepage/package.json` 中 4 个依赖均为 `catalog:` |
| AC-03 子项目依赖归属未改变，只统一版本来源 | 代码检查 | 通过 | 两个子项目仍保留自己的 `devDependencies` 声明，仅版本字面量改为 `catalog:` |
| AC-04 `pnpm install` 能通过 | 命令验证 | 通过 | `CI=true pnpm install --no-frozen-lockfile` 成功完成，并更新 `pnpm-lock.yaml` |
| AC-05 至少一个相关构建命令能通过，证明依赖链路未被破坏 | 命令验证 | 通过 | `pnpm homepage:build` 构建成功 |

## 关键证据

### 1. catalog 配置证据

- `pnpm-workspace.yaml` 已新增：
  - `@tailwindcss/postcss`
  - `autoprefixer`
  - `postcss`
  - `tailwindcss`

### 2. 子项目引用证据

- `packages/bilibili-live-tool/package.json`
  - `@tailwindcss/postcss`: `catalog:`
  - `autoprefixer`: `catalog:`
  - `postcss`: `catalog:`
  - `tailwindcss`: `catalog:`
- `packages/bilibili-live-tool-homepage/package.json`
  - `@tailwindcss/postcss`: `catalog:`
  - `autoprefixer`: `catalog:`
  - `postcss`: `catalog:`
  - `tailwindcss`: `catalog:`

### 3. lockfile 证据

- `pnpm-lock.yaml` 已新增 `catalogs.default`
- 两个 importer 中对应依赖的 `specifier` 已更新为 `catalog:`

## 命令验证记录

### `pnpm install`

- 首次直接执行在无 TTY 环境下被 pnpm 保护中断，不属于配置错误
- 改为 `CI=true pnpm install --no-frozen-lockfile` 后成功
- 说明：
  - `CI=true` 用于适配无交互环境
  - `--no-frozen-lockfile` 用于允许 lockfile 根据 catalog 新 specifier 更新

### `pnpm homepage:build`

- 执行成功
- 输出结论：`build complete`

## TDD / 测试策略核对

- 本次变更为 workspace 依赖配置治理，不适合以单元测试做 red -> green
- 已按 plan 使用安装验证 + 构建验证作为替代证据
- 该例外已在 `execution/changelog.md` 记录

## 规格约束符合性

1. 未将公共依赖抬升到根 `package.json`
2. 未使用 `pnpm.overrides`
3. 仅覆盖当前真实重复的 4 个依赖
4. 子项目依赖归属保持不变
5. 未引入额外依赖治理脚本

结论：spec constraint compliance: pass
