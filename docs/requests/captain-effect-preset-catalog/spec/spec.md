# 工程规格：上舰特效内置模板

## 范围与目标

修复上舰特效模板目录为空、页面只显示 `captain-default` 的缺陷。交付 12 个本地内置模板，包含“金色尊享”，并使选择、配色、预览和真实触发构成完整链路。

## 来源映射与产品白名单

| 规格项 | 来源 |
| --- | --- |
| 至少 10 个预设模板 | P1、P3 |
| “金色尊享” | U1、P2 |
| 自定义主/副色 | P1、PRD 9.6.3-9.6.4 |
| 预览与保存 | PRD 9.6.5，现有合约 |
| 兼容已保存 `captain-default` | U1 已观察数据，行为保持约束 |
| 其他本地主题名称/配色/装饰 | U1 “多内置几个”明确授权；不作为 PRD 原文声称 |

## 内置模板目录合同

Rust `CaptainEffectPresetDto` 在现有 `presetId` / `displayName` 之上增加 `primaryColor` / `secondaryColor`，均为非空字符串，颜色必须符合 `#RRGGBB`。TypeScript `CaptainEffectPreset` 与 adapter 使用相同字段和校验。

| `presetId` | 显示名 | 主色 | 副色 |
| --- | --- | --- | --- |
| `captain-default` | 经典应援 | `#00AEEC` | `#FB7299` |
| `golden-premium` | 金色尊享 | `#FFD700` | `#FFA500` |
| `starlight-blue` | 星海蓝耀 | `#38BDF8` | `#6366F1` |
| `cherry-blossom` | 樱花绽放 | `#FB7185` | `#F9A8D4` |
| `violet-galaxy` | 紫曜星河 | `#A78BFA` | `#7C3AED` |
| `emerald-stream` | 翡翠流光 | `#34D399` | `#0F766E` |
| `crimson-celebration` | 赤焰庆典 | `#F43F5E` | `#F97316` |
| `silver-frost` | 冰晶银辉 | `#E2E8F0` | `#94A3B8` |
| `neon-party` | 霓虹派对 | `#22D3EE` | `#F472B6` |
| `aurora-dream` | 极光幻彩 | `#2DD4BF` | `#C084FC` |
| `sunset-flare` | 落日鎏光 | `#FB923C` | `#EAB308` |
| `midnight-crown` | 午夜王冠 | `#818CF8` | `#312E81` |

目录顺序是 UI 顺序；`captain-default` 保持首项和默认持久化 ID，避免破坏旧配置。

## 页面行为

- 页面加载 snapshot 后不自动重写已保存颜色。
- 用户在 `el-select` 中选择目录项时，同一处事件处理器将 `draft.presetId`、`draft.primaryColor`、`draft.secondaryColor` 更新为选中项值。
- 用户随后可使用现有颜色选择器修改颜色；预览/保存使用草稿的最终颜色。
- 已保存的未知 `presetId` 继续显示一个兼容选项，不变更其颜色；Canvas 走默认风格分支。
- 删除“完整特效模板素材尚未提供”警告与对应中英文 locale 键，并搜索确认无孤儿引用。

## Canvas 渲染合同

- 共享信息带、文字、淡入/淡出、DPI 缩放、ResizeObserver 和销毁清理链不变。
- 在绘制文字前，根据 `payload.config.presetId` 选择一个有界的本地装饰策略，使用主/副色绘制颗粒、星芒、光带、环形或冠冕等简化几何形态。
- 12 个 ID 必须至少分布到 6 种不同的装饰策略，不得只改颜色却谎称不同模板。
- 未知 ID 使用经典应援的稳定默认策略。
- reduced motion 模式禁止持续帧循环，但仍绘制一帧可读完整画面。

## 类型、边界与异常

- 不新增持久化字段；`CaptainEffectConfig.presetId` 继续是模板选择的唯一持久化标识。
- adapter 对目录数组逐项校验非空 ID/显示名和两个十六进制颜色；任一无效时使用现有 `MalformedMediaPayloadError` 失败关闭。
- 配置页选择处理仅接收已适配的 `CaptainEffectPreset`，不再重复校验颜色。
- Canvas 装饰函数为纯绘制函数，状态仅来自当前 context、几何、颜色和时间；不发起 I/O 也不修改 store/props。

## 用户意图合同

- 字面请求：不要只有 `captain-default`，多内置特效模板，并提供“金色尊享”。
- 实际目标：用户能够从真实可用的多种特效风格中选择，而不是看到一组无效的名称。
- 禁止解释：只在下拉框增加文本、但预览和实际渲染完全一致；或删除 `captain-default` 导致旧数据失效。
- 成功检查：目录数量与名称、选择后配色、多策略 Canvas 渲染、旧 ID 降级均有自动化或静态合约证据。

## 模式与架构信号

- pattern-fit：选择/状态分支信号存在，采用 `Level 1 local lightweight shape`：Rust 定值目录 + Vue 本地选择处理器 + Canvas 的 typed record 风格映射。
- 拒绝类/工厂/插件注册机制：内置项数量固定，无运行时扩展、外部依赖或多态生命周期，正式模式会过度设计。
- `architecture-design_required=true`：目录 ID 在 Rust 生产层和 Canvas 消费层共享，需记录所有权、依赖方向和兼容边界。
- Vue 组件决策：配置页选择处理保持内联；`CaptainEffectSurface` 继续作为已有业务生命周期边界，不新增 Vue SFC。

## 代码质量与审阅约束

- 先更新 Rust/TS 类型合约和失败测试，再实现生产代码。
- 目录与风格映射使用不可变数据；用户草稿的唯一可变状态继续属于页面。
- 不新增泛化 `utils`、manager、class 或外部动画库；Canvas 的单调用方辅助函数保持在组件内。
- 必须阅读并遵循 `packages/bilibili-live-tool/tsconfig.json` 和 `tsconfig.eslint.json`；类型来源为 `platform/desktop/contracts/media.ts`。
- 仅修改请求工件、特效 DTO/IPC、TS 合约/适配、特效页/表面、locale 死键清理与相关测试；不格式化或重写用户的其他未提交改动。

## 验收标准

1. `captain_effect_get` 的 snapshot 返回 12 个稳定、唯一的内置模板，包含 `golden-premium / 金色尊享`。
2. 每个目录项包含合法 `#RRGGBB` 默认主/副色，adapter 严格验证并冻结适配结果。
3. 切换模板更新草稿 ID 和默认配色，不自动保存；手动改色、预览和保存继续正常。
4. Canvas 使用不少于 6 种可识别的装饰策略，未知 ID 降级到经典风格。
5. 现有 Canvas 帧循环、DPI、ResizeObserver、reduced-motion 和可访问文本合约不回归。
6. 相关 Vitest、TypeScript 检查、ESLint 检查和 Rust 媒体/IPC 测试通过。

## 非范围

新音效、外部图片/视频素材、模板商店或下载、数据库迁移、新 IPC 命令、上舰事件队列或特效窗口几何修改。
