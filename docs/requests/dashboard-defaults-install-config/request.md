# Request

Fix three installed-app issues reported with a screenshot: unreadable dashboard metric values, a non-system default theme, and a configuration directory outside the installation directory.

Done means the five dashboard values are readable, new/reset UI preferences default to `system`, and release builds use the executable directory for `app.db` while preserving an existing legacy database on first migration.
