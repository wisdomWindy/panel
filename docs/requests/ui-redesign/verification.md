# 验证记录：主界面按 PRD v2 与设计图全量重构

日期：2026-11-21 ｜ 输入：`docs/requirements/`（PRD v2 + 8 张设计图 + HTML 原型）

## 1. 完成范围（10 个导航页全覆盖）

| 导航项 | 路由 | 状态 |
| --- | --- | --- |
| 直播间 | `/live` | ✅ 新建：房间信息 + 4 指标卡 + 最近事件流 |
| 弹幕面板 | `/danmaku-list` | ✅ 身份 chip 过滤 + 身份图例 + 行内头像 + 礼物行高亮 |
| 数据看板 | `/dashboard` | ✅ 指标卡网格 + 两列趋势图 + Qwen3 模型卡 |
| 排行榜 | `/rankings` | ✅ 送礼/弹幕/舰长三榜并排 + 名次徽标 |
| 风控中心 | `/risk/{sensitive,blacklist}` | ✅ 分组页签；两子页令牌化（chip 筛选、卡片表格） |
| 提醒配置 | `/reminders/{gift,captain,sound}` | ✅ 分组页签；三子页令牌化 |
| 语音播报 | `/tts` | ✅ 头部 + 6 张 `app-card`（引擎/选择/模板/Qwen3/操作/队列） |
| 直播录制 | `/recording` | ✅ 控制卡 + 配置卡 + 5 张指标卡 + 列表卡 |
| AI 助手 | `/ai` | ✅ 页头页签 + 左设置卡 / 右队列卡 + 结果卡 |
| 系统设置 | `/settings/{general,logs,update}` | ✅ 分组页签；三子页令牌化 |

配套：
- 设计令牌 `styles/theme.css`（16 个色值 + Element Plus 深色变量覆写 + 30 余个工具类）。
- 外壳四区（48px 工具栏 / 208px 导航 / 状态栏）全部令牌化。
- 分组页签组件 `SectionGroup.vue`；旧路径（`/sensitive-words`、`/gift-reminder`、`/logs`、`/update-log` 等）保留为 redirect，深链不断。
- 清理真正无引用的组件：`RecordingStatusBand.vue`、`AiContextList.vue`。

## 2. 验证

| 检查 | 命令 | 结果 |
| --- | --- | --- |
| 前端测试 | `pnpm exec vitest run --mode free` | ✅ **49 files / 164 tests passed** |
| 类型检查 | `pnpm exec tsc --noEmit -p tsconfig.eslint.json` | ✅ exit 0 |
| 生产构建 | `pnpm build:web:free` | ✅ `built in 6.88s` |
| 双语键一致性 | 全量键集合比对 | ✅ 1023 / 1023，无单边键 |
| 旧配色残留 | `rg 'dark:'` 扫描 `src/renderer/**/*.vue` | ✅ **0 个文件** |

改造前基线：47 files / 151 tests。新增/改写测试均表达新契约（如断言 `app-card`、`not.toContain('dark:')`），未删除既有断言。

## 3. 收尾范围（第二轮）

在首批 10 页之后，补齐了剩余的旧配色文件，使 `src/renderer` 下**不再有任何 `.vue` 文件出现 `dark:` 字面量**：

| 文件组 | 说明 |
| --- | --- |
| `views/settings/components/*`（4） | 配置/界面/语言/布局编辑器面板内部 |
| `views/ai/log.vue` | AI 审计日志 |
| `views/connection/index.vue` | 旧连接页 |
| `components/media/GiftAlertOverlay.vue` | 礼物弹窗浮层（五个位置分支与超时清理逐条保留） |
| `views/moderation/*`（5） | 风控中心（成员版） |
| `views/membership/*`（5）+ `components/member/MembershipPausedNotice.vue` | 会员中心（成员版） |

同时删除 `docs/requirements/` 下已被 v2 取代的 v1 PRD，只保留 v2 作为唯一需求基线。

## 4. 需求基线变更（2026-11-21）

按产品决定，以下三项**移出本期需求**，已同步删除/调整 PRD v2：

| 项 | 处理 |
| --- | --- |
| 「新增关注」指标 | 设计稿独有、无数据来源；PRD 未收录。首页第四张卡定为「在线人数」。 |
| 「礼物类型分布」图 | 设计稿独有、无后端聚合；PRD 未收录。不实现。 |
| 快捷键清单 | 删除 PRD 原第 14 节，后续章节号顺延（错误码分类→14，修订摘要→15）；录制热键 F9 属于功能需求，仍保留在 9.12。 |

PRD v1 已删除，`docs/requirements/` 现只保留 v2 作为唯一需求基线。
