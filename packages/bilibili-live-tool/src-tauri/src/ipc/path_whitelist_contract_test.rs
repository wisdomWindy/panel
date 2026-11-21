//! PRD 11.3: the renderer must never hand the backend a raw filesystem path.
//!
//! Paths are issued by the native dialog as opaque grants and every command
//! resolves them through `PathGrantPort` with an explicit purpose. These tests
//! keep that invariant from eroding as commands are added.

const IPC_SOURCES: &[(&str, &str)] = &[
    ("shell.rs", include_str!("shell.rs")),
    ("media.rs", include_str!("media.rs")),
    ("live_data.rs", include_str!("live_data.rs")),
    ("release.rs", include_str!("release.rs")),
    ("commands.rs", include_str!("commands.rs")),
    ("dto.rs", include_str!("dto.rs")),
];

#[test]
fn no_command_accepts_a_raw_filesystem_path() {
    for (name, source) in IPC_SOURCES {
        // `commands.rs` owns the native dialog helper; it is the only place
        // allowed to hold a PathBuf, and it never receives one from the UI.
        if *name == "commands.rs" {
            continue;
        }
        assert!(
            !source.contains("PathBuf"),
            "{name} must not exchange raw paths over IPC"
        );
    }
}

#[test]
fn command_modules_do_not_touch_the_filesystem_directly() {
    for (name, source) in IPC_SOURCES {
        assert!(
            !source.contains("std::fs::") && !source.contains("tokio::fs::"),
            "{name} must delegate file access to the application layer"
        );
    }
}

#[test]
fn every_path_crosses_ipc_as_a_grant_identifier() {
    // The two modules that move files all resolve grants with an explicit
    // purpose before touching the disk.
    let shell = include_str!("shell.rs");
    let media = include_str!("media.rs");
    assert!(shell.contains("PathPurpose::ConfigExport"));
    assert!(shell.contains("PathPurpose::ConfigImport"));
    assert!(media.contains("PathPurpose::GiftSoundExport"));
    assert!(media.contains("PathPurpose::GiftSoundFile"));
    assert!(media.contains("PathPurpose::HighlightExport"));
}
