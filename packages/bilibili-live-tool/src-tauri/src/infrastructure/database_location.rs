use std::{fs, io, path::{Path, PathBuf}};

const DATABASE_FILE_NAME: &str = "app.db";

pub fn resolve_database_path(app_data_directory: &Path) -> io::Result<PathBuf> {
    let legacy_database_path = app_data_directory.join(DATABASE_FILE_NAME);
    if cfg!(debug_assertions) {
        return Ok(legacy_database_path);
    }

    prepare_installed_database_path(&std::env::current_exe()?, &legacy_database_path)
}

/// Release builds keep the database next to the executable.
///
/// A database left in the old per-user location is deliberately **not** copied
/// over: earlier versions tried to migrate it and an incompatible schema then
/// broke start-up, so the installed app starts from a fresh database instead.
fn prepare_installed_database_path(
    executable_path: &Path,
    _legacy_database_path: &Path,
) -> io::Result<PathBuf> {
    let install_directory = executable_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "the application executable has no parent directory",
        )
    })?;
    fs::create_dir_all(install_directory)?;
    Ok(install_directory.join(DATABASE_FILE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_location_ignores_a_legacy_database_instead_of_migrating_it() {
        let directory = tempfile::tempdir().unwrap();
        let legacy_directory = directory.path().join("legacy");
        let install_directory = directory.path().join("installed");
        fs::create_dir_all(&legacy_directory).unwrap();
        fs::create_dir_all(&install_directory).unwrap();
        let legacy_database = legacy_directory.join(DATABASE_FILE_NAME);
        fs::write(&legacy_database, "legacy").unwrap();

        let result = prepare_installed_database_path(
            &install_directory.join("bilibili-live-tool.exe"),
            &legacy_database,
        )
        .unwrap();

        // The old file stays where it is and the app starts from a fresh database.
        assert_eq!(result, install_directory.join(DATABASE_FILE_NAME));
        assert!(!result.exists());
        assert_eq!(fs::read_to_string(&legacy_database).unwrap(), "legacy");
    }

    #[test]
    fn an_existing_installed_database_is_never_overwritten() {
        let directory = tempfile::tempdir().unwrap();
        let legacy_database = directory.path().join("legacy.db");
        let install_directory = directory.path().join("installed");
        fs::create_dir_all(&install_directory).unwrap();
        fs::write(&legacy_database, "legacy").unwrap();
        let installed_database = install_directory.join(DATABASE_FILE_NAME);
        fs::write(&installed_database, "installed").unwrap();

        let result = prepare_installed_database_path(
            &install_directory.join("bilibili-live-tool.exe"),
            &legacy_database,
        )
        .unwrap();

        assert_eq!(result, installed_database);
        assert_eq!(fs::read_to_string(result).unwrap(), "installed");
    }

    #[test]
    fn a_fresh_install_returns_the_install_directory_database_path() {
        let directory = tempfile::tempdir().unwrap();
        let install_directory = directory.path().join("installed");
        fs::create_dir_all(&install_directory).unwrap();

        let result = prepare_installed_database_path(
            &install_directory.join("bilibili-live-tool.exe"),
            &directory.path().join("missing-app.db"),
        )
        .unwrap();

        assert_eq!(result, install_directory.join(DATABASE_FILE_NAME));
        assert!(!result.exists());
    }
}
