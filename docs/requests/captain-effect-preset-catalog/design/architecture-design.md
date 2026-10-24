# 架构设计：上舰特效内置模板

## 业务能力与所有权

| 能力/数据 | owner | 职责 |
| --- | --- | --- |
| 内置模板目录 | Rust `ipc::media` | 稳定 ID、显示名、默认配色和顺序 |
| desktop 边界校验 | `modules/media/adapters.ts` | 将 unknown IPC 负载收窄为冻结的 `CaptainEffectSnapshot` |
| 可编辑配置草稿 | `views/captain-effect/index.vue` | 页面唯一可写 owner，响应用户选择与自定义配色 |
| 特效渲染生命周期 | `CaptainEffectSurface.vue` | 消费 payload，管理 Canvas 帧循环/resize/销毁，不写入业务状态 |

## 文件与符号变更

- `src-tauri/src/ipc/dto.rs`：扩展 `CaptainEffectPresetDto` 默认色字段。
- `src-tauri/src/ipc/media.rs`：添加窄范围 `captain_effect_presets()` 目录生产函数，`captain_snapshot` 直接消费。
- `platform/desktop/contracts/media.ts`：扩展 `CaptainEffectPreset` 类型。
- `modules/media/adapters.ts`：在现有 snapshot adapter 中完成目录颜色校验。
- `views/captain-effect/index.vue`：增加就近的 `handlePresetChange`，不抽取新 hook/helper 文件。
- `components/media/CaptainEffectSurface.vue`：增加组件内部的 typed record 装饰映射和纯绘制 helper。
- 相关 Rust/TS 合约测试：补数量、必需项、校验和渲染分支证据。

## 依赖规则矩阵

| 调用方 | 允许依赖 | 禁止依赖 |
| --- | --- | --- |
| Rust `captain_snapshot` | 本模块 DTO 与纯目录函数 | renderer、Vue、文件系统或数据库 |
| TS adapter | platform contract 与本文件边界 helper | store、Vue 组件、locale |
| 配置页 | store snapshot 和 platform types | Rust DTO 形状、Canvas 内部映射 |
| Canvas surface | `CaptainEffectPayload`、Canvas 2D API、i18n | store、desktop invoke、持久化层 |

数据单向流动：`Rust catalog -> serialized DTO -> adapter -> immutable snapshot -> page draft -> save/preview config -> payload -> canvas`。不引入反向 import 或第二个可写目录。

## 类型与函数设计

- `CaptainEffectPreset` 继续使用结构化 interface，不把 ID 收窄为枚举，以允许旧配置和未来 Rust 目录无前端数据迁移地加载。
- `captain_effect_presets() -> Vec<CaptainEffectPresetDto>` 是 Rust 纯函数，无 I/O、无可变全局。
- `handlePresetChange(presetId)` 是页面事件边界：查找已适配目录项并显式更新三个草稿字段。
- Canvas 风格使用 `type CaptainDecoration = ...` 和 `Readonly<Record<string, CaptainDecoration>>`。风格选择是纯查表，未知键回退到 `classic`。
- 绘制 helper 显式接收 context、布局几何、主/副色和动画 progress；只改变 Canvas context 这一明确副作用边界。

## 状态拓扑与兼容

- 可写状态：仅页面 `reactive<CaptainEffectConfig> draft`。
- 只读状态：store `snapshot.presets`、Canvas `props.payload`。
- 派生状态：表单有效性和翻译选项，不新增可写副本。
- 兼容 owner：页面保留未知 ID 选项，Canvas 保留默认风格。清理触发器为未来确认无旧配置后，不在本请求删除。

## 复用候选与 Anti-DRY 决策

| 候选 | 语义/分层 | 生产使用点 | 变化性 | 决策 |
| --- | --- | --- | --- | --- |
| 十六进制颜色校验 | TS adapter 边界，与 config 颜色同语义 | 同一 adapter 内 4 字段 | 稳定，单变因 | 复用现有 regex，提为同文件纯 `hexColor` helper，避免四处漂移 |
| 模板目录 | Rust IPC 产品配置 | 1 个真实生产调用方 | 稳定但无共享需求 | 就近放在 `ipc/media.rs`，不新建 constants 文件 |
| 模板默认色与 Canvas 风格 | 目录配置 vs 渲染策略，不同分层 | 各 1 处 | 颜色和形态独立演进 | 保持分离；仅共享稳定 `presetId` 合约，不将 Canvas 知识泄漏到 Rust |
| 装饰绘制 helper | Canvas 局部纯几何逻辑 | 1 个组件 | 变体数有限 | 就近放在 SFC，不新建 utility 文件 |

## Pattern-Fit 决策

- 问题：根据稳定 ID 切换 Canvas 装饰行为，并使用一个明确 adapter 边界收窄 IPC 数据。
- 深度：`Level 1 local lightweight shape`。
- 选择：复用现有 adapter 模式；Canvas 采用 typed record 策略映射，不创建 class 层级。
- 拒绝备选：classic Strategy classes / Factory / Registry / Plugin 机制。当前变体均是本地绘制，没有动态注册、资源加载或独立生命周期，额外间接层不会降低耦合。
- 过度信号：若每个映射只调用同一函数且参数无差异，应删除映射；未来只在真正引入外部资产或动态扩展时重新评估更高层级。

## 复杂度预算与迁移

- 新增文件：0 个生产代码文件。
- 新增公开 API：`CaptainEffectPreset` 增加 2 个字段；无新 command/event。
- 新增间接层：0。
- 迁移：无数据迁移；旧 `presetId` 和 config 形状不变。
- 回滚面：目录扩展和 Canvas 风格映射均可独立回退，不影响已保存配置。

## ADR

### ADR-01：Rust 拥有内置目录

- 决策：继续使用 snapshot 中的 Rust-owned catalog，并增加默认色字段。
- 备选：前端硬编码目录。
- 理由：现有合约已将目录作为 Rust snapshot 一部分；修复真正 owner 可避免只掩盖 UI 症状。
- 验证：Rust 合约测试检查数量、唯一 ID、金色尊享和颜色。

### ADR-02：Canvas 使用本地 typed record 风格映射

- 决策：以 `presetId -> decoration` 映射选择局部绘制策略。
- 备选：将风格枚举加入 config/DTO，或创建多个 Vue 组件。
- 理由：风格是 renderer 实现细节，不应增加持久化数据或组件生命周期复杂度。
- 验证：聚焦测试/合约测试证明映射覆盖与默认降级，原 Canvas 生命周期合约仍通过。

## 架构风险与未决问题

无阻塞问题。主要风险是 Rust 目录 ID 与 renderer 映射漂移；计划用静态合约测试检查所有目录 ID 都在 Canvas 映射中出现，并保留未知 ID 降级。
