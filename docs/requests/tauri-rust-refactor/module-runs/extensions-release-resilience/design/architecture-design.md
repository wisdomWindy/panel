# Extensions Release And Resilience Architecture

## Boundaries And Ownership

- `domain/release.rs`: pure update rollout/blacklist/state rules and plugin capability/package validation.
- `application/release.rs`: sole writable owner for update, plugin and resilience snapshots; operation sequencing and crash isolation.
- `application/release_ports.rs`: injected updater, package verifier/runtime, repository, housekeeping and clock ports.
- `infrastructure/release.rs`: unavailable production updater/plugin adapters plus SQLite/filesystem repository/housekeeping.
- `ipc/release.rs`: thin command/DTO/event mapping and safe errors.
- frontend desktop release contract/adapters: unknown runtime data to readonly view models.
- `stores/release.ts`: one native snapshot owner; `useReleaseRuntime`: one listener lifecycle.
- update/plugin/settings pages: orchestration only. Existing application-shell store continues to own configuration import/export/reset.

## File Shape

```text
src-tauri/src/
  domain/release.rs
  application/{release.rs,release_ports.rs}
  infrastructure/release.rs
  ipc/release.rs
src/renderer/
  platform/desktop/contracts/release.ts
  modules/release/adapters.ts
  stores/release.ts
  composables/useReleaseRuntime.ts
  views/update-log/{index.vue,UpdateAvailableDialog.vue}
  views/plugin-manager/{index.vue,PluginInstallDialog.vue}
```

About/license content stays inline in settings unless verified resource data makes it a cohesive section. No generic manager, factory, registry or audit facade is added.

## Dependency Direction

```text
App/pages -> release runtime/store -> desktop transport -> IPC
IPC -> application release state -> domain rules
application state -> injected ports -> infrastructure
infrastructure -> SQLite/filesystem/updater/runtime adapters
```

| From | May depend on | Forbidden |
| --- | --- | --- |
| domain | std/serde value types | Tauri, SQLx, UI, filesystem/network |
| application | domain + port traits | concrete infrastructure, Vue, Tauri commands |
| infrastructure | domain/application ports | IPC/UI |
| IPC | application/domain + shared safe error | SQL rows, renderer |
| frontend adapter/store | desktop contract/transport | SQL/provider DTO, components |
| runtime | stores + typed listen | pages, normalization |
| pages/components | normalized stores/view props | invoke/listen/fetch/localStorage/path APIs |

Cycle risk between shell configuration and release state is avoided by keeping their stores/states separate; App composes their runtimes only.

## State Topology And Data Lifecycle

- Native release state owns immutable `UpdateSnapshot`, plugin map keyed by validated ID, resilience/about snapshot and operation generation.
- Derived frontend state owns filtered plugin tabs, action availability and candidate button visibility. These are computed, not persisted.
- Async ownership: native application state owns update/plugin operation IDs; frontend store owns pending command names; root runtime owns unlisteners.
- Data lifecycle: raw manifest/package/SQL row -> infrastructure validation -> domain model -> application snapshot -> safe IPC DTO -> TS adapter -> immutable Pinia snapshot -> precise component props -> typed intention -> native validation/effect.
- `null` is explicit absence; malformed enum/value is an adapter error. Templates do not normalize.

## Pure Functions And Side Effects

- Pure: semantic-version validation, rollout eligibility, forced/blacklisted policy, progress transition, permission/resource authorization, domain normalization, IPC adapters and tab filtering.
- Side effects: updater/package/runtime/repository/housekeeping ports, application operations, IPC event emission, Pinia actions and root listener lifecycle.
- Filesystem enumeration is bounded to the database directory and exact backup naming convention. It never follows arbitrary renderer paths.

## Reuse And Anti-DRY

| Candidate | Sites / semantics | Decision |
| --- | --- | --- |
| Ed25519/hash/path confinement | member module and plugin/update packages; same security mechanism, different descriptor policy | reuse low-level verifier/confinement primitives; keep descriptor rules separate |
| migration/export backups | database migration and config export; different lifecycle/rollback | keep separate; only migration owns three-backup pruning |
| update/plugin lifecycle | both stateful native operations but different permissions/terminal states | keep separate enums under one state owner; no generic state machine |
| safe IPC errors/adapters | all desktop features, identical technical semantics | reuse existing DesktopError/transport pattern |
| update/install dialogs | one caller each, distinct business lifecycle | extract approved single-caller components; do not share |
| permission labels | table and install review, same feature semantics | one narrow typed mapping in owning plugin page/module |

The only two-site security reuse is justified because duplicate signature/path code is security-sensitive and the existing member artifact mechanism is stable. No broad shared utils file is created.

## Pattern Decisions

- Level 2 reuse of existing layered Adapter/Command/store/runtime architecture.
- Lightweight State transitions for update/plugin statuses and Strategy ports for unavailable versus future production providers.
- Capability Policy is a pure exhaustive match; no role engine.
- Reject browser `Function` sandbox, dynamic JavaScript loader, global event bus, provider factory, plugin manager class and DI container. The real change axes are provider/runtime implementation and package policy, both covered by ports.

## 2026-09-16 Distribution Architecture Amendment

### Architecture Objective And Scope

Replace only the external-distribution placeholders: Tauri application update provider, GitHub Actions release/config generation, and platform artifact catalog generation/validation. Existing Vue store/pages, IPC command names, rollout policy and local/plugin security boundaries remain unchanged.

### Expert Architecture Decisions

| ADR | Problem | Decision and owner | Rejected alternative | Required evidence |
| --- | --- | --- | --- | --- |
| ER-A01 | Free/member need independent stable manifests although GitHub has one repository-wide latest release. | Versioned channel releases hold bundles; Actions publishes `free/latest.json` and `member/latest.json` to an `updates` branch. Channel config compiles one raw GitHub endpoint. | One shared `releases/latest` endpoint can cross channels; fixed mutable release tags lose history. | Workflow/schema tests and endpoint scan. |
| ER-A02 | Release key must not enter source or local artifacts. | `scripts/prepare-release-config.mjs` creates an ignored temporary channel config from the public key variable; Tauri consumes private key/password secrets directly. | Committed placeholder/private key; runtime-downloaded public key. | secret-name scan, missing-input failure test, generated config test. |
| ER-A03 | Existing UI separates check, download and install. | `TauriUpdateProvider` owns `PendingUpdate { update, verified_bytes }` behind an async mutex. `check` replaces both, `download` calls Tauri verification and retains bytes, `install` consumes only that pair. | JavaScript updater plugin; download-and-install in one step; storing updater bytes in application/IPC. | Rust provider contract tests plus compile/build. |
| ER-A04 | Resource sources and target coverage vary independently. | A pure release-script resolver converts allowlisted upstream GitHub release metadata into a project artifact catalog keyed by `(artifactId, os, arch)`, requiring SHA-256 and HTTPS. Runtime installation remains behind the existing module/plugin/media provider ports. | Hard-coded mutable URLs in Vue/Rust; generic mirror fallback; one archive for all platforms. | Node fixture tests, catalog validator tests, no-unapproved-host scan. |
| ER-A05 | App update signatures, WASM signatures and third-party hashes protect different trust claims. | Keep Tauri Minisign, project WASM package signature and upstream SHA-256 validation separate; share only target/path/hash-format pure rules when identical. | One universal verifier or treating a GitHub TLS download as signature proof. | Negative verification tests and explicit error mapping. |

### Ownership Map And File Shape

```text
.github/workflows/release-tauri.yml
packages/bilibili-live-tool/
  scripts/
    prepare-release-config.mjs
    resolve-runtime-artifacts.mjs
    *.test.mjs
  src-tauri/
    tauri.*.conf.json                 # local channel config, no release key
    src/infrastructure/release.rs     # TauriUpdateProvider + unavailable plugin provider
    src/application/release*.rs       # existing policy/state ports, no Tauri types
```

- workflow owns trigger, matrix, permissions, release upload and fixed-manifest publication.
- config script owns release-only JSON merge and validation; generated config is temporary and ignored.
- artifact resolver owns upstream host/repository allowlist, OS/arch asset-name selection, digest validation and deterministic catalog serialization.
- `TauriUpdateProvider` is the sole writable owner of the Tauri `Update` handle and verified bytes. `ReleaseState` remains the sole writable owner of user-visible `UpdateSnapshot`.
- existing member/media/plugin provider ports remain installation owners. The catalog does not bypass entitlement, license acceptance, WASM permission review or FFmpeg source policy.

### Dependency Rule Matrix

| Owner | Allowed dependencies | Forbidden dependencies |
| --- | --- | --- |
| workflow | repository scripts, official GitHub actions, GitHub Releases/API | application secrets in files, renderer code |
| release config script | channel JSON + environment public key/endpoint | private-key persistence, network |
| artifact resolver | GitHub API DTO, pure target/asset rules, Node crypto/fs | Vue, Tauri IPC, arbitrary mirrors |
| infrastructure update provider | `tauri_plugin_updater`, domain/application contracts, app handle | Vue DTO, direct policy decisions, private signing key |
| application release state | provider traits + pure domain policy | Tauri `Update`, URLs, installer bytes |
| renderer | existing safe `UpdateSnapshot` contract | updater bindings, fetch, URLs, paths, signatures |

### State Topology And Data Lifecycle

- check: compiled endpoint/public key -> Tauri updater -> signature-bearing `Update` -> provider pending handle -> normalized candidate -> existing rollout policy -> `UpdateSnapshot`.
- download: pending handle -> Tauri HTTPS download -> Tauri signature verification -> provider-owned bytes -> `downloaded` snapshot. No bytes/path crosses IPC.
- install: application status gate + matching provider handle/bytes -> Tauri install -> current snapshot/process exit behavior. Any new check/skip invalidates retained bytes.
- release metadata: tag event -> tests -> channel/platform build -> signed bundles and `.sig` -> versioned channel release -> generated `latest.json` -> validated channel path in `updates` branch.
- runtime catalog: allowlisted upstream release API -> exact asset match/digest -> deterministic project catalog -> versioned release asset. Catalog is immutable input; installed-resource state remains in its owning native module.

### Reuse And Anti-DRY Matrix

| Candidate | Sites/semantics | Decision |
| --- | --- | --- |
| target OS/arch normalization | updater target, FFmpeg, Whisper, optional native WASM companion | one pure resolver-script mapping; stable technical commonality with 3+ consumers |
| hash formatting | member artifact verifier and release catalog | keep separate for now: runtime Rust bytes vs publication-time Node metadata, two layers and two callers |
| update/plugin signature | Tauri Minisign vs project WASM signature | keep separate: different key formats, payloads and lifecycle |
| update state machine | existing application state and new Tauri provider | reuse existing Strategy/State boundary; provider does not duplicate rollout policy |
| release JSON merge | free/member config | one script parameterized by channel; same config semantics and two stable callers |

### Extension Seams, Locality And Complexity Budget

- real seams: `UpdateProvider` implementation and release-script target table. No provider registry/factory is added.
- scripts are separate files because they are CI entrypoints with independent tests, not page-private helpers.
- maximum new production files: one workflow plus two scripts/tests; Rust provider stays in existing `infrastructure/release.rs` unless file length exceeds reviewability after implementation.
- maximum new public Rust API: `TauriUpdateProvider::new`; no new IPC command or frontend contract.
- maximum indirection: UI/store -> IPC -> application state -> provider -> Tauri plugin, matching existing architecture.
- no new Vue subscriptions, stores, components or reactive fan-out.

### Migration, Cleanup And Rollback

- local channel configs continue to build without secrets using `UnavailableUpdateProvider` when updater configuration is absent; release config selects `TauriUpdateProvider`.
- generated `tauri.release.*.conf.json` and downloaded resource staging are ignored and deleted by workflow cleanup.
- rollback is a previous version tag plus republished channel manifest; historical versioned releases remain intact.
- remove `UnavailableUpdateProvider` only after local no-config behavior has an equivalent explicit provider; retain `UnavailablePluginProvider` until a real signed WASM asset exists.
- workflow failure before manifest publication leaves the previous channel manifest active.

### Risks And Open Architecture Questions

- Tauri action matrix merging of `latest.json` must be verified in Actions; publication job must reject a manifest missing any built target.
- macOS FFmpeg LGPL-shared official-linked binary coverage and Whisper executable shape are not uniform. Resolver publishes only allowlisted discovered assets and must not synthesize coverage.
- OS code signing/notarization remains separate deployment ownership; updater signatures do not replace it.

## Migration And Cleanup

1. Add native contracts/rules and red tests.
2. Implement state/IPC/unavailable adapters and register commands in both channels.
3. Migrate frontend stores/pages/runtime.
4. Remove renderer `plugins/**`, update service, Electron `src/main/**`, Electron scripts/config/dependencies and stale declarations/tests.
5. Prune migration backups to newest three and test rollback preservation.
6. Verify no `eval`/`new Function`/Electron/browser update-plugin caller remains.

Rollback is scoped to release files and package metadata. SQLite schema migrations remain additive; a startup failure degrades release/plugin status without preventing live-data/media/shell startup.

## Complexity Budget

- At most 5 new Rust production files and 8 frontend production files, matching the approved file shape above.
- One new frontend store/runtime and one IPC command group; no extra public barrel beyond contract exports.
- Maximum effect path: page -> store -> transport -> IPC -> state -> port.
- One event listener group, no per-page listener/timer, no new cache/debounce/virtualization.
- Existing config UI and repository are reused, not duplicated.

## ADRs

- ADR-ER-01: native release state replaces browser update/plugin execution; this is the minimum structure that enforces IPC/path/security ownership.
- ADR-ER-02: external adapters remain unavailable until signed contracts/artifacts exist; this prevents fabricated success.
- ADR-ER-03: plugin capability authorization is exhaustive deny-by-default and independent per runtime ID.
- ADR-ER-04: migration backup pruning belongs to database bootstrap; config exports are user artifacts and are never retention-pruned.
- ADR-ER-05: Electron dependency closure is deleted only after Tauri-native UI callers are live.

## Risks And Exit

- A production WASM engine/updater cannot be selected without ABI/manifest/key inputs; ports preserve the seam and UI remains truthful.
- Bundle warning and dependency cleanup require full free/member builds.
- The architecture preserves approved behavior and is execute-plannable. Advance to `plan`.
