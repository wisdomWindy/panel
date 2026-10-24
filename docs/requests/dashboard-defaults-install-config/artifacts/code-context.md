# Code Context

- UI: `views/dashboard/index.vue` owns the metric markup.
- UI defaults: `InterfaceSettingsPanel.vue` provides the local draft fallback.
- Domain defaults: `domain/shell.rs::UiPreferences::default` owns persisted reset/new defaults.
- Startup: `lib.rs::run` currently chooses `app_data_dir()/app.db`.
- Display/open directory: `ApplicationShellState` receives the database parent, so changing the real database path updates both behaviors without UI falsification.
- New filesystem boundary is justified as a separate infrastructure module because executable-path selection, legacy file migration, and filesystem tests are OS persistence concerns.
