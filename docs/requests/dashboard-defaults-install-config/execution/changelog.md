# Execution Changelog

- Added a dashboard contract test and explicit `text-gray-900` classes to all five metric values.
- Changed renderer and Rust UI defaults from `dark` to `system` without modifying the saved-preference load path.
- Added `infrastructure/database_location.rs` and routed startup database selection through it.
- Release builds use `current_exe().parent()/app.db`; debug builds retain `app_data_dir()/app.db`.
- First release startup copies a missing legacy database plus present `-wal` and `-shm` files. Existing installed databases are authoritative and legacy sources remain intact.
- Built and installed the NSIS package, launched the installed executable, and saved the current UI theme as `system`.

Product-content delta: none beyond the three user-requested behaviors.

