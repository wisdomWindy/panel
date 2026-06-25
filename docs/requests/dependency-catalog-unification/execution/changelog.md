# 执行变更记录

## 本轮执行概览

- 阶段：`execute`
- 执行范围：仅覆盖已批准 spec / plan 中定义的 4 个重复 devDependencies 的 catalog 化
- 执行模式：串行

## 变更记录

### 1. 建立 workspace catalog

- 修改 `pnpm-workspace.yaml`
- 新增 `catalog`
- 收敛以下公共版本来源：
  - `@tailwindcss/postcss`
  - `autoprefixer`
  - `postcss`
  - `tailwindcss`

### 2. 子项目改为 `catalog:` 引用

- 修改 `packages/bilibili-live-tool/package.json`
- 修改 `packages/bilibili-live-tool-homepage/package.json`
- 仅替换上述 4 个重复依赖的版本字面量
- 未调整依赖所属包，也未新增根 `package.json` 公共依赖

## TDD / 测试策略说明

- 本次为 workspace 依赖配置治理，不属于适合先写自动化单测的行为改动。
- 采用的替代验证方式：
  - `pnpm install` 验证 catalog 解析与 lockfile 更新
  - `pnpm homepage:build` 验证相关构建链路

## 偏差与范围控制

- 无规格外实现
- 无额外依赖升级/降级意图
- 若 lockfile 出现变化，需在验证阶段确认未混入无关漂移

## 执行结果

- `pnpm install`：
  - 首次在无 TTY 环境下被 pnpm 保护中断，随后改用 CI 安装参数继续验证
  - 由于当前环境默认 `frozen-lockfile`，改用 `pnpm install --no-frozen-lockfile` 更新 lockfile
  - 在允许联网的执行环境下安装成功，并生成 catalog 对应 lockfile 变更
- `pnpm homepage:build`：
  - 构建成功
- 安装过程附带非阻塞提示：
  - pnpm 提示部分依赖的 build scripts 被忽略，可由后续显式 `pnpm approve-builds` 决定是否放开
