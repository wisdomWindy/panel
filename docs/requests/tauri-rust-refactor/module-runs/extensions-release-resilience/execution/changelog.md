# Extensions Release And Resilience Execution Changelog

## T01 Contract And Red Tests

- Added closed Rust/TypeScript update, plugin, about and resilience contracts plus exact free/member command registry tests.
- Added deterministic rollout/pause/blacklist/force, plugin permission/resource, domain allowlist, WASM filename and malformed adapter tests.

## T02 Rust State, IPC, And Resilience

- Added native release application state, injected updater/plugin/event ports, typed unavailable adapters and redacted IPC errors.
- Enforced signed-provider fail-closed behavior, install-after-downloaded gating, semantic-version/identity validation, stable installation rollout buckets and latest-generation-wins commits.
- Added real `update://status`, `plugin://status` and `plugin://crashed` event emission after committed state changes.
- Exposed path-free migration backup count/timestamp state; existing database bootstrap keeps the newest three exact migration backups and preserves rollback tests.

## T03 Frontend Runtime And Pages

- Replaced browser-owned update/plugin state with validated adapters, one Pinia snapshot owner and one root listener lifecycle.
- Rebuilt update, plugin and About/license views with explicit unavailable, pending, empty, forced-update, permission-review and confirmation states.
- Added pessimistic toggle/uninstall and listener rollback/disposal tests.

## T04 Legacy Cleanup And Compliance

- Removed Electron main/preload source, Electron scripts/dependencies, browser update service, browser plugin loader/sandbox/runtime and obsolete callers.
- Removed browser speech/database/media owners after their Tauri replacements were active and regenerated the lockfile.
- Production scans contain no Electron runtime, `new Function`, `eval`, `window.open`, direct renderer update/plugin `fetch`, or member surface in the free bundle.

## T05 Verification And Review

- Repaired during review: missing provider/event ports, dead native event subscriptions, forced-update normalization, exposure policy application, stale operation overwrite protection, exact migration backup reporting and a frontend complexity-budget arithmetic error.
- Final verification: free/member Vitest 37 files and 99 tests each; TypeScript pass; ESLint 0 errors and 2 pre-existing logger warnings; Rust free 76/member 93 tests; fmt and strict Clippy pass.
- Final free/member Vite and Tauri builds produced MSI and NSIS installers; all four are below 60 MiB.
- Live UI inspected update, plugin and About/license degraded states at 1280x720. External provider success paths remain intentionally unavailable.

## 2026-09-16 External Distribution Re-entry

### T06 Release Script Contracts

- Added pure, Node-tested release configuration generation. It accepts only `free` or `member`, requires an updater public key, requires HTTPS, enables Tauri updater artifacts and writes ignored build-time overlays.
- Added manifest finalization that verifies release version, platform presence, HTTPS artifact URLs and non-empty Tauri signatures before adding the existing rollout policy fields.
- Added seven Node contract tests. The initial red run failed on missing modules and the implementation run passed all tests.

### T07 Native Tauri Updater Provider

- Added `tauri-plugin-updater` and registered it before application state initialization.
- Replaced `UnavailableUpdateProvider` with `TauriUpdateProvider`, retaining the checked `Update`, Tauri-verified downloaded bytes and normalized candidate metadata inside the native provider.
- Installation consumes only retained verified bytes. Skip clears only the matching candidate; stale downloads cannot replace a newer check; network/release-not-found checks normalize to the silent current state while configuration/manifest errors fail closed.
- Added strict metadata/default/offline tests and retained the unavailable plugin provider because no signed WASM package, ABI or plugin verification key exists.

### T08 Signed Dual-channel GitHub Release

- Added `.github/workflows/release-tauri.yml` with minimal job permissions, signed free/member draft releases, current GitHub-hosted Windows/Linux x64/Linux ARM64/macOS ARM64/macOS x64 targets and Tauri Action updater artifacts.
- Private signing key/password are read only from GitHub Actions secrets. The public key is read from a repository variable and injected into ignored release configs.
- The publish job validates and writes channel-specific `latest.json` files to the `updates` branch, then publishes both GitHub Releases. Optional WASM assets require an adjacent `.sig` and are never fabricated.

### T09 Official Runtime Artifact Catalog

- Added an allowlisted GitHub API resolver for FFmpeg.org-linked BtbN LGPL shared builds and `ggml-org/whisper.cpp` official binaries.
- The catalog maps exact OS/architecture asset names, requires HTTPS repository ownership and GitHub SHA-256 digests, downloads and re-hashes every asset in Actions, and records source/license metadata.
- Live upstream smoke resolution found eight supported Windows/Linux x64/ARM64 assets. macOS remains absent because whisper.cpp does not publish a matching executable asset.

### Execute Self-healing

- Fixed one `code-organization-error`: Node test filenames overlapped Vitest's discovery glob. Renamed them to `*.node-test.mjs`; Node tests and both 99-test Vitest suites then passed.
- Fixed one `syntax-type-error`: `Option::is_none_or` exceeded the declared Rust 1.77.2 MSRV. Replaced it with the compatible `map_or` form; strict free/member Clippy then passed.
- Each blocker had one execute attempt and one successful recheck; no blocker remains.

### Review Self-healing

- Fixed a `logic-error` in the native updater adapter: version-only stale checks could attach bytes from an older same-version manifest to a newer `Update` handle. Check/download/install now share a monotonic generation, out-of-order checks cannot overwrite newer state, and install holds the pending-update owner through consumption.
- Fixed a release-contract mismatch in the runtime catalog: entries now expose `artifactId`, deterministic `generatedAt`, `selectedEntry`, positive size and unique target validation. Duplicate upstream assets fail closed.
- Restricted finalized updater artifact URLs to this project's GitHub Release path instead of accepting arbitrary HTTPS hosts, and added a regression test.
- Made the upstream resolver consume either Actions token name (`GH_TOKEN` or `GITHUB_TOKEN`) and pinned Node 24 in the publish job.
- Re-ran all release, Rust, frontend, formatting, lint, type and build gates. The active re-entry used two self-healing iterations; no blocker remains.

### Repository Signing Setup

- Authenticated GitHub CLI as the repository administrator and configured Actions secrets `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` plus repository variable `TAURI_SIGNING_PUBLIC_KEY` on `wisdomWindy/panel`.
- Generated the key pair with the project-pinned Tauri signer. Private material was streamed to GitHub, the public minisign payload was structure-validated, and the temporary key directory was removed without writing secrets into the workspace.
