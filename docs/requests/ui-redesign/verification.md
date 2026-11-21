# 验证记录：主界面按 PRD v2 与设计图重构

日期：2026-11-21 ｜ 输入：`docs/requirements/`（PRD v2 + 8 张设计图 + HTML 原型）

## 1. 本轮完成

| 项 | 证据 |
| --- | --- |
| 设计令牌 | `src/renderer/styles/theme.css`，色值与 `ui/bilibili-live-helper-ui.html` 逐项一致（`--app-bg/#13151a` 等），并覆写 Element Plus 深色变量 |
| 顶部工具栏 | 重写 `AppShellToolbar.vue`：48px、品牌、房间输入、连接按钮（含状态点）、通知/设置图标 |
| 左侧导航 | 重写 `AppShellNavigation.vue`：自定义导航项 + 激活左色条，208px，分组悬停展开子项 |
| 底部状态栏 | 重写 `AppShellStatusBar.vue`：7 字段、状态点、竖直分隔线 |
| 四区布局 | `App.vue` 网格行高改为 48/1fr/32，壳层背景统一走令牌 |
| 首屏「直播间」 | 新增 `views/live-room/index.vue`：房间信息 + 4 指标卡 + 最近事件流 |
| 路由 | `/` 重定向到 `/live`；导航首项 `liveRoom → /live` |
| 国际化 | 新增 `liveRoom.*`（13 键）与 `shell.navigation.liveRoom`、`liveData.tags.ordinary`，中英双语对齐 |

## 2. 验证

| 检查 | 命令 | 结果 |
| --- | --- | --- |
| 前端测试 | `pnpm exec vitest run --mode free` | ✅ **48 files / 154 tests passed** |
| 类型检查 | `pnpm exec tsc --noEmit -p tsconfig.eslint.json` | ✅ exit 0 |
| 生产构建 | `pnpm build:web:free` | ✅ `built in 6.55s` |

基线对照：改造前 47 files / 151 tests 通过。新增 3 项 `live-room-page.test.ts`；
`shell-components-contract.test.ts` 与 `shell-contract.test.ts` 的 4 处断言按新契约更新（未删除断言，改为表达新结构）。

## 3. 未完成（如实说明）

PRD 共有 10 个导航页，本轮完成**外壳 + 直播间**两个层次，其余页面仍是旧版内容，仅通过 Element Plus 深色变量继承新配色：

| 页面 | 状态 |
| --- | --- |
| 弹幕面板 | 旧模板（有筛选/搜索/清空，但无设计图里的身份 chip 与「身份图例」侧栏） |
| 数据看板 / 排行榜 | 旧模板，配色随令牌 |
| 风控中心 / 提醒配置 | 仍为两个/三个独立菜单项，未合并为分组页 |
| 语音播报 / 直播录制 / AI 助手 / 系统设置 | 旧模板，配色随令牌 |

## 4. 下一步（按设计图逐页对齐）

1. 弹幕面板：身份 chip 过滤 + 身份图例侧栏 + 行内徽标（`preview-danmu.png`）。
2. 数据看板/排行榜：指标卡与榜单表格按 `preview-board.png`、`preview-rank.png` 重排。
3. 风控中心、提醒配置、系统设置：合并为分组页，子页签 + 二级路由。
4. 录制/AI/播报：按 `preview-record.png`、`preview-ai.png`、`preview-tts.png` 重排版式。

> 说明：设计图中的「新增关注」指标后端当前无数据来源（WebSocket 仅提供进入事件），首页第 4 张卡改为「在线人数」，未伪造数据。
