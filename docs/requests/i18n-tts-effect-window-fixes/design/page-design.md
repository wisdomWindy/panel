# 页面设计

## 交付单元

`i18n-tts-effect-window-fixes`，非拆分 bugfix 请求。

## 页面目标

保持现有操作台布局不变，让语言、TTS 默认/重置和上舰特效窗口配置在用户操作后产生一致、明确、可验证的结果。

## 目标用户与场景

- 使用 English 界面的直播助手用户。
- 使用 Windows 系统 Microsoft HuiHui 进行中文播报的用户。
- 需要把上舰特效投放到指定屏幕区域并控制窗口尺寸的主播。

## 来源与上游约束

- U1-U5 与 `artifacts/prd-snapshot.md` 是产品内容授权来源。
- 既有 vue-i18n、Element Plus、Tailwind 工具类和 media store/IPC 是不可替换的项目约束。
- 不改变导航结构、路由、业务字段含义或媒体工作流。

## 产品内容白名单映射

| 设计内容 | 来源 |
| --- | --- |
| 免费版页面英文文案 | U1 |
| 默认 Microsoft HuiHui | U2 |
| 重置成功/失败反馈 | U3 |
| 窗口宽度、窗口高度字段 | U4 |
| 左上、右上、左下、右下、居中位置结果 | U5 与现有 position 枚举 |
| `560×280` 默认值 | 现有代码事实，兼容性保持 |
| px 单位与安全上下限 | 防止不可用窗口的技术约束 |

## 用户旅程与任务成功

1. 用户在设置中选择 English，当前页面及后续导航页面立即显示英文；切回中文同样即时恢复。
2. 用户进入语音播报页：已有音色保持不变；没有已选值且系统返回 HuiHui 时，下拉框默认显示它。
3. 用户点击“重置默认”，确认后按钮进入保存中；成功时草稿与持久化快照一致并显示成功消息，失败时保留可编辑页面并显示失败消息，取消不产生任何变化。
4. 用户在上舰特效“视觉与位置”区域填写宽高并选择位置，保存或预览后弹窗按当前尺寸和位置出现。

## 信息架构与内容策略

- 全局页面：只把现有标题、字段、按钮、提示、空态和弹窗文字迁移到 locale，不改变阅读顺序。
- TTS：音色仍与引擎放在同一行；重置仍属于页头/表单次操作。
- 上舰特效：窗口宽高紧邻“显示位置”，归入“视觉与位置”，先决定模板/时长，再决定窗口几何，最后决定颜色。

## 布局结构

- 保留现有页面容器、表单网格、表格、抽屉与弹窗。
- 上舰特效在位置控件之后增加两列响应式宽高输入：窄屏单列，`sm` 及以上双列。
- 不嵌套新卡片，不增加浮动区块。

## 视觉层级与方向

- 延续安静、紧凑的桌面操作台风格。
- 宽高使用 Element Plus 数字输入，后缀显示 `px`，与现有表单标签层级一致。
- 成功/失败沿用全局 ElMessage，不新增常驻横幅。

## 区块拆解

- 国际化：页面标题、说明、字段、动作、状态、空态、确认框和 tooltip。
- TTS：引擎/音色、播报筛选、模板、重置反馈、队列。
- 上舰特效：视觉与位置、声音与连续触发；窗口宽高属于前者。

## 样式方向

仅复用已有 Element Plus 控件和 Tailwind utility class。无新 scoped CSS、CSS modules、内联 style 对象或语义 CSS 类。

## Tailwind 与 class 策略

- 新布局只使用短的响应式 grid utility。
- class 保持模板内可读，不移动到常量/computed/helper。
- 不为本次变更新抽取只负责隐藏 class 的组件。

## 交互骨架

- locale 选择：现有 select/change 流程，不增加确认。
- TTS reset：点击、确认、异步保存、成功/失败；保存中禁用重复提交。
- 宽高：数字输入、blur/change 校验由 Element Plus 范围限制与 domain 校验共同承担。
- effect preview/save：现有按钮和 pending 状态不变，携带完整新配置。

## 状态与反馈模型

- 初始加载：沿用现有 skeleton/loading。
- HuiHui 不存在：不伪造选项，保持未选中并允许用户选择可用音色。
- 重置 pending：复用 store `save` pending；成功 `ElMessage.success`，失败 `ElMessage.error`，取消静默。
- 尺寸无效：按钮禁用，并由 domain/adapter 防止非法值进入窗口层。
- 窗口 monitor 不可用：沿用可重试 media operation error，不退化到错误坐标。

## UX 文案立场

- 中英文文案语义对应，英文采用简短操作型表达。
- 后端原始错误仍可作为诊断回退；页面自有 fallback 文案由 locale 提供。
- 不新增帮助教程或说明卡。

## 响应式行为

- 原页面断点保持不变。
- 宽高在窄屏堆叠，避免数字输入压缩；桌面双列以减少纵向长度。
- 英文长标签允许表单自然换行，不固定窄标签宽度。

## 可访问性与键盘

- 使用原生表单标签语义和 Element Plus 键盘行为。
- icon button 的 tooltip/aria-label 同步国际化。
- 成功/失败消息由 Element Plus live region 提供；TTS 状态仍保留 `aria-live`。
- Canvas 的无障碍文本跟随 locale。

## 可用性风险与控制

- 长英文导致溢出：保留 flex-wrap/grid 响应式布局并运行桌面/窄屏检查。
- 默认音色覆盖用户选择：仅在 selectedVoiceId 为空时解析默认。
- 重置仅改 UI 未保存：强制复用 store.save 并用返回快照回填。
- 多显示器/缩放坐标错误：基于主显示器 logical geometry 与 scale factor 计算。
- 窗口超出屏幕：宽高范围校验并在坐标计算中饱和到显示器边界。

## 组件复用与拆分决策

- 复用现有 `DataRangeControl`、`TtsQueuePanel`、`CaptainEffectSurface` 等组件；它们已有明确业务边界。
- 页面 locale 接入保持在各现有 SFC 内，不新增一层纯透传翻译组件。
- 默认音色解析是跨“初始加载/重置”两处相同业务规则，放在 media domain helper；不是 Vue 组件抽取。
- 窗口宽高表单保持 `captain-effect/index.vue` 内联：单一页面、简单双字段、无独立生命周期。

## 设计风险

- 全页面文案迁移容易遗漏动态 tooltip、aria-label 和空态，需静态扫描与运行时抽查共同验证。
- Tauri 多显示器 API 的物理/逻辑坐标需 Rust 单测覆盖纯计算并在 Windows 实机预览。

## 未决 UI 问题

无。
