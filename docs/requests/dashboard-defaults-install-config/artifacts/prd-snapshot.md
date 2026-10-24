# PRD Snapshot

1. All five dashboard summary values use an explicit readable dark text color on their white cards.
2. New and reset UI preferences default to `system` in both renderer fallback and Rust domain state.
3. Release startup selects `<executable-directory>/app.db`.
4. When that target does not exist, copy an existing legacy AppData database and its `-wal`/`-shm` sidecars; never overwrite an existing target.
5. Debug builds retain the legacy AppData location.
