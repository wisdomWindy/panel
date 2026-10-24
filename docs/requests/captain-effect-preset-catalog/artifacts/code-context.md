# 代码上下文

## 上下文需求

本缺陷横跨 Rust IPC DTO、TypeScript contract/adapter、Vue 页面和 Canvas 渲染，bugfix-intake 需确认目录所有权与完整消费链。结构图为首选但非必需。

## 图能力检查

- 状态：`missing`
- 检测：仓库未提供可调用的 code graph 命令或索引，仅存在框架策略文档。
- 迭代：bugfix-intake iteration 1。

## 安装或引导记录

- attempted: no
- result: not needed
- reason: 本次是有明确入口和类型链的局部跨层修复，定向文本搜索和双端合约测试可形成闭环。

## 回退记录

- fallback used: yes
- 方法：用 `rg` 跟踪 `CaptainEffectPreset`、`CaptainEffectSnapshot`、`presetId`、`captain_snapshot` 和 `CaptainEffectSurface` 引用。
- 置信度：高；目录从 Rust 到 Vue 的链路有显式 DTO、adapter 和测试边界。

## 相关入口

- Rust 目录生产：`src-tauri/src/ipc/media.rs::captain_snapshot`
- Rust DTO：`src-tauri/src/ipc/dto.rs::CaptainEffectPresetDto`
- TS 合约/适配：`platform/desktop/contracts/media.ts` 与 `modules/media/adapters.ts`
- 配置消费：`views/captain-effect/index.vue`
- 效果消费：`components/media/CaptainEffectSurface.vue`

## 功能链与根因

`captain_snapshot` 将 `presets` 硬编码为 `Vec::new()` -> adapter 正常将空数组传入 store -> 页面发现当前 `presetId` 不在目录中，只补一个值为 `captain-default` 的选项 -> Canvas 完全不读 `presetId`，所有模板即使可选也会使用同一结构。

## 依赖和副作用边界

- Rust 拥有内置模板目录与稳定 ID。
- adapter 负责在 desktop IPC 边界失败关闭式校验目录字段。
- Vue 页面只在用户明确切换模板时将目录默认色复制到可编辑草稿。
- Canvas 只消费 `presetId` 和最终配置颜色，不拥有持久化状态。

## 影响范围

只需修改上述 Rust/TS/Vue 特效链与对应合约测试；无新命令、无数据库迁移、无窗口定位变更。

## 后续检查

- 验证 Rust 序列化后至少 10 项且包含“金色尊享”。
- 验证 adapter 拒绝缺失配色或非法颜色的目录项。
- 验证页面不再显示“素材未提供”警告，且切换模板会应用默认配色。
- 验证 Canvas 仍正确取消帧、断开 ResizeObserver 并支持 reduced motion。
