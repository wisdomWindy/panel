# 依赖 catalog 统一请求

## 请求 ID
`dependency-catalog-unification`

## 来源
- 本地需求指令
- 目标范围：`packages/*` 子项目依赖版本统一

## 业务摘要
当前 monorepo 下存在两个子项目：`@monorepo/bilibili-live-tool` 与 `@monorepo/bilibili-live-tool-homepage`。用户希望将 packages 中多个子项目共同使用的依赖设置为“公共依赖”，并明确选择采用 `catalog` 方案，即通过 workspace 级别的版本目录统一依赖版本，而不把依赖边界粗暴抬升到根包或打乱各子项目自己的依赖声明。

## 目标陈述
为当前 monorepo 建立 workspace 级别的 `catalog` 版本源，将多个子项目实际共用的依赖版本集中到 `pnpm-workspace.yaml` 维护，并让子项目通过 `catalog:` 引用这些公共版本，保持依赖边界不变但版本管理统一。

## 初始完成信号
当以下条件同时成立时，本请求可视为完成：
1. `pnpm-workspace.yaml` 已定义本次共用依赖的 `catalog`
2. 相关子项目的 `package.json` 已改用 `catalog:` 引用公共版本
3. `pnpm install` 与相关构建/验证命令可通过
4. review / verification 工件明确记录版本统一范围与未纳入范围

## 触发条件
当多个 workspace 子项目开始重复声明同一批依赖版本，且团队希望统一维护这些版本但不改变依赖归属时，启动本请求。

## 初始上下文来源
- 根 `package.json`
- `pnpm-workspace.yaml`
- `packages/bilibili-live-tool/package.json`
- `packages/bilibili-live-tool-homepage/package.json`
- 用户明确决策：采用 `catalog` 方案

## 人工交接点
- Spec 审批：确认 catalog 只统一重复依赖版本，不改变运行时边界
- Plan 审批：确认本次只覆盖当前真实重复依赖，不扩散到全部依赖清理
- 交付后人工复核：未来新增重复依赖是否继续遵循 catalog 方式

## 影响范围
- `pnpm-workspace.yaml`
- `packages/bilibili-live-tool/package.json`
- `packages/bilibili-live-tool-homepage/package.json`
- `pnpm-lock.yaml`

## 参与模块
- pnpm workspace 配置
- 子项目依赖声明
- monorepo 依赖安装与解析链路
