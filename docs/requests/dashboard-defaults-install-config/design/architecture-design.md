# Architecture Design

## Ownership

- Renderer markup owns contrast.
- `UiPreferences::default` owns durable new/reset defaults; renderer fallback mirrors it.
- A new `infrastructure::database_location` module owns release/debug path selection and one-time legacy file copying.
- `lib.rs` supplies Tauri AppData and invokes the resolver before constructing `AppState`.

## Data And Failure Flow

`app_data_dir -> legacy app.db` plus `current_exe -> install app.db` -> resolve/migrate -> `AppState::new` -> database parent -> config overview/open directory.

Migration is copy-only, target-absent only, and includes SQLite sidecars. An existing install database is authoritative. I/O errors propagate through Tauri setup; no false success or misleading displayed path.

## Rollback

The legacy AppData files are retained, so reverting the binary restores the old location. Removing the install-directory database causes the next new binary launch to copy from legacy again.
