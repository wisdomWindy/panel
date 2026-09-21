# Page Design: shell-connection-redesign

## Page Objective

把截图中的近似空白首屏重构为紧凑、安静、适合长时间使用的直播运营工作台。用户进入应用后，应先看到当前连接状态和房间连接入口，其次看到导航与直播间摘要，底部始终保留运行状态。[source-backed: U1, U2; allowlist P01-P05]

## Target Users / Scenarios

- 主播或直播运营人员启动桌面应用，输入房间号并连接。[P02]
- 用户在直播间、弹幕、看板、风控、提醒、录制、插件和设置之间切换。[P01, P03]
- 用户在未连接、连接中、已连接、重连或失败时快速确认当前状态。[P01, P04]

## Source And Upstream Constraints

- 用户截图是失败基线，目标是“正确展示、功能正常使用”。
- 现有 shell 已定义品牌、连接控件、导航、通知、设置和七项状态栏；connection view 已定义状态和三个摘要字段。
- 不新增业务内容，只改变布局、层级、视觉样式和现有状态的呈现。
- Figma MCP 不可用；本文件是本次可追溯的设计合同，不代表已创建 Figma 文件。

## Product-content Allowlist Mapping

| UI 内容 | 来源 |
| --- | --- |
| 品牌图标与 `Bilibili 直播助手` | P01；现有应用名与 Tauri icon |
| 房间号选择、连接/断开 | P02 |
| 通知、系统设置 | P03 |
| 侧边导航全部既有条目 | P01 |
| `直播间` 与连接状态 | P01, P04 |
| 房间号、房间名称、在线人数 | P04 |
| 底部七项状态 | P01 |
| 浅色/深色视觉状态 | P05 |

## User Journey And Task Success

1. 应用启动后壳层立即可见，不等待桌面 bootstrap 才出现。
2. 顶部工具栏以品牌、房间输入、连接主操作、通知/设置的顺序建立清晰操作路径。
3. 左侧导航显示当前位置并支持既有分组展开；视觉上不抢夺连接主任务。
4. 首屏先显示页面标题和当前连接状态，再显示三个房间摘要字段。
5. 底部状态栏持续显示连接、房间、弹幕速率、在线、录制、内存和 CPU。
6. 连接中禁用重复提交；连接失败沿用现有错误提示；用户可修改输入后重试。

完成信号：首屏无需猜测即可识别“应用是什么、当前是否连接、在哪里输入房间号、如何连接、当前房间数据是什么”。

## Information Architecture And Layout

- Root：三行应用壳，顶部 64px 工具栏，中部内容区，底部 36px 状态栏。
- Toolbar：品牌块（左）/ 房间连接控件（中）/ 工具入口（右），使用边框和留白分组，不使用浮动卡片。
- Body：左侧使用现有 `--shell-navigation-width`（默认 208px）导航 + 右侧可滚动内容。小于 768px 时导航收为 72px 图标栏，主内容保持最小可用宽度。
- Connection page：标题行、状态带、三个同级摘要卡。无卡片嵌套。
- Status bar：连接状态作为首项，其余指标横向排列，溢出时可水平滚动。

## Visual Hierarchy / Direction

- 视觉语气：中性浅灰工作区 + 白色工作表面，深色模式使用 zinc 中性色，不使用大面积蓝黑单色。
- 品牌信号：现有黄/青 icon 在首屏工具栏清晰可见，产品名紧邻。
- 主色：连接主操作沿用 Element Plus 蓝色；在线成功用 emerald，等待/重连用 amber，断开/错误用 rose/red，并同时保留文字或图标，不依赖颜色单独表达。
- 字体层级：产品名和页标题克制；摘要值较大但不使用 hero 尺寸；元数据保持紧凑。
- 形状：卡片圆角不超过 8px；按钮继续使用既有 Element Plus 规格；无渐变、装饰圆球或嵌套卡片。

## Section Breakdown

- AppShellToolbar：真实品牌图标、产品名、房间选择、连接按钮、通知按钮、设置按钮。
- AppShellNavigation：当前项高亮、图标与文本对齐、分组项可展开。
- Connection：页面标题和状态、连接状态带、房间号/名称/在线人数三个摘要卡。
- AppShellStatusBar：七项指标，连接点与文本双重状态表达。

## Styling Constraints

- 只使用 Tailwind CSS-style utility classes，不新增 scoped CSS、CSS module、Sass/Less 或 inline style object。
- Element Plus theme variables 仅通过局部继承的 arbitrary-property utilities 控制，避免全局样式副作用。
- class 值保持在模板中可审查；通过现有组件边界和短小子容器降低 class 长度，不把 class 字符串藏入常量/computed/helper。
- 条件 class 仅用于当前连接态/选中态的小型颜色与强调切换。

## Interaction Skeleton / State Feedback

- 房间选择：保留 filterable、allow-create、default-first-option 和现有校验 tooltip。
- 连接：pending 使用原 `loading`，connecting 保留禁用；connected/reconnecting 时显示断开动作。
- 导航：保留 click emit、disabled 与 `aria-current`；不改变路由树。
- 通知：保留 popover 与关闭后焦点恢复；不新增通知内容。
- 设置：保留路由跳转。
- shell bootstrap：壳层保持可见，使用 `aria-busy` 表达初始化，不用全屏 loading 遮罩制造空白感。
- connection error：保留现有不可关闭 error alert。

## Responsive Behavior

- >= 768px：使用现有可配置导航宽度和文字导航，工具栏单行，摘要根据容器宽度切换一列/三列。
- 小于 768px：导航收为 72px 图标栏且文本保持 screen-reader 可读，连接页摘要单列，工具栏不重叠；应用仍以桌面最小宽度为主，不发明移动端抽屉行为。

## Accessibility / Keyboard

- 所有 icon-only 按钮保留 tooltip 与 `aria-label`。
- 当前路由保留 `aria-current=page`；当前状态有文字，不只靠颜色。
- 连接输入使用既有 aria-label/aria-invalid；错误提示由 tooltip 和页面 alert 承担。
- 焦点顺序按品牌后的连接输入、连接按钮、通知、设置、导航、内容；popover 关闭后恢复触发按钮焦点。
- 颜色对比以 zinc 文本配白/深色背景；深色 connection page 不再使用 `text-gray-900`。

## Component Reuse And Split Strategy

- 复用并修改现有 `AppShellToolbar`、`AppShellNavigation`、`AppShellStatusBar` 与 connection page，不新增 SFC。
- 每个现有组件已有清晰单一 owner 和独立测试面，保留边界。[code-fact-backed]
- connection page 的状态带与三个摘要块保持同文件内联：仅一个真实生产调用方、无独立生命周期，抽取会增加 props 跳转。
- 不创建 logo wrapper、metric card、status badge 等单调用方透传组件。

## UX Risks And Controls

- 风险：深色模式 Element Plus 与 Tailwind 表面不一致。控制：在 shell 容器局部设置必要 CSS variables，并以浏览器深色实测。
- 风险：窄视口导航文字挤压主内容。控制：在 `lg` 以下变为固定 72px 图标栏。
- 风险：连接入口在页面和工具栏重复。控制：只保留现有工具栏输入/按钮，connection page 专注状态与摘要。
- 风险：初始化遮罩让用户再次看到空白。控制：去掉 root 级遮罩，保留可访问的 busy 状态。

## Open UI Questions

- `missing-source`：没有获授权的新空状态插画、帮助文案、快捷操作或统计字段，本设计不添加。
- Figma MCP 输出不可用，不阻塞本地设计与实现，但最终必须披露。
