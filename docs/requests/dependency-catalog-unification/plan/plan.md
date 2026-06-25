# 依赖 catalog 统一执行计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 用 pnpm workspace catalog 统一 packages 子项目中重复依赖的版本来源，但不改变各子项目自己的依赖边界。

**Architecture:** 通过 `pnpm-workspace.yaml` 维护公共版本目录，让两个子项目保留各自的 `package.json` 依赖声明，只将版本字面量改成 `catalog:`。这样既能统一版本管理，又不会把运行时/开发依赖边界混到根包里。

**Tech Stack:** pnpm workspace, YAML, JSON, monorepo package management

---

## 任务拆解

### 任务 1：在 workspace 层建立公共依赖 catalog

**关联规格**：spec/spec.md - 4.1 / 5.1 / 5.4 / 6 / 7.1 / AC-01 / AC-03

**执行内容**：
- 修改 `pnpm-workspace.yaml`
- 新增 `catalog`
- 将当前两个子项目真实共用的 4 个依赖版本集中维护

**测试策略**：
- 先更新 workspace 配置
- 再用子项目 `package.json` 的 `catalog:` 引用去验证能否解析

**执行模式**：串行

---

### 任务 2：把子项目重复依赖改为 `catalog:` 引用

**关联规格**：spec/spec.md - 4.2 / 5.2 / 5.3 / 7.2 / AC-02 / AC-03

**执行内容**：
- 修改 `packages/bilibili-live-tool/package.json`
- 修改 `packages/bilibili-live-tool-homepage/package.json`
- 仅将这 4 个重复依赖改为 `catalog:`：
  - `tailwindcss`
  - `@tailwindcss/postcss`
  - `postcss`
  - `autoprefixer`

**测试策略**：
- 通过代码检查确认版本号已迁移为 catalog 引用
- 通过 workspace 安装确认引用能被解析

**执行模式**：串行（依赖任务 1）

---

### 任务 3：更新 lockfile 并验证解析链路

**关联规格**：spec/spec.md - 8 / 9 / 10 / 11 / AC-04 / AC-05

**执行内容**：
- 运行 `pnpm install`
- 确认 `pnpm-lock.yaml` 正常更新
- 运行至少一个相关构建命令验证依赖链路未被破坏

**测试策略**：
- `pnpm install`
- `pnpm homepage:build`

**执行模式**：串行（依赖任务 1、2）

## 依赖关系

```text
任务 1 workspace catalog
  ↓
任务 2 子项目引用 catalog
  ↓
任务 3 安装与构建验证
```

## 整洁性与复杂度控制

1. 不把依赖边界抬到根 `package.json`
2. 不引入额外依赖治理脚本
3. 仅覆盖当前真实重复的 4 个依赖，不做全量 catalog 化
4. 子项目仍保留自己的依赖声明，便于局部理解和维护

## 模式决策与替代方案

- **采用**：pnpm workspace `catalog`
- **解决问题**：
  - 集中版本管理
  - 保留子项目依赖边界
  - 降低重复版本号维护成本
- **拒绝方案**：
  1. 根 `package.json` 公共依赖
  2. `pnpm.overrides` 隐式强制版本
  3. 人工同步版本号

## 并行执行建议

不启用 workflow-style 并行。

原因：
1. 任务量很小；
2. 任务 2 依赖任务 1；
3. 任务 3 必须在前两步完成后才能有效验证。

## 触发与上下文准备

- 启动前读取：
  - `pnpm-workspace.yaml`
  - `packages/bilibili-live-tool/package.json`
  - `packages/bilibili-live-tool-homepage/package.json`
  - `pnpm-lock.yaml`
- 触发顺序：
  - 先配 catalog，再改子项目，再跑安装验证

## 受影响文件或模块

| 文件/模块 | 变更类型 | 关联任务 |
|---|---|---|
| `pnpm-workspace.yaml` | 修改 | 任务 1 |
| `packages/bilibili-live-tool/package.json` | 修改 | 任务 2 |
| `packages/bilibili-live-tool-homepage/package.json` | 修改 | 任务 2 |
| `pnpm-lock.yaml` | 修改 | 任务 3 |

## 测试策略

1. 结构检查
   - 确认两个子项目对应依赖已改成 `catalog:`
2. 安装验证
   - `pnpm install`
3. 构建验证
   - `pnpm homepage:build`
4. 解析边界验证
   - 确认未把无关依赖迁入 catalog

## 观察与人工介入点

1. `plan` 审批：确认只处理当前真实重复依赖
2. 安装阶段：如果 pnpm catalog 语法与当前版本不兼容，需要决定是否改为等价 catalog 表达
3. 验证阶段：确认 lockfile 更新没有引入无关版本漂移

## 回滚说明

1. 若 catalog 语法导致 install 失败，可回滚 `pnpm-workspace.yaml` 与子项目 package.json 的本次变更
2. 若构建或安装结果异常，优先回滚 catalog 引用，再评估是否需要换成等价的 pnpm workspace 语法
3. 不回滚不相关的业务代码文件
