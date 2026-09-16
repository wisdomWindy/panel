# Module: Application Shell

## Boundary

PRD 功能 19、20 与 16 的配置 UI：四区主窗口、动态导航、状态栏、主题/布局、设置、多语言。

## Raw Source Snapshot

- 8：默认 1280x800、最小 1024x680；toolbar/nav/content/status 四区。
- 8.1-8.4：免费、会员未付费、激活、到期四类可见性。
- 8.5-8.7：room/连接/member/通知/设置、七状态项、动态 nav 矩阵。
- 9.16：配置导入/导出/重置与选择范围。
- 9.19：中文/英文、系统语言、即时切换、缺失回退中文。
- 9.20：主题、强调色、透明度、字体、布局、弹幕显示项。

## Markdown Normalization

不适用：来源 wireframe 已在 Markdown code fence 内，表格可直接消费。

## Source Grounding And Allowlist

- 允许页面结构和可见性只按 8/9.16/9.19/9.20。
- 免费包绝不显示登录/会员/AI/续费；会员态 UI 只反映 Rust capability snapshot。
- 主题与布局可自定义，但不得创造 PRD 未列出的营销/说明页面。

## Responsibilities

- Vue：responsive shell、routes/nav、状态消费、i18n、theme/layout/settings UX。
- Rust：window/system metrics、persisted UI config、locale error code、member capability snapshot。

## Acceptance

- 桌面最小尺寸无重叠；高 DPI、多窗口正常；中英文完整；免费/会员矩阵准确；配置即时生效且可恢复。

## Downstream Obligations

- page_design_required: `true`
- architecture_design_required: `true`
- entry_stage: `page-design`
- execution_order: `4`
