# Interaction Media 任务板

| ID | 任务 | 状态 | Readiness | 模式/前置 | Spec/规则 | 范围与 owner | Source / product delta | 测试切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| IM-T01 | 合同镜像、giftId 与红测 | done | ready | 串行 TDD / plan approved | IM-01..08、ADR-01/04/06 | Rust/TS DTO、contract maps、fixtures；先类型后 caller migration | PRD 9.7/9.8；仅 giftId，其他 none | serde、TS contract、surface count、raw-path scan |
| IM-T02 | Rust 纯领域规则 | done | ready | 串行 TDD / T01 | BF-01..06、IM-A01..08 | domain/media models/rules；pure owner | approved spec / none | validation/reducer/queue/transition negative tests |
| IM-T03 | SQLite、codec、path grant | done | ready | 串行 TDD / T02 | IM-03/05/06、ADR-04/07 | migration/repository/files/PathGrantStore | PRD 9.8/9.13/10 / none | temp DB、rollback、wrong-purpose、compensation |
| IM-T04 | 系统 I/O 适配器 | done | ready | 串行 TDD / T03 | IM-01..06、ADR-03/05/07 | ports + audio/TTS/FFmpeg/notify/window/hotkey/preview | approved behavior + official API / none | dependency gate、fake/unavailable、cleanup、range |
| IM-T05 | 四 runtime 与 event ingress | done | ready | 串行 TDD / T04 | BF-01..06、ADR-01/02 | MediaState + four runtime + AcceptedLiveEventConsumer | approved spec / none | accepted-only、priority/fallback/retry/waterline/shutdown |
| IM-T06 | IPC、事件、能力与 effect bootstrap | done | ready | 串行 TDD / T05 | IM-06、IM-A09..11、ADR-05/07 | 34 commands/7 events/free-member/effect zero command | approved contract / none | registry/capability/target/error/free-member checks |
| IM-T07 | TS adapter/store/root runtime | done | ready | 串行 TDD / T06 | IM-06/07、ADR-06 | immutable VM、four stores、one six-listener root | approved architecture / none | mapper/stale/dedupe/finally/late-cleanup/typecheck |
| IM-T08 | Captain/Gift 页面与 effect surface | done | ready | 串行 / T07 | BF-01..03、IM-A01..04 | three routes/pages + approved extracted components | page-design allowlist only | component/UI/canvas/FPS/DPI/screenshots |
| IM-T09 | TTS/Recording/Highlight 页面 | done | ready | 串行 / T08 | BF-04..07、IM-A05..09 | two routes/pages + queue/status/lists/settings/preview | page-design allowlist only | template/status/F9/preview/focus/responsive |
| IM-T10 | 删除闭包与全量验收 | done | ready | 串行 / T09 | IM-08、IM-A10..14、ADR-08 | old services/engines/stores/Electron/settings/tests + evidence | code facts/acceptance / none | exact scans、full suite、dual installers、runtime |

## 每任务执行字段

| ID | 触发/页面容器 | 目标文件与符号 | 关键交互/状态 | API/类型策略 | 清理/评审约束 |
| --- | --- | --- | --- | --- | --- |
| T01 | accepted event/desktop contracts | `GiftEvent.giftId`、serde DTO、contract maps/barrel/tests | invalid/null/raw path fail-fast | Rust authority，TS exact mirror | 迁移 constructors/imports/generated refs；无 runtime behavior |
| T02 | Rust domain only | `domain/media/*` pure types/rules | FIFO/priority/overflow/combo/heat/disk/transitions | newtype/enums、immutable inputs | 无 I/O/class manager；constants 最窄 owner |
| T03 | persistence/dialog grant | migration/repository/files/path purposes | atomic/rollback/cancel/revoke | opaque grant、versioned import | 无 raw path DTO/browser file；partial failure compensation |
| T04 | OS adapter boundary | ports/audio/speech/ffmpeg/notification/window/hotkey/preview | unavailable/permission/conflict/cancel/dispose | locked crates + structured errors | no business policy；handles must stop/join/revoke |
| T05 | accepted event internal trigger | MediaState/four runtimes/live-data consumer | accepted-only/dedupe/fallback/pause/stop/shutdown | domain models through ports | media failure isolated；no renderer projection |
| T06 | Tauri command/event/effect root | IPC/lib/capabilities/config/main bootstrap | targeted events/zero effect authority | exactly 34/7、DesktopError | free/member parity；no extra public surface |
| T07 | App root/store actions | media adapters/stores/useMediaRuntime | latest-wins/dedupe/finally/dispose | DTO->VM->draft/payload | components/pages cannot import transport/raw DTO |
| T08 | 提醒配置/effect window | captain/gift pages/components | dirty/pending/confirm/focus/fallback/missing | stores only | delete unapproved controls/browser audio/effect managers |
| T09 | TTS/直播录制 | tts/recording pages/components | validation/queue/status/start-stop/finalize/preview | opaque preview/session DTO | no pause-resume/open-folder/download/duplicate settings |
| T10 | all owned chains | exact files from before-scan | no dual path; external pending honest | all contracts frozen | no unrelated formatting/refactor/debug/dead/stale/mock |

## 通用执行合同

- 顺序固定 T01 -> T10；每项完成后立刻更新本表状态和 `execution/changelog.md`，可测试行为遵循 red -> green -> refactor。
- `S3 broad/full`，但 task context 只读目标、direct callers/consumers、focused tests、approved spec/page-design/ADR；不得改 PRD 或邻近 membership/plugin/update/ASR/AI/logger。
- 页面 practical goal 是可用的本地媒体工作流；禁止 fake success、Electron compatibility、renderer event projection、raw path、重复 writable owner。
- Robustness：DTO/storage/grant/file/process/OS/API 均 validate/catch；异步 `finally`；stale/cancel/partial failure/cleanup 明确。
- FP：rules/mappers/validators/formatters/payload 纯函数，输入不可变；side effects 仅在 approved runtime/port/repository/transport/store/lifecycle boundary。
- UI：Element Plus + Tailwind utilities；无 scoped/inline authored styling；1024x680/1280x800、keyboard/focus、stable loading/empty/error/degraded geometry。
- Review：函数不混合多职责；无 magic variables/generic manager/one-caller helper file；每个 hunk 映射 task/source/cleanup；结束前 exact import/export/caller/event/watch/timer/test/mock/comment scan。
- 待确认项：无阻塞产品问题。External captain assets/cloud TTS/bundled FFmpeg 保持 pending/unavailable。已批准技术选择仅为计划中锁定 crate/plugin 版本与测试 clock/token；若编译不兼容，走 unavailable evidence，不现场换栈。
