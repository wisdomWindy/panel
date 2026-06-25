# 依赖 catalog 统一规格说明

## 1. 背景与目标

当前 monorepo 下的 workspace 子项目为：

- `packages/bilibili-live-tool`
- `packages/bilibili-live-tool-homepage`

经现状检查，当前两个子项目真正重复声明的依赖只有 4 个：

- `tailwindcss`
- `@tailwindcss/postcss`
- `postcss`
- `autoprefixer`

这些依赖目前在两个子项目中以相同版本分别声明。用户希望把它们改成“公共依赖”的管理方式，但不接受把依赖直接抬升到根包，而是明确选择 `catalog` 方案：保留子项目自己的依赖声明，只把版本源集中到 workspace 级别管理。

本次目标就是用 `pnpm-workspace.yaml` 的 `catalog` 集中维护这 4 个公共版本，并让两个子项目改用 `catalog:` 引用。

## 2. 范围

### 2.1 本次包含

1. 在 `pnpm-workspace.yaml` 中新增 `catalog`
2. 将当前真实重复的 4 个依赖统一纳入 `catalog`
3. 更新两个子项目的 `package.json`，将这 4 个版本改为 `catalog:` 引用
4. 更新 lockfile 并验证安装与构建链路

### 2.2 本次不包含

1. 不把运行时依赖整体抬到根 `package.json`
2. 不对“只在单个子项目存在”的依赖做 catalog 化
3. 不进行无关的依赖升级或降级
4. 不顺手清理所有潜在版本不一致问题，只处理当前真实重复项

## 3. 用户流程

### 流程 1：开发者查看依赖版本

1. 开发者在子项目 `package.json` 中看到 `catalog:` 引用
2. 若需确认真实版本，前往 `pnpm-workspace.yaml` 查看 catalog
3. 若未来两个子项目继续共用同一依赖，优先复用现有 catalog 项

### 流程 2：开发者更新公共依赖版本

1. 开发者只需修改 `pnpm-workspace.yaml` 中对应 catalog 项
2. 子项目无需手动同步多个版本号
3. 重新安装后 workspace 使用统一版本

## 4. 模块设计

### 4.1 workspace 配置层

- `pnpm-workspace.yaml`
  - 继续保留现有 `packages` 声明
  - 新增 `catalog`
  - catalog 仅承载当前确实跨包共用的依赖版本

### 4.2 子项目依赖声明层

- `packages/bilibili-live-tool/package.json`
- `packages/bilibili-live-tool-homepage/package.json`

要求：
- 保持依赖归属不变
- 只把重复依赖的版本改为 `catalog:`
- 不把某个子项目实际不依赖的包强行声明进去

## 5. 责任边界与约束

### 5.1 责任与边界约束

1. 根 workspace 只负责公共版本目录，不负责替子项目声明依赖归属
2. 子项目仍然对“自己依赖什么”负责
3. catalog 统一的是“版本来源”，不是“依赖所有权”

### 5.2 命名与领域约束

1. catalog 项命名应直接使用原始依赖包名，避免再造别名
2. 若后续出现额外共用依赖，也应遵循同样命名规则

### 5.3 重复规则约束

1. 仅把两个及以上子项目当前真实共用、且版本应保持一致的依赖纳入 catalog
2. 不为了形式统一，把单项目依赖也一并迁入 catalog

### 5.4 副作用与依赖约束

1. 必须通过 `pnpm install` 验证 catalog 引用能正常解析
2. 需要选择至少一个相关构建命令验证依赖链路未被破坏
3. 不允许通过 overrides 隐式覆盖版本来替代本次 catalog 目标

### 5.5 复杂度与可读性约束

1. 不引入额外依赖治理脚本
2. 不引入根共享 package 之类的新模式层
3. workspace 配置应尽量直白，后续维护者能一眼看懂

## 6. 变化轴与模式决策

### 6.1 变化轴

1. 版本管理从“各包手写重复版本号”转为“workspace catalog 集中管理”
2. 子项目保留依赖边界，但失去重复的具体版本字面量

### 6.2 模式决策

- **采用**：`pnpm-workspace.yaml` `catalog`
- **原因**：
  - 最符合“统一版本但不改变依赖归属”的目标
  - 相比 `overrides`，更显式、更易读
  - 相比根公共依赖，边界更清晰
- **拒绝方案**：
  1. 根 `package.json` 公共依赖方案
  2. `pnpm.overrides` 隐式强制版本方案
  3. 人工约束、不落配置的方案

## 7. 技术契约

### 7.1 workspace catalog 契约

`pnpm-workspace.yaml` 中应存在类似结构：

```yaml
catalog:
  tailwindcss: ^4.3.0
  "@tailwindcss/postcss": ^4.3.0
```

说明：
- 允许根据实际 pnpm 语法做等价表达；
- 但必须能被两个子项目通过 `catalog:` 正常引用。

### 7.2 子项目依赖引用契约

子项目 `package.json` 中的重复依赖应采用：

```json
{
  "devDependencies": {
    "tailwindcss": "catalog:"
  }
}
```

说明：
- 不再在两个子项目中重复写具体版本号；
- 但依赖字段仍保留在各自子项目中。

## 8. 边界情况

1. 如果某个依赖只在一个子项目使用：
   - 不纳入本次 catalog。
2. 如果两个子项目未来需要同名依赖但版本必须故意不同：
   - 不应强制 catalog 化，需要在后续请求中单独说明。
3. 如果 catalog 改动后 lockfile 发生正常重写：
   - 允许更新，但不得夹带无关依赖升级。

## 9. 风险

1. `pnpm-workspace.yaml` 语法使用不当会导致 install 失败。
2. 若 catalog 引用写法与当前 pnpm 版本不兼容，可能需要小幅调整表达方式。
3. 若同时存在 lockfile 漂移，review 需要确认无无关升级混入。

## 10. 验收标准

| 编号 | 验收条件 | 验证方式 |
|---|---|---|
| AC-01 | `pnpm-workspace.yaml` 已新增公共依赖 catalog | 代码检查 |
| AC-02 | 当前两个子项目共同使用的 4 个重复依赖均改为 `catalog:` 引用 | 代码检查 |
| AC-03 | 子项目依赖归属未改变，只统一版本来源 | 代码检查 |
| AC-04 | `pnpm install` 能通过 | 命令验证 |
| AC-05 | 至少一个相关构建命令能通过，证明依赖链路未被破坏 | 命令验证 |

## 11. 验证与交接

- 技术验证：
  - `pnpm install`
  - `pnpm homepage:build`
- 人工复核重点：
  1. catalog 范围是否只覆盖真实重复依赖
  2. 是否没有把运行时依赖错误抬到根

## 12. 相关文件

- `pnpm-workspace.yaml`
- `packages/bilibili-live-tool/package.json`
- `packages/bilibili-live-tool-homepage/package.json`
- `pnpm-lock.yaml`
