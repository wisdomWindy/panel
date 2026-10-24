# 工程规格

## 验证回流结构约束修订（2026-09-22）

HuiHui 解析函数可位于 `views/tts/voice-selection.ts`，但必须保持纯函数、只由 TTS 页面在生产代码中调用，且不得升级为 shared helper。该修订仅改变测试边界，不改变 U2 的默认音色行为。

## 范围与来源映射

| 合同 | 来源 |
| --- | --- |
| 免费版导航可达页面及直接子组件随 locale 切换 | U1 |
| 无已选音色时优先 Microsoft HuiHui | U2 |
| TTS 重置持久化并提示成功/失败 | U3 |
| 特效窗口宽高可配置 | U4 |
| 特效窗口按五种位置显示 | U5 |
| 保留 `560×280` 默认与旧配置兼容 | 现有代码事实 + 安全派生 |

闭集范围仅包括上述合同及必要类型、适配、持久化、窗口副作用、测试和文档更新。

## 用户意图合同

- stated request：修复五项已列出的界面与媒体配置问题。
- practical goal：语言、页面设置和真实桌面行为一致，用户无需猜测设置是否生效。
- success criteria：English 页面无遗留中文 UI；HuiHui 自动选中；重置结果可观察；特效尺寸/位置与配置一致。
- forbidden interpretations：只翻译导航；只在下拉框伪造 HuiHui；重置只改本地草稿；只保存位置但不移动窗口；只加宽高控件但不传到桌面窗口。
- acceptable approaches：复用 vue-i18n、现有 store/IPC，扩展 CaptainEffectConfig，并在 Tauri window port 实现屏幕几何。

## 模块与职责

### 国际化

- `locales/zh-CN.json` 与 `locales/en-US.json` 是页面文案唯一资源 owner，两者键结构必须一致。
- 免费版路由可达页面和它们直接使用的 live-data/media 组件通过 `useI18n()` 消费资源。
- 业务数据值（用户名、礼物名、日志内容、用户保存模板）不得翻译。
- route title 改为 locale key，并在 route/locale 变化时更新 document title。
- Element Plus locale 继续由 `elementPlusLocale` 驱动，不新增语言状态。

覆盖页面：connection、danmaku-list、dashboard、rankings、sensitive-words、blacklist、captain-effect、gift-reminder、gift-sound、tts、recording、plugin-manager、settings、update-log，以及仍可直接访问的 gift-list/logs；覆盖其现有 live-data/media 子组件。

### TTS 默认音色

- 输入：`TtsConfig` 与当前 engine 下 `TtsVoice[]`。
- 若 `selectedVoiceId` 非空，原值保持。
- 若 engine 不是 system，不自动选择 HuiHui。
- 否则按 `displayName.trim().toLocaleLowerCase()` 匹配包含 `microsoft huihui` 的可用音色，返回其真实 `voiceId`。
- 没有匹配时保持 `null`，不创建虚假选项。
- 初始加载与重置默认复用同一纯解析规则；切回 system 且当前音色不可用时同样优先 HuiHui。

### TTS 重置

- 点击后显示现有确认框；取消/关闭不修改草稿、不调用 IPC、不提示成功。
- 确认后构造默认配置，`selectedVoiceId` 使用上述解析结果，并调用 `store.save`。
- pending 复用 `pendingOperations.has('save')`，重置与保存按钮在同一保存操作期间不可重复提交。
- 成功：用返回 snapshot 回填草稿并显示 locale 化成功消息。
- 失败：显示规范化错误或 locale fallback；不宣称成功。

### 上舰特效尺寸合同

`CaptainEffectConfig` 新增：

- `windowWidth: number/u32`，单位 px，默认 560，允许 320..=1920。
- `windowHeight: number/u32`，单位 px，默认 280，允许 180..=1080。

页面在“视觉与位置”内提供两个 `el-input-number`，整数步进，显示 `px` 后缀。formValid 必须覆盖宽高范围。保存、预览、重置、adapter、IPC DTO、domain model、repository JSON 均携带字段。

旧 repository JSON 中缺少字段时，Rust serde 分别回填 560 与 280；非法显式值必须 fail-fast，不得静默夹取后持久化。

### 上舰特效位置合同

`EffectWindowSpec` 包含 label、width、height、position。`EffectRuntime` 从当前/预览 config 构造完整 spec。

Tauri adapter 使用主显示器 logical geometry 计算窗口左上角，边距 24 logical px：

- topLeft：左上 + margin。
- topRight：右边界 - window width - margin。
- bottomLeft：下边界 - window height - margin。
- bottomRight：同时应用右/下计算。
- center：显示器内容矩形中心。

坐标使用 saturating/clamp 逻辑，窗口大于可用区域时不得产生负向溢出计算。主显示器或窗口创建不可用时返回现有 media error，不使用错误的 `(0,0)` 假成功。

## 字段展示语义

- 窗口宽度/高度代表外部透明 WebView 的 logical pixel 尺寸，不代表 Canvas 内部装饰带尺寸；数字不可点击跳转，无特殊强调色。
- 显示位置代表外部窗口相对主显示器的位置；选择值默认态与其他 select 相同，只有选中态由 Element Plus 表达。
- 音色显示值是系统返回的 displayName；空值表示未显式选择，不展示伪造名称。
- locale 文案是 UI chrome，不修改业务数据或用户生成内容。

## API 与数据合同

- 权威前端契约：`src/renderer/platform/desktop/contracts/media.ts`。
- 权威桌面 DTO：`src-tauri/src/ipc/dto.rs`，camelCase serde。
- renderer 继续经 `adaptCaptainEffectConfig` fail-fast 归一，不让 DTO 未校验值进入组件。
- Rust domain config 负责可持久化语义和范围校验；infrastructure window port 负责 OS monitor/window API。
- 不新增网络 API、权限或服务端职责。

## TypeScript 上下文

- governed by `packages/bilibili-live-tool/tsconfig.json`，测试/lint 使用 `tsconfig.eslint.json` 扩展。
- `strict`, `noUnusedLocals`, `noUnusedParameters`, `moduleResolution=bundler`, `resolveJsonModule` 与 `@/*` alias 直接约束实现。
- 必读类型闭包：desktop media contracts、adapters、Vue auto imports、vue-i18n 类型；禁止通过全仓 `.d.ts` 扫描猜测类型。

## UI/UX 与状态合同

- 沿用 `design/page-design.md` 的布局、交互、响应式和可访问性决定。
- 英文长标签允许换行，不更改功能分组。
- tooltip、aria-label、确认框、toast、空态、状态标签均在国际化覆盖内。
- loading/error/empty 状态只替换现有文案，不新增业务状态。

## 边界与异常

- locale key 缺失属于测试失败；不得依赖 fallbackLocale 让 English 显示中文。
- 用户已有 voiceId 即使 displayName 不是 HuiHui也必须保留。
- HuiHui 条目不可用时忽略。
- 重置保存竞态由 store 现有同 key 去重承担。
- 宽高为 NaN、非整数、越界或 DTO 缺失（新 IPC 请求）必须拒绝；仅旧持久化 JSON 缺失允许 serde 默认。
- monitor 含非零/负起点（多显示器）时计算必须包含 monitor origin。

## 模式适配决策

- pattern depth：Level 2，复用现有 vue-i18n、adapter、store command 与 port/adapter 模式。
- locale 是横切关注点，但已有框架模式，不引入自定义观察者或 DOM 翻译层。
- 窗口位置计算采用纯函数 + 现有 port adapter，拒绝新增 manager/factory。
- TTS 默认音色规则保持页面内 local pure helper：单个页面 owner，虽有多个本地调用点但无跨模块生产 caller，不创建独立 helper 文件。
- 宽高控件保持页面内联；单调用方、无独立生命周期，拒绝抽取新 Vue SFC。

## 架构复用信号

- locale 消费在多个页面重复，但语义由 vue-i18n 既有机制统一，无新共享 helper。
- `CaptainEffectConfig` 跨 TS/Rust 多层复制是显式边界合同，不抽成跨语言代码生成。
- 屏幕几何规则只属于 Tauri window infrastructure，保持纯函数与副作用函数分离。
- `architecture-design_required=true`：类型字段跨 renderer/Rust，且位置副作用所有权需要明确依赖方向。

## 函数式与副作用约束

- preferred voice 解析和 window coordinate 计算必须是确定性纯函数。
- locale option/label 派生使用 computed 或函数，不写第二份可变翻译状态。
- store snapshots、props、DTO inputs 视为不可变；页面草稿使用显式复制。
- IPC/save/window creation 只在 store action、event handler、Rust application/infrastructure 边界执行。
- 不引入函数式工具库。

## 生产代码质量约束

- 先扩展 CaptainEffectConfig/DTO/EffectWindowSpec 类型，再实现消费者。
- 空值和非法范围 fail-fast；`null` voiceId 有明确“未选择”语义。
- 常量仅用于跨反序列化/校验/默认配置共享的尺寸规则与窗口边距。
- 命名保持 `is/has/can` 布尔、`handle*` UI handler、`validate/resolve/calculate` 纯规则。
- 无需 memo/cache/virtualization；locale computed 是 Vue 正常派生。

## 样式约束

- 新样式只用 Tailwind utility；不增加 scoped CSS、style object 或 class 常量。
- 条件 class 只用于现有小型状态色。
- 不以隐藏 class 的方式规避格式化行宽。

## 人工评审就绪约束

- 保留工作区既有未提交修改，特别是 locale、App、Rust live-data 相关用户改动。
- 只触碰本规格列出的页面/媒体合同链，避免全文件格式化噪音。
- 每个 changed hunk 映射到 i18n、TTS 或 effect 任务与测试。
- 执行后清理 unused imports、旧中文字面量、过期测试 fixture 与重复默认对象。
- 证据至少包括 focused red/green、全量 Vitest、type-check、lint、Rust fmt/check/test、web build、Windows 桌面预览和安装包构建。

## 自愈完成标准

需求一致性、逻辑正确性、类型/语法、运行时安全、代码组织、可维护性与技术债 blocker 全部通过；同一 blocker 不超过框架重试边界。

## 验收标准

1. English 下免费版所有导航可达业务页面及主要子组件显示英文 UI，locale 文件键结构一致，无 English fallback 到中文。
2. 已有 voiceId 保留；无 voiceId 且可用列表含 Microsoft HuiHui 时选中其真实 id。
3. TTS 重置确认后调用一次保存，成功/失败消息正确，取消零副作用。
4. CaptainEffectConfig 全链路包含 width/height，旧 JSON 缺失字段可加载，非法值被拒绝。
5. EffectWindowSpec 使用配置尺寸和位置；纯坐标测试覆盖五种位置、非零原点与窗口大于显示器。
6. Windows 预览中修改尺寸可观察，选择居中时窗口位于主显示器中心而非左上角。

## 风险

- 国际化 diff 大，最主要风险是动态属性遗漏或 locale key 漂移。
- Tauri monitor API 的 physical/logical 单位错误会造成高 DPI 偏移。
- 新 Rust 字段会影响所有 CaptainEffectConfig fixture，必须闭包更新。
