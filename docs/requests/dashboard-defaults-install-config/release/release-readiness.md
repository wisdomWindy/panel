# Release Readiness

- Rollout: install the generated NSIS package over the existing installation.
- Migration order: close old process, install binary, launch new binary, copy legacy database only when install target is absent, then initialize SQLite.
- Observation: verify process path, installed `app.db`, `config_overview.displayPath`, `ui_get.theme`, and dashboard computed colors.
- Rollback trigger: startup failure or database-open failure after upgrade.
- Rollback path: reinstall the previous binary; the legacy AppData database remains untouched.
- Residual risk: installation-directory ACLs must permit the current user to create and update SQLite files.

Result: pass

