# 架构设计

## 验证回流修订（2026-09-22）

- `ADR-2` 的所有权结论保持不变：HuiHui 解析仍只属于 TTS feature，不进入 shared utility。
- 实现约束由“与 SFC 同文件”修订为“与 SFC 同功能目录的私有纯函数文件”：`views/tts/voice-selection.ts` 仅有 `views/tts/index.vue` 一个生产调用方。
- 单调用方例外理由：直接导入 TTS SFC 会引入 Element Plus CSS/组件环境，无法将确定性 voice 规则作为无 DOM 纯单元测试。拆分不新增公开 API、可变状态或跨功能依赖。
- 复杂度预算更新：新增 1 个 feature-private 纯函数文件和 1 个对应单元测试文件；其余预算不变。

## 交付单元与目标

`i18n-tts-effect-window-fixes`。在不引入第二套状态或窗口控制通道的前提下，让页面文案、TTS 默认/重置和特效窗口几何沿现有 owner 完整闭环。

## 范围与触发信号

- 多页面横切 locale 消费。
- `CaptainEffectConfig` 同时跨 TypeScript contract/adapter/store 与 Rust DTO/domain/repository/runtime/infrastructure。
- 屏幕位置计算和 OS window 副作用需要清晰分层。

## 上游输入

以已批准 `spec/spec.md` 和 `design/page-design.md` 为行为与 UI 合同；不在本阶段改变字段、默认、范围或验收标准。

## 脚手架边界

不适用：既有 Vue/Tauri 应用，不新增包或入口。

## 架构决策记录

### ADR-1：页面直接消费现有 vue-i18n

- 问题：多数页面仍含中文字面量。
- 选择：locale JSON 为文案 owner，各 SFC 用 `useI18n` 的 `t`/computed 消费。
- 备选：DOM MutationObserver 自动翻译、全局中文字典 helper。拒绝，因为会绕过 Vue 响应式、遗漏属性并形成第二套国际化机制。
- 最小性：不新增抽象层，只补齐已有项目模式。
- 依赖方向：page/component -> vue-i18n + locale resources；locale 不依赖 feature。
- 状态 owner：`i18n-runtime` 保持唯一 locale writable owner。
- 证据：locale key parity 测试、页面 contract 扫描、运行时语言切换抽查。

### ADR-2：TTS 首选音色为页面局部纯规则

- 问题：系统 voice id 只能在 snapshot 到达后解析。
- 选择：TTS 页面 local pure function 从 immutable voices 中解析，供初始 apply、engine change 和 reset 使用。
- 备选：写死 voice id、修改 Rust 默认配置、创建 shared helper 文件。拒绝，因为 voice id 是运行时系统值，规则只有一个页面 owner，独立文件增加跳转。
- 状态 owner：持久化 config 仍在 Rust/store；draft 仍在页面。
- 副作用：只有 `handleReset`/`handleSave` 调用 store。
- 证据：页面/组件测试覆盖已有选择、匹配、无匹配与保存反馈。

### ADR-3：窗口几何进入 domain config，坐标归 infrastructure

- 问题：尺寸固定且 position 未被窗口实现消费。
- 选择：宽高属于 `CaptainEffectConfig`；application 生成完整 `EffectWindowSpec`；infrastructure 读取 monitor、纯计算 logical position、创建窗口。
- 备选：renderer 计算坐标后传 IPC、在 application 层调用 Tauri、只在 payload/CSS 改大小。拒绝，因为会泄漏 OS API、逆转依赖或只改变内容不改变窗口。
- 数据 owner：repository JSON 保存 config；domain 校验；Tauri adapter 拥有 monitor/window 副作用。
- 兼容：serde 缺字段默认，仅服务旧持久化数据；新 DTO 必须显式完整。
- 证据：domain/DTO/runtime/window geometry tests + Windows 预览。

## 能力与 ownership map

| 能力 | Owner | Writable state / side effect |
| --- | --- | --- |
| locale 选择 | application-shell + i18n-runtime | 当前 locale |
| 页面翻译 | 各 SFC | 无，仅读取 locale |
| TTS config/draft | Rust runtime/store / TTS page | snapshot / local draft |
| 首选音色解析 | TTS page local pure function | 无 |
| Captain config | media domain/repository | 持久化 JSON |
| effect 排队 | EffectRuntime | queue/current state |
| monitor/window | TauriEffectWindowPort | OS window |

## 模块边界与文件结构

- `src/renderer/locales/*.json`：新增/扩充对称 keys。
- free route 页面和 `components/live-data|media`：只做文案消费与已有 UI 编排。
- `platform/desktop/contracts/media.ts`：前端 CaptainEffectConfig 类型。
- `modules/media/adapters.ts`：桌面边界 fail-fast。
- `src-tauri/domain/media/{models,rules}.rs`：持久化类型、serde default、范围规则。
- `src-tauri/application/media/{effects,ports,state}.rs`：config -> window spec。
- `src-tauri/ipc/{dto,media}.rs`：camelCase 显式字段转换。
- `src-tauri/infrastructure/media/window.rs`：monitor 获取、坐标纯函数、窗口创建。

不新增共享 helper、composable、manager、factory 或 Vue 组件文件。

## 代码关系与依赖方向

`SFC -> vue-i18n/locales`

`SFC -> store -> typed desktop invoke -> adapter -> safe renderer contract`

`IPC DTO -> domain config -> application runtime -> EffectWindowPort <- Tauri adapter`

domain/application 不得 import Tauri；renderer 不得 import Rust/OS geometry；locale resource 不得 import feature code。

## 依赖规则矩阵

| From | 可依赖 | 禁止依赖 |
| --- | --- | --- |
| Vue pages/components | vue-i18n、store、renderer contracts | Tauri API、Rust DTO |
| renderer adapters | desktop contracts | Vue component/store |
| Rust domain | serde、domain constants | Tauri/window |
| Rust application | domain、port traits | concrete Tauri window |
| infrastructure window | application port、Tauri | renderer/store |

公开 API 仅扩展已有 `CaptainEffectConfig` 和 `EffectWindowSpec`；坐标 helper 保持 module-private。

## 状态拓扑与数据生命周期

- locale：shell event/save -> projection -> `localeState` -> i18n computed -> SFC render；无镜像 writable state。
- TTS：desktop snapshot -> adapter -> store snapshot -> copied page draft；preferred voice 是派生值；save response 替换 snapshot/draft。
- Effect：page draft -> adapter-validated request -> DTO -> domain validate/save -> EffectRuntime config -> spec -> Tauri window；payload继续负责 Canvas 内容。
- async pending/去重仍由现有 store owner 管理；dispose/generation 语义不变。

## 复用候选与 Anti-DRY

| 候选 | 语义/分层 | 真实使用 | 稳定性/变因 | 决策 |
| --- | --- | --- | --- | --- |
| 页面翻译调用 | 同一机制、不同业务 key | 3+ | 稳定，key 不同 | 复用 vue-i18n，不抽 helper |
| preferred HuiHui | TTS 页面规则 | 1 page / 3 local flows | 稳定，voice list + engine | 同文件 local pure helper |
| effect width/height defaults | 同一 domain 配置 | TS/Rust 多边界 | 稳定，跨语言复制不可消除 | 各契约显式值，测试保证一致 |
| position map | window infrastructure | 1 production caller | 稳定，5 enum variants | private pure function |
| gift popup position | 相似枚举但不同窗口/生命周期 | 不同 owner | 独立演进 | 保持分离 |

共性分类：vue-i18n 为基础设施机制；Captain fields 为领域配置；坐标为基础设施纯规则；页面 labels 为 feature UI 内容。

## 函数与入口设计

- `resolvePreferredVoiceId(config, voices): string | null`：不变更输入。
- `applyConfig(config, voices)`：显式复制数组/模板并应用已选/首选 voice。
- `handleReset()`：确认和保存命令边界。
- `validate_captain_effect_config`：宽高 fail-fast。
- `EffectWindowSpec { label, width, height, position }`。
- `calculate_window_position(monitor, spec) -> LogicalPosition<f64>`：纯函数。
- `TauriEffectWindowPort::ensure`：获取主 monitor 并创建窗口的唯一副作用入口。

## 类型策略与 adapter 边界

- TS width/height 为 number，但 adapter 先验证 integer/range，返回安全 `CaptainEffectConfig`。
- Rust 使用 `u32`；serde field defaults 仅负责旧 repository JSON 缺失。
- DTO 字段为 `u32`，请求缺失自动反序列化失败。
- MediaPosition 继续使用现有 discriminated enum/union，不新增字符串分支。

## Extension seams 与拒绝项

真实变化轴仅为现有五位置和两个尺寸值。拒绝显示器选择、任意坐标、百分比、拖拽、窗口 profile、strategy registry 与 window manager；未来只有用户明确请求这些能力时再扩展 contract。

## 复杂度预算

- 新生产文件：0。
- 新公开字段：TS/Rust 各 2 个；EffectWindowSpec 新增 1 个 position。
- 新抽象层：0。
- 新纯 helper：页面本地 1 个、Rust window module 私有 1 个。
- 迁移：更新所有 CaptainEffectConfig fixtures/constructors 与 locale 消费点。

## 兼容、清理与回滚

- 旧 JSON fallback owner：Rust domain serde，长期保留，因为安装升级需要。
- 不建立双写或 feature flag。
- 如果 Tauri geometry 编译/API 不匹配，修正 infrastructure 实现，不回退到 `(0,0)`。
- 删除硬编码页面文案后清理失效常量和测试断言；不得保留第二条翻译路径。

## 可读性守卫

- locale keys 按 feature 分组，禁止平铺无语义 key。
- 坐标公式用命名 helper，window creation 保持线性流程。
- 不全文件格式化，不改无关 member-only 页面。

## 架构风险

- 大量 locale key 可能出现两语言结构漂移。
- 高 DPI 与负 monitor origin 需纯测试和实机证据。
- 工作区已有重叠 locale/App 修改，编辑必须基于当前内容增量合并。

## 未决问题

无。
