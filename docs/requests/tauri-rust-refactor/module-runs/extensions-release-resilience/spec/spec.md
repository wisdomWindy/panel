# Extensions Release And Resilience Engineering Specification

## Delivery Contract

- request/module: `tauri-rust-refactor/extensions-release-resilience`
- speed: `S3 broad`; architecture-design required
- goal: finish native configuration/release/plugin/compliance boundaries, remove Electron and browser-executed plugin/update production paths, and prove both channels remain buildable.
- sources: requirement module; PRD 9.17-9.19 and 10; structured PRD snapshot 11/13-15; approved page design.

## Scope And Grounding

| ID | Contract | Grounding |
| --- | --- | --- |
| ER-S01 | Existing SQLite/Tauri configuration overview, dialog-owned export/import, transactional validation and confirmed reset remain authoritative. | source-backed, PRD 9.17 |
| ER-S02 | Migration creates a backup before change, rolls back on failure, and retains at most three migration backups. | source-backed, snapshot data workflow |
| ER-S03 | Update channel is compile-time free/member; policy evaluates rollout bucket, pause, blacklist and force flags before exposing an update. | source-backed, PRD 9.18 |
| ER-S04 | Download/install is unavailable without a signed manifest/updater provider; signature failure never reaches install; offline check is silent. | source-backed + external-interface-pending |
| ER-S05 | Update UI shows current/latest version, release notes, native progress and exact immediate/later/skip/restart actions; forced update omits Later. | source-backed, PRD 9.18 |
| ER-S06 | Plugins are signed WASM packages only. JavaScript, `Function`, eval, browser fetch/file loading and arbitrary native paths are forbidden. | source-backed, PRD 9.19/10 |
| ER-S07 | Each enabled plugin has an independent runtime identity. Host capability policy allows sanitized event/current-room UID and declared-network access only; config, DB, key, login and direct file access are denied. | source-backed, PRD 9.19 |
| ER-S08 | Plugin install/list/toggle/uninstall/config actions are native-owned, pessimistic and audited; absent signature/package/runtime contracts fail closed. | source-backed + external-interface-pending |
| ER-S09 | A plugin trap/crash transitions only that plugin to failed/disabled and leaves the host/runtime collection alive. | source-backed, PRD 9.19 |
| ER-S10 | About/licenses displays only actual bundled application/channel data and present license resources; absent FFmpeg/Whisper artifacts are unavailable. | source-backed + external-artifact-pending |
| ER-S11 | Logs and uploaded diagnostics exclude secrets; no default crash upload is added. Aggregate statistics remain permanent; unspecified raw retention durations do not authorize deletion. | source-backed/missing-source, snapshot 13-15 |
| ER-S12 | All Electron production source, dependencies, scripts and browser plugin/update paths are removed after native callers replace them. | code-fact-backed dependency closure |

## Explicit Non-Scope

- No fabricated update endpoint, release, signature key, plugin catalog/package, license body, FFmpeg/Whisper binary, crash upload or success result.
- No arbitrary plugin JavaScript, filesystem/database/key/login/config access, undeclared network, hidden member code in free output, or danmaku send path.
- No raw-log deletion period because the surviving source does not specify one.

## Native Domain And IPC Contracts

### Update

- `ReleaseChannel = free | member` comes only from the build feature.
- `UpdateStatus = unavailable | current | available | downloading | downloaded | failed`.
- `UpdateCandidate`: version, readonly release-note strings, rollout percentage `0..=100`, paused, forced, and blacklisted-version list. Invalid percentages/versions fail validation.
- `UpdateSnapshot`: currentVersion, channel, status, candidate nullable, progressPercent nullable, errorCode nullable.
- Commands: `update_status`, `update_check`, `update_download`, `update_install`, `update_skip`.
- Events: `update://status`; sequence/state replacement is native-owned.
- A stable local installation identity is hashed into bucket `0..99`; eligible iff not paused, current version not blacklisted, and bucket < rollout. A blacklisted current version forces an otherwise valid candidate.

### Plugins

- `PluginPermission` is an exhaustive enum: sanitizedEvent, currentRoomUid, effect, panel, dataSource, network.
- Explicitly forbidden capabilities are represented by `PluginResource`: config, database, key, login, filesystem and undeclaredNetwork; authorization always returns denied and an audit code.
- `PluginManifest`: id, name, version, declared permissions, declared network domains, signature metadata; unknown permissions or invalid domains fail validation.
- `PluginStatus = disabled | enabled | updateAvailable | failed | unavailable`.
- `PluginSnapshot`: normalized manifest fields/status/error code only; no installation path, secret or raw host object.
- Commands: `plugin_list`, `plugin_select_install`, `plugin_install`, `plugin_toggle`, `plugin_uninstall`, `plugin_config_get`, `plugin_config_save`.
- Events: `plugin://status`, `plugin://crashed`. Operation IDs prevent stale mutation results.
- File selection occurs natively; renderer never supplies a local path. Without signature key/runtime provider, selection/install returns a typed unavailable error.

### Resilience/About

- `resilience_status` exposes last migration backup timestamp/count, retention policy availability and third-party artifact availability without paths or secrets.
- `about_info` exposes application version, build channel, and verified license descriptors only.
- Startup invokes migration-backup pruning and non-destructive retention housekeeping. A housekeeping failure degrades status and must not prevent free capabilities from starting.

## Frontend Contracts And State

- Rust IPC DTOs are authoritative; TypeScript readonly contracts preserve camelCase names and validate every unknown payload.
- A single `useReleaseStore` owns update/plugin/about snapshots and per-operation pending/error state; pages never invoke/listen directly.
- One root runtime owns update/plugin event listeners and disposal. Duplicate start is idempotent; partial listener setup rolls back.
- `null` means an explicitly absent candidate/progress/error/license artifact. `undefined` is allowed only before initial bootstrap.
- Update/plugin lists replace immutable snapshots after validation. No optimistic switch, row removal or installation success.
- Existing `ConfigManagementPanel` remains owned by the application-shell store; it is not merged into release state.

## UI Behavior

- Update page follows the approved operational layout. Release notes are provider data rendered as plain text. No `window.open`; actions call native commands.
- Optional candidate exposes Immediate update, Later and Skip. Forced candidate exposes Immediate update only; install appears only after native status `downloaded`.
- Plugin tabs filter installed/updateable/disabled states without changing source data. Columns are name/version/status/permissions/actions.
- Install accepts only a native-selected signed WASM package and reviews native-verified permissions before confirmation; absent provider shows unavailable.
- Configure appears only when native config schema exists. Toggle and uninstall remain pending until native response; uninstall requires source-backed confirmation.
- Persistent exact safety meaning: plugins run in independent WASM sandboxes and cannot read sensitive configuration.
- About/license section never synthesizes legal text or a source URL.

## Security, Privacy, And Abuse Boundaries

- All paths are native grants; all command enums, IDs, booleans and versions are validated in Rust.
- Plugin payloads are sanitized allowlisted DTOs. No secret/Cookie/API key/config/database handle/path crosses IPC or plugin host calls.
- Signature verification occurs before archive inspection/promotion; traversal and non-WASM executable entries are rejected.
- Update/plugin errors are stable codes with redacted messages. Logs must not serialize keys, tokens, Cookies, manifests containing secrets, or local absolute paths.
- No renderer network call implements update/plugin behavior. Offline update check maps to silent current/unavailable state.

## Async And Failure Semantics

- Check/install/toggle/uninstall actions dedupe by operation. Late events with an older operation ID are ignored.
- Download progress is monotonic `0..100`; install requires verified downloaded state.
- Plugin crash cleanup is idempotent and affects one runtime ID only.
- Migration/housekeeping uses bounded file enumeration, retains newest three valid backups and never follows links outside the database directory.
- Missing database/updater/key/runtime/license resources produce typed unavailable status rather than startup failure.

## Pattern, Reuse, And Locality Signals

- Reuse Level 2 project boundaries: Rust domain/application/infrastructure/IPC, typed desktop gateway, Pinia store and root runtime.
- Adapt State for update/plugin lifecycle, Strategy ports for external updater/signature/WASM runtime, Adapter for IPC DTOs, and one small capability Policy function.
- Keep config state separate from release/plugin state because transactions/scopes differ.
- Share the existing artifact signature/hash/confinement mechanism only if architecture confirms the package descriptor semantics match; do not copy crypto/path rules.
- Reject browser sandbox, plugin manager/factory, global event bus, service locator, generic audit store and speculative provider registry.
- Page-local filtering/formatting stays local. Extract only the approved update/install dialogs with lifecycle/test ownership.

## Quality And Review Constraints

- Type-first, exhaustive enums, strict null, no `any`, no raw DTO/template normalization, fail-fast invalid states.
- Pure immutable policy/validation/adapter functions; I/O only in infrastructure, application actions, IPC, store actions and runtime lifecycle.
- Tailwind utility classes only; no scoped CSS/inline styles/class indirection.
- Preserve existing tsconfig strict/path alias/ambient Element Plus declarations and generated component declarations.
- Every changed hunk maps to a plan task/source/cleanup/test. Remove stale comments, mocks, dependencies and Electron scripts without unrelated formatting.

## Acceptance Criteria

1. Corrupt/incompatible config import preserves prior data; reset remains confirmed; migration rollback and three-backup retention are tested.
2. Update rollout, pause, blacklist, force, channel and invalid-manifest policy have deterministic Rust tests.
3. No unsigned/invalid update can install; offline checking is silent; missing provider is visibly unavailable only on user-opened update UI.
4. Plugin forbidden capabilities are denied/audited; unknown permissions, unsafe domains and non-WASM packages are rejected.
5. Plugin crash isolation test proves a second runtime remains active.
6. UI/store tests prove pessimistic mutation, forced action visibility, filter states, permission review and no browser network/path execution.
7. Electron production source/dependencies/scripts and legacy browser plugin/update chain have zero references.
8. Free/member Rust, Vitest, typecheck, lint, Vite and Tauri builds pass; free output remains member-clean and both installers remain under 60 MiB.
9. Verification records security, privacy, migration, resilience, cleanup and unavailable external-resource limits.

## Architecture Route

Architecture design must fix state ownership, command registry, package-verifier/runtime ports, reuse of artifact security, database backup/retention ownership, listener lifecycle, migration cleanup, file layout and complexity budget without altering these behaviors.

## 2026-09-16 External Distribution Amendment

### User Intent Contract

- literal request: external application updates and signing keys use GitHub Actions with Tauri Updater; WASM packages and FFmpeg/Whisper resources download official binaries for the current platform.
- practical goal: replace the unavailable update/resource placeholders with an auditable release pipeline and fail-closed native download boundaries without committing private keys or inventing third-party mirrors.
- success: free/member builds use separate signed manifests, application updates are verified by Tauri before install, and resource selection cannot cross OS/architecture or fall back to an unapproved source.
- forbidden interpretations: no private key in Git, no unsigned update/resource install, no renderer-owned download, no mutable `latest` URL without an integrity/signature boundary, and no unofficial Whisper/FFmpeg fallback when an official platform asset is absent.

### Source-Grounded Contracts

| ID | Contract | Grounding |
| --- | --- | --- |
| ER-S13 | GitHub Actions is the release authority. Version tags trigger free/member Tauri builds and versioned GitHub Releases; each channel publishes a distinct Tauri static manifest under the repository `updates` branch. | confirmed-decision, user 2026-09-16 |
| ER-S14 | `TAURI_SIGNING_PRIVATE_KEY` and its password exist only as GitHub Actions secrets. The updater public key is injected into release config at build time and may be exposed; local builds without release configuration remain fail-closed. | confirmed-decision + Tauri Updater contract |
| ER-S15 | The Rust `UpdateProvider` adapts Tauri Updater. Check retains the verified candidate handle, download verifies the Tauri signature before exposing `downloaded`, and install consumes only the retained verified bytes. Network/offline errors normalize to the existing silent unavailable/current behavior. | confirmed-decision + code-fact-backed existing provider/state |
| ER-S16 | Free/member endpoints are independent (`updates/free/latest.json`, `updates/member/latest.json`) and a release build compiles exactly one endpoint/channel. A manifest from the wrong channel is never selected. | source-backed PRD 9.18 + confirmed-decision |
| ER-S17 | Runtime artifact catalogs use explicit `artifactId`, version, target OS, target architecture, upstream URL, SHA-256, archive kind, selected entry and license/source metadata. Selection requires an exact target match and HTTPS. | confirmed-decision + source-derived integrity boundary |
| ER-S18 | FFmpeg artifacts may come only from executable-package sources linked by `ffmpeg.org`; selected packages must be LGPL shared builds where that upstream offers them. Whisper runtime binaries may come only from `ggml-org/whisper.cpp` releases. Missing official assets produce unavailable state, never an alternate mirror. | confirmed-decision + PRD 11 |
| ER-S19 | Project-owned signed WASM plugin/module packages are distributed as GitHub Release assets and selected from the signed project catalog. WASM is platform-neutral unless its manifest explicitly declares a target; any native companion artifact uses exact platform selection. | confirmed-decision + PRD 9.19 |
| ER-S20 | GitHub workflow and catalog generation pin action versions, minimize permissions, validate required secrets/variables, preserve SHA-256 metadata and publish only after channel builds/tests succeed. | source-derived supply-chain requirement |

### External Contracts And Ownership

- GitHub owns release storage and immutable version assets; the `updates` branch owns stable channel manifest URLs.
- Tauri Updater owns application bundle download, Minisign verification and installer execution. Application code only maps its typed result into `UpdateSnapshot` and existing UI progress.
- A repository script owns release-config injection and artifact-catalog validation. It must reject missing key material, non-HTTPS URLs, duplicate target tuples, invalid hashes and unsupported archive kinds before publication.
- Rust owns target detection, catalog validation, bounded download, SHA-256 verification, confined extraction and atomic promotion for runtime resources. Vue receives status/progress only.
- TypeScript remains governed by `tsconfig.eslint.json` -> `tsconfig.json`; no new browser updater/downloader declaration source is introduced.

### Failure And Edge Semantics

- Missing signing secret/public key stops the release workflow before packaging; it does not produce an unsigned release.
- A `404`/offline manifest check is silent at startup. A user-triggered check may retain the existing typed unavailable error without leaking URLs or local paths.
- Signature/hash mismatch deletes temporary bytes and leaves the active app/resource untouched.
- Download cancellation/failure never changes the installed resource. Promotion is same-filesystem atomic replacement after verification and confined extraction.
- Unsupported OS/architecture, missing official upstream asset, malformed catalog or unexpected archive entry returns a stable unavailable/invalid code.
- The updater candidate/bytes are single-owner state. A new check invalidates older downloaded bytes; skip clears only the matching candidate.

### Pattern And Reuse Decision

- pattern depth: Level 2, reuse the existing `UpdateProvider` Strategy, native application State, IPC Adapter and immutable Pinia snapshot flow.
- introduce no generic download manager. Application updates remain owned by Tauri Updater; runtime artifacts use a narrow catalog/download adapter because their verification and installation semantics differ.
- share pure target/hash/path-confinement primitives only where semantics are identical. Keep Tauri Minisign verification, WASM package signature policy and SHA-256 resource integrity as separate authorities.
- architecture-design remains required to fix the updater handle/bytes owner, channel manifest publication topology, artifact catalog owner and FFmpeg/Whisper/WASM installation boundaries.

### Amendment Acceptance

1. Workflow syntax/config tests prove two channel manifests, secret-only private key usage, updater artifact generation and versioned release publication.
2. Rust tests prove exact platform selection, HTTPS/hash validation, wrong-channel rejection, stale-candidate invalidation and no install before verified download.
3. A release build with injected public key creates `.sig` updater artifacts; a local build without release secrets still compiles and reports updater unavailable.
4. Source scans prove no private signing key, browser download, unofficial resource fallback or renderer path ownership.
5. Existing dual-channel TypeScript/Rust tests, lint, formatting, Clippy and builds remain green.
