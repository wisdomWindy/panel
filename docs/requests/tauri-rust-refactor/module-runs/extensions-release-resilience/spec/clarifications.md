# Extensions Release And Resilience Clarifications

## 2026-09-16 Confirmed Decisions

- Application update hosting: GitHub Releases for versioned bundles; channel-specific Tauri manifests published by GitHub Actions to the repository `updates` branch.
- Application signing: Tauri signing private key/password remain GitHub Actions secrets; the public key is injected only into release configuration.
- Runtime resources: exact-platform official binary assets only. FFmpeg source must be linked by `ffmpeg.org`; Whisper source must be `ggml-org/whisper.cpp`; unsupported official coverage remains unavailable.
- WASM distribution: project-owned signed WASM packages are GitHub Release assets and use the project catalog, not arbitrary user URLs.

## Remaining External Inputs

- GitHub repository administrators must create `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` secrets and `TAURI_SIGNING_PUBLIC_KEY` repository variable before the first release run.
- Windows/macOS operating-system code-signing certificates are separate from Tauri updater signing and remain deployment-owner inputs.
- No concrete WASM plugin binary currently exists in the repository; the pipeline/catalog contract can be delivered, but publishing a plugin asset requires its own source package and signing input.
- Official Whisper releases do not guarantee every OS/architecture has the same executable shape. Catalog generation must publish only discovered allowlisted assets and leave missing targets unavailable.

## External Interface Pending

- Update endpoints/manifests, Ed25519 public key, channel server contract and rollback installer mechanism are absent.
- Plugin package schema/signature key, native configuration schema and production WASM host ABI are absent.
- FFmpeg LGPL shared binary, exact license resource/source URL, Whisper binary/model manifests and license resources are absent.

These items authorize typed unavailable adapters only. They block claims of production update/plugin/third-party distribution success, but not local policy, UI, migration, isolation and cleanup work.

## Missing Source

- Raw event/AI/audit log retention durations are not present in the surviving source; do not delete them by an invented schedule.
- Crash-report destination/payload/consent is absent; do not upload crashes by default.
- Update/plugin success copy beyond PRD labels is absent; use native status labels and errors only.

## Technical Assumptions

- Reversible, non-product choice: keep the existing Rust layered structure and one shared desktop transport.
- Reversible, non-product choice: deterministic tests use injected clocks/ports and synthetic unsigned descriptors; production adapters stay unavailable.
- No human decision is required to execute the fail-closed local scope.
