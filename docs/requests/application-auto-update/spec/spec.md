# 应用自动更新与检查更新 - 功能规格

## Goal And User Intent Contract

- Literal request：添加自动更新和检查更新功能（SRC-01）。
- Practical goal：用户无需知道隐藏路由或离开应用，启动后即可被提醒新版本，也能主动检查并完成安全更新（SRC-01..05）。
- Forbidden interpretations：不将“自动更新”解释为未经确认的静默安装；不绕过 Tauri 签名校验；不新增更新服务器、新频率选项或插件更新。
- Success criteria：启动检查、全局弹窗、设置页手动检查和现有下载/安装链路共用一个 store 且完成可验证。

## Source Grounding And Product Allowlist

| Spec item | Label | Source |
| --- | --- | --- |
| Startup performs one channel-aware update check | `source-backed` | SRC-01, SRC-02, SRC-03 |
| Offline startup check is non-blocking and silent | `source-backed` | SRC-02 |
| Available update opens a global release-notes dialog | `source-backed` | SRC-02, SRC-04 |
| User-confirmed download, signature verification and install/restart | `source-backed` | SRC-05 |
| Settings About area exposes a manual check control | `source-backed` | SRC-01; exact placement uses the existing version owner as `code-fact-backed` constraint |
| Current/available/failure manual feedback | `source-derived` from the requested manual operation; wording is limited to factual operation outcome |
| Reuse `UpdateSnapshot` and existing native commands | `code-fact-backed` | scoped code-context |
| Forced update cannot be dismissed or skipped | `source-backed` | PRD forced-update rule plus existing `isForced` contract |

No other product-facing control, setting, navigation item, default, persistence rule or analytics event is allowed.

## End-To-End Journeys

### Startup Check

1. The main renderer mounts and starts the singleton release runtime.
2. Runtime bootstraps native release state, then registers native release listeners.
3. Runtime invokes one `checkUpdate()` for the current process lifecycle.
4. When the check is offline/unavailable, the runtime remains active and other application startup work is unaffected. The error stays diagnosable in the release store but no startup error toast/dialog is shown.
5. When status becomes `available`, `downloading`, or `downloaded` with a candidate, the root application opens `UpdateAvailableDialog`.
6. Download/install/skip actions invoke only the release store actions. Native code remains responsible for candidate validation, signature validation, rollout and latest-operation protection.

### Manual Check

1. User opens Settings -> About and License.
2. User activates the semantic Element Plus button labelled `update.checkUpdate`.
3. While `pending` contains `checkUpdate`, the button shows loading and cannot start a duplicate operation; store-level dedupe remains the final guard.
4. `current` shows the localized latest-version success result.
5. Candidate states show the localized available result and the root dialog opens from the same store transition.
6. Failure shows the normalized localized/native error message and leaves the button retryable after pending clears.

## UI And Display Contract

| Field/control | Meaning and source | Display contract |
| --- | --- | --- |
| Application version | Installed binary version from `AboutSnapshot` | Prefix with `v`; `--` only before bootstrap. Not clickable. |
| Channel | Native build channel `free|member` | Plain text, not editable or clickable. |
| Check update button | User-triggered version check (SRC-01) | Existing Element Plus button, Refresh icon, loading during `checkUpdate`, keyboard/focus behavior supplied by semantic button. |
| Manual outcome | Result of the user's explicit check | Success toast for current/available, error toast for failure; no persistent duplicate banner is introduced. |
| Update dialog | Available candidate and release notes (SRC-04) | Existing component and responsive width. Forced candidates disable all ordinary dismiss paths. Non-forced candidates retain Later and Skip. |
| Progress | Native download completion percentage | Existing progress bar when non-null. |

Authored layout changes use inline Tailwind utilities only and stay within the existing About section hierarchy. No custom CSS or nested card is introduced.

## State And Async Contract

- Mutable update state owner: `useReleaseStore.update` only.
- Operation state owner: `useReleaseStore.pending`; `run(key, effect)` deduplicates equal keys.
- Root dialog visibility is local UI state in `App.vue`, derived/reacted from candidate status; it does not copy the candidate payload.
- Startup check sequencing: bootstrap -> listener registration -> mark runtime active -> auto check. Registering first prevents missing emitted status events.
- Auto-check failure is intentionally caught at the lifecycle boundary to satisfy offline-silent behavior; `store.error` still contains `normalizeDesktopError` output.
- `start()` remains idempotent through `active`/`pending`; `stop()` invalidates generations and disposes listeners/store state.
- Native `ReleaseState` generation protection and renderer `run()` dedupe remain the race policy. No timer, polling, debounce or cache is added.
- `null` candidate means no actionable version; `undefined` is not admitted by `UpdateSnapshot`. Dialog must not open without a candidate.

## Contracts And Responsibilities

No IPC or DTO additions are required. Renderer code reuses:

```ts
type UpdateStatus = 'unavailable' | 'current' | 'available' | 'downloading' | 'downloaded' | 'failed';

interface UpdateSnapshot {
  readonly currentVersion: string;
  readonly channel: 'free' | 'member';
  readonly status: UpdateStatus;
  readonly candidate: UpdateCandidate | null;
  readonly progressPercent: number | null;
  readonly errorCode: string | null;
}
```

- Rust/Tauri owns transport, manifest validation, signature verification, channel selection, rollout, pause, blacklist, forced update, download and install.
- Store owns renderer state, pending/error normalization and command invocation.
- Runtime owns startup sequencing and event subscription lifecycle.
- Root application owns global dialog presentation and its action orchestration.
- Settings page owns explicit manual-action feedback only.

## Component And Locality Decisions

- Reuse `UpdateAvailableDialog.vue`; it already owns the cohesive update interaction and has two real production consumers after this change (`App.vue` and `/update-log`).
- Keep root action handlers in `App.vue`; they are page-local orchestration and do not justify another composable or utility.
- Keep settings manual-check handler in `settings/index.vue`; it has one production caller and no reusable business transformation.
- Do not extract status predicates unless implementation demonstrates repeated non-trivial semantics. A local status check is preferable to a one-caller helper file.
- No new Vue SFC is introduced. Existing dialog props/emits/state ownership remain unchanged.

## Pattern-Fit Decision

- Concrete problem: coordinate one lifecycle-triggered side effect and two UI consumers over existing shared release state.
- Depth: Level 2.
- Decision: reuse existing project pattern.
- Pattern: singleton lifecycle composable + Pinia store actions + existing observer/event bridge.
- Frontend shape: composable lifecycle, store action, root component dialog boundary.
- Rationale: the existing release runtime already owns bootstrap/subscriptions and the store already owns command dedupe; extending those owners is the lightest structure that prevents duplicate update state.
- Rejected triggered alternative: a second update service/composable would create parallel command/state ownership; direct checks in `App.vue` would leak lifecycle sequencing into the root component.
- Overbuilt signal: any new manager, event bus, strategy/factory or polling layer with only this one update channel behavior.

## TypeScript And Production Quality Constraints

- Governing compiler context is `packages/bilibili-live-tool/tsconfig.json`; `tsconfig.eslint.json` governs the explicit type-check command. Strict nullability, unused checks, bundler resolution and `@/*` alias must pass.
- Reuse `UpdateSnapshot` from the desktop contract and the existing Vue shim; no broad `any`, assertion-based state suppression or duplicate DTO is permitted.
- Fail fast on absent candidate before download/install/skip handlers; native errors must remain normalized through store `run()`.
- Boolean names use `is*`/`has*`; event handlers use `handle*` for new code.
- No classes, memoization, watchers with duplicated effects, or global mutable flags.
- Automatic offline failure may be silent to the user only at startup; manual failure must be actionable.
- Existing user work in the dirty tree must be preserved; no broad formatting or unrelated cleanup.

## Architecture Design Routing

`architecture_design_required=true` because the change coordinates runtime lifecycle, shared store state, global UI and a page-local manual entry. Architecture design must decide exact ownership/dependency direction and migration of existing `/update-log` behavior, but may not change the source-grounded user behavior, native update contract or security boundary.

## Edge Cases

1. Base/dev configuration has no updater endpoint: startup remains usable; manual check reports the normalized error.
2. Auto and manual check overlap: store `run('checkUpdate')` returns the same promise.
3. Component unmounts during check: generation guards prevent stale listener mutation; native latest-operation protection remains active.
4. Candidate disappears after another check/skip: root dialog closes or becomes non-actionable; handlers guard candidate-dependent actions.
5. Forced candidate: close icon, modal click, Escape, Later and Skip are unavailable.
6. Download/install failure: dialog remains present when candidate remains, pending clears, and retry is possible.
7. Web-only preview without Tauri: no unhandled rejection from automatic startup check.

## Acceptance Criteria

| ID | Criterion | Evidence |
| --- | --- | --- |
| AC-01 | Release runtime performs exactly one automatic check after bootstrap/listener registration and remains active when it fails. | Focused runtime tests. |
| AC-02 | Main app opens the existing update dialog for actionable candidate states and wires download/install/skip to the release store. | App contract/component test plus type check. |
| AC-03 | Forced updates cannot be dismissed by footer, close button, backdrop or Escape. | Dialog contract test. |
| AC-04 | Settings About exposes a loading manual check control and reports current/available/failure outcomes. | Settings contract test and scoped manual/browser inspection. |
| AC-05 | Auto/manual paths share one Pinia update state and preserve duplicate-request protection. | Store/runtime tests and code review. |
| AC-06 | Existing Tauri signed release configuration and update command contracts remain intact. | Release Node/Rust contract tests. |
| AC-07 | Scoped Vitest tests, TypeScript check, lint and renderer build pass. | Fresh command output. |

## Risks And Non-Goals

- Risk: startup auto-check could reject the runtime start; bounded by explicit lifecycle catch plus test.
- Risk: a forced dialog may still have an implicit Element Plus dismiss path; all relevant dialog props must be tested statically or at component level.
- Risk: user tree already changes adjacent application-shell/settings files; edits must be surgical and diff-reviewed.
- Non-goals are those listed in the source snapshot, including silent install, new backend service, polling frequency UI, plugin updates and release workflow redesign.

## Verification Contract

- TDD for runtime auto-check, forced dialog dismissal, and settings entry contracts.
- Run targeted Vitest files, `pnpm type-check`, scoped ESLint check, renderer build, release Node tests and relevant Rust release/foundation tests.
- Verify changed-hunk mapping, reference closure, no stale imports/tests and no change to signed release security semantics.
