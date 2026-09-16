# Application Shell 执行记录

## 执行上下文

- self-healing iteration: `7`
- re-entry source: `gate_recovery`
- approved tasks: `AS-T01..AS-T10`
- stage: `execute`

## 变更前链路审查

- 根入口：`main.ts -> App.vue -> router -> feature pages`。`App.vue` 当前直接持有房间草稿、连接动作、硬编码导航/状态和更新弹窗生命周期。
- 设置入口：`router /settings -> views/settings/index.vue -> legacy config/AI/ASR/moderation/update/plugin children`。页面直接创建 Blob/object URL/anchor 并接收粘贴 JSON。
- 配置 owner：`stores/config.ts` 与 `i18n-runtime.ts` 直接写 localStorage；本模块将迁移 shell locale/UI/config authority，日志/更新相邻字段仅在后续 owner 可用前保留其真实生产调用方。
- Native 链：typed desktop contracts/transport -> Rust IPC -> `AppState` -> application/domain/ports -> SQLite/path grants。新 shell surface 必须沿该方向接入。
- 邻接只读 owner：`useLiveRoomStore`、`useLiveFeedStore`、`useMediaRecordingStore`；不得复制其 writable domain state。
- 测试引用为适配面，不是旧生产链保留理由。

## 任务进度

- AS-T01: done
- AS-T02..AS-T10: done

## TDD 记录

- AS-T01 red: focused Vitest 因 shell contract 与纯函数模块尚不存在按预期失败。
- AS-T01 first green: 5 个测试中 4 个通过；free bundle 断言的 `/ai/i` 误命中 `captainEffect`，已收窄为精确禁止 token。
- AS-T01 green: TS focused 5/5、type-check pass；Rust free focused 4/4 pass。free cfg 的 `unused_mut` 已通过显式 cfg 返回分支修复，不压制 lint。
- Plan-gap recovery: 执行发现配置 scope 必须从技术分组改为 PRD 9.17.3 的精确七项免费/两项会员集合；已回补 plan，不改变 spec 或产品范围。
- AS-T02..T05 green: Rust 持久化、事务导入、native file/directory/system metrics/system locale adapters、单一 ApplicationShellState、13 commands/3 events 与 free/member 注册完成；会员 AI/禁言配置在下游 producer 接入前显式 `isAvailable=false`，导入导出 fail closed。
- AS-T06..T07 green: readonly adapters/Pinia/runtime/projection 与 toolbar/navigation/content/status 四区根壳完成；root 只做运行时编排，房间/feed/recording 继续由邻接 store 持有；会员 unavailable 降级到精确免费导航。
- AS-T08..T09 green: settings 收敛为配置管理/语言/界面；native grant 导入导出、取消 no-op、reset 二次确认、OS locale、saved/draft/preview/save/reset 与四区布局边界完成；旧 Blob/object URL/pasted JSON/localStorage 设置入口删除。
- AS-T10 green: 中英壳层词条、编译期导航短路、免费 bundle 会员 route scan、orphan settings deletion 与全量验证完成。

## 验证证据

- Vitest: `--mode free` 25 files / 65 tests pass；`--mode member` 25 files / 65 tests pass。
- TypeScript: `pnpm type-check` pass。
- ESLint: 0 errors；77 warnings 均位于未进入当前模块清理范围的 legacy logger/web-speech/plugin files。
- Rust tests: free 61 unit + 18 foundation contracts pass；member 61 unit + 19 foundation contracts pass；focused shell 双通道各 8 pass。
- Rust static: `cargo fmt --check` pass；free/member `cargo clippy --all-targets -- -D warnings` pass。
- Web builds: free/member pass；仅保留既有主 chunk >550kB warning。
- Tauri builds: free/member 均生成 MSI + NSIS，共四个 bundles。
- Static cleanup: owned production files无 `localStorage`, `createObjectURL`, `new Blob`, pasted JSON 或 `useConfigStore`；grant revoke 与 timer/listener dispose 均有生产调用和测试。
- Bundle isolation: final free Web bundle中无 `/membership`, `membership_purchase`, `member.ai.` 或 `member.moderation.`。
- Visual: browser degraded state四区壳与设置 rail 可渲染，文案完成 i18n；低于 PRD 1024px minimum 时按 desktop minimum constraint 横向保护。

## Introduced Content Provenance Ledger

- AS-T01 tests/contracts: product-content delta limited to PRD 8 / 9.17 / 9.20 / 9.21 and approved spec AS-F01..08.
- Execution/process artifacts: product-content delta: none; source is approved plan and execute-stage recording contract.

## Frontend Architecture Quality Execution Notes

- One writable shell owner remains the target; no implementation exception recorded.
- Dependency direction and 13/3 public-surface budget remain unchanged.

## 风险与下一验证重点

- Verify re-entry `logic-error/status danmaku rate clock`, same-stage attempt 1 / total attempt 1: cached computed used non-reactive `Date.now()` and could retain a stale positive rate after traffic stopped. Added a shell-owned one-second observation clock advanced even when metric sampling fails, made rate derivation depend on it, and added a red-to-green store regression test.
- 实际 native 文件对话框、目录打开、窗口 opacity/high-DPI 交互未进行人工点击；由 command/port/store tests、双 installer build 与 Web degraded visual evidence 覆盖，保留为 verify manual residual。
- Member/notification producer 仍属于后续模块；本模块只保留 fail-closed seam，不声明其业务已完成。

## Verify Re-entry Completion

- `logic-error/status danmaku rate clock`, attempt 1/1: added the reactive observation clock and regression coverage; full suites remain green.
- `logic-error/UI preview concurrency`, attempt 1/1: removed same-key preview deduplication, added latest-wins generation handling, ignored stale failures, invalidated previews on save/reset/dispose and kept controls interactive during previews.
- `change-chain-error/config mutation refresh`, attempt 1/1: added a scope-driven refresh coordinator for the real safety, gift-media and recording snapshots after successful import/reset. Interface/locale refresh remains shell-owned; room/plugin have no reloadable renderer snapshot and are not falsely reported as refreshed.
- `logic-error/UI preview event semantics`, attempt 1/1: preview no longer emits the committed `ui_preferences_changed` event; only persisted changes use that event.
- `requirement-mismatch/member reset result`, attempt 1/1: reset now returns only executable scopes. Member AI/moderation remain visible but unavailable in overview and fail closed in repository operations.
- Final evidence after re-entry: Vitest free/member 26 files / 70 tests each; TypeScript pass; ESLint 0 errors / 77 neighboring warnings; Rust free 61+18 and member 61+19 pass; fmt and strict free/member Clippy pass; free/member Web and Tauri builds pass; four installers regenerated at 2026-09-15 18:38-18:39.

## Review Re-entry Completion

- `code-organization-error/single-caller config refresh module`, attempt 1/1: removed the standalone helper and colocated scope-driven refresh orchestration with its sole configuration-panel caller.
- `requirement-mismatch/layout interaction`, attempt 1/1: added pointer-captured region separators, arrow-key resizing, responsive preview variables and one shared bounded draft; aligned the navigation default to the approved 208px value.
- `logic-error/locale save rollback`, attempt 1/1: a failed locale save now restores both the global projection and the selector draft to the persisted locale.
- `logic-error/bootstrap-before-reveal`, attempt 1/1: root shell state starts pending, invokes `app_bootstrap` with shell reads once per generation and preserves geometry behind the Element Plus loading mask.
- `logic-error/post-commit config refresh`, attempt 1/1: successful native import/reset remains successful if a subsequent preference or feature snapshot read degrades; refresh errors remain structured and grants still revoke in `finally`.
- `field-semantics-error/backup timestamp`, attempt 1/1: negative timestamps fail adaptation and localized backup times now include seconds.
- `requirement-mismatch/notification trigger`, attempt 1/1: replaced the empty handler with shell-owned anchored open/close/focus state and a stable empty downstream content boundary without inventing notification entries or copy.
- Final evidence after review re-entry: Vitest free/member 25 files / 69 tests each; TypeScript pass; ESLint 0 errors / 77 neighboring warnings; Rust free 61+18 and member 61+19 pass; fmt and strict free/member Clippy pass; exact free bundle scan clean; free/member Web and Tauri builds pass; four final installers regenerated at 2026-09-15 19:23-19:25.
