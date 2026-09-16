# Application Shell 任务板

| ID | 任务 | 状态 | Readiness | 模式/前置 | Spec/规则 | 范围与 owner | Source / product delta | 测试切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| AS-T01 | 合同、纯规则与红测 | done | ready | 串行 TDD / plan approved | AS-F01..08、A02..12 | Rust/TS DTO、13/3 maps、validators/navigation/status fixtures | PRD 8/9.17/9.20/9.21；approved fields only | serde/count/matrix/malformed/rules red tests |
| AS-T02 | Rust 持久化与配置事务 | done | ready | 串行 TDD / T01 | F05..07、A07..12、ADR-01/03/04 | domain shell + ShellRepository + ConfigArchiveV1 | approved spec / none | temp DB/roundtrip/rollback/incompatible/reset |
| AS-T03 | 系统/文件/目录 adapters | done | ready | 串行 TDD / T02 | F01/F04/F05、A03/A07/A13、ADR-06 | SystemMetrics/File/Directory ports + adapters | approved architecture / none | fake/unavailable/grant/args/shutdown |
| AS-T04 | ApplicationShellState | done | ready | 串行 TDD / T03 | F01/F04..08、A03/A07..13/A16 | one Rust state + 13 use cases + event sink | approved architecture / none | rollback/dedupe/order/target/shutdown |
| AS-T05 | Tauri IPC 与双通道注册 | done | ready | 串行 TDD / T04 | A04/A05/A12/A15/A16、ADR-02/05 | IPC/lib/capabilities/bootstrap | approved surface / none | exact names/count/free-member/source scans |
| AS-T06 | TS adapter/store/root runtime | done | ready | 串行 TDD / T05 | F01/F03/F04/F06/F07、A03/A06/A09..14 | readonly DTO/VM + store + one runtime/projection | approved architecture / none | malformed/stale/finally/dispose/rollback |
| AS-T07 | 四区根壳 | done | ready | 串行 / T06 | F01..04/F08、A01..06/A13/A14 | App + toolbar/nav/status components/router | page-design allowlist only | room/nav/status/a11y/responsive screenshots |
| AS-T08 | 配置管理 | done | ready | 串行 / T07 | F05、A07/A08/A13 | settings + ConfigManagementPanel + grants | PRD 9.17 allowlist only | cancel/confirm/pending/atomic/cleanup |
| AS-T09 | 语言、界面、布局 | done | ready | 串行 / T08 | F06/F07、A09..11/A14 | LanguageSettings + InterfacePanel + LayoutDialog | PRD 9.20/9.21 allowlist only | preview/rollback/fallback/keyboard/DPI |
| AS-T10 | i18n、删除闭包与验收 | done | ready | 串行 / T09 | A01..16、ADR-08 | all owned chains + execution evidence | approved copy/acceptance; none beyond | exact scans/full suite/dual installers/runtime |

## 每任务执行字段

| ID | 触发/页面容器 | 目标文件与符号 | 关键交互/状态 | API/类型策略 | 清理/评审约束 |
| --- | --- | --- | --- | --- | --- |
| T01 | approved contract boundary | shell domain/TS contracts/tests | invalid/null/free-member fail-fast | Rust serde authority, readonly TS mirror | no I/O/behavior invention; type first |
| T02 | config/locale/UI persistence | rules/repository/archive | validate -> transaction -> commit/rollback | versioned closed archive | no raw schema/secrets/generic repo manager |
| T03 | metrics/file/open effects | ports/sysinfo/file/opener | warmup/unavailable/failure/dispose | typed ports, cfg OS adapter | no shell interpolation/raw path/fake zero |
| T04 | Rust use-case intents | ApplicationShellState/use cases | saved/draft preview/rollback/events | immutable snapshots | room/media remain external truth; no mega state |
| T05 | invoke/listen/bootstrap | IPC/lib/capabilities | targeted events/build projection | exactly 13 commands/3 events | free build member absence; no aliases |
| T06 | App root lifecycle | adapters/store/useRuntime/projection | latest wins/dedupe/finally/dispose | DTO -> adapter -> VM | no page Tauri/raw DTO/duplicate writable owner |
| T07 | root window | App/toolbar/navigation/status/router | connect/nav/route fallback/seven status | store view models only | Tailwind utilities; a11y; stable geometry |
| T08 | settings/config | ConfigManagementPanel/settings | scopes/grant/cancel/confirm/pending/refresh | grant-id-only actions | delete Blob/object URL/paste/localStorage chain |
| T09 | settings/locale/interface | language/interface/layout/projection/i18n | saved-draft-preview-save-reset/cancel | exact enums/bounds/defaults | no custom widget/theme/font expansion |
| T10 | all shell chains | source/tests/build/artifacts | verify -> classify -> owning re-entry | frozen contracts | no debug/dead/stale/orphan/hidden runtime path |

## 通用执行合同

- 顺序固定 T01 -> T10；每项完成后更新本表与 `execution/changelog.md`，可测试行为遵循 red -> green -> refactor。
- `S3 broad/full`，但每项只读目标、direct callers/consumers、focused tests 与 approved artifacts；不修改 PRD 或后续模块业务。
- Robustness：外部 DTO/storage/grant/file/OS/locale/UI input validate/catch；pending/failure/cancel/stale/partial success/finally 明确。
- FP：rules/mappers/validators/formatters/payload/reducer 纯函数、不可变输入；effects 只在 approved port/repository/IPC/store/lifecycle/event boundary。
- UI：Element Plus + Tailwind utilities；无 scoped/inline authored styling；键盘/focus/aria、1024x680/1280x800、高 DPI、stable boundary states。
- Review：无 >150 行混责函数、magic variables、generic manager/one-caller helper；每个 hunk 映射 task/source/cleanup，结束跑完整 import/export/caller/event/watch/timer/test/mock/comment 闭包扫描。
- 待确认项：无产品 blocker。已批准技术假设仅 `sysinfo 0.30.13` 单实例 bounded sampling；不兼容时保持 metrics unavailable 并记录证据，不现场换栈。
