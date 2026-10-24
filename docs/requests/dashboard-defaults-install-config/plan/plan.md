# Plan

1. Add failing dashboard, system-default, startup-source, and database-location contract tests.
2. Add `infrastructure/database_location.rs` with deterministic selection/migration tests; wire it from `lib.rs`.
3. Add explicit `text-gray-900` to five dashboard values and switch renderer/Rust defaults to system.
4. Run focused and full frontend/Rust checks, build NSIS, install, save current theme as system, and verify UI/config path in the installed app.

Code review contract: preserve saved choices, never overwrite target DB, copy sidecars, propagate migration errors, keep debug data out of build directories, add no new API, and remove no existing fallback without tests.
