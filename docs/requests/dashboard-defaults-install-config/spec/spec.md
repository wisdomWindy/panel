# Specification

## Behavior

- Every dashboard metric `<dd>` includes `text-gray-900` on the existing white surface.
- Renderer `defaultUi().theme` and Rust `UiPreferences::default().theme` equal `system`.
- Release startup resolves the database to the current executable parent plus `app.db`.
- If the release target database is absent and the legacy database exists, copy `app.db`, `app.db-wal`, and `app.db-shm` when present. Existing target files win and are never overwritten.
- Debug startup keeps `app_data_dir()/app.db`.
- Config overview/open-directory continue deriving from the actual database parent.

## Constraints

- Preserve saved explicit theme choices in product code.
- Preserve the legacy source database after copying.
- No IPC or renderer contract changes.
- Filesystem failure remains a startup error rather than silently presenting an incorrect directory.
- Pattern decision: Level 1 adapt lightweight pattern. Keep startup selection in `lib.rs`; isolate executable-path and migration filesystem behavior in infrastructure for deterministic tests.

## Acceptance

- Static UI contract finds five explicit dark metric values.
- Rust default test expects `ThemeMode::System`.
- Filesystem tests prove debug/installed selection, sidecar migration, and no-overwrite behavior.
- Installed application reports its executable directory as configuration location and the current UI setting is saved as system.
