use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

const DATABASE_FILE_NAME: &str = "app.db";
const SQLITE_SIDECAR_SUFFIXES: [&str; 2] = ["-wal", "-shm"];

pub fn resolve_database_path(app_data_directory: &Path) -> io::Result<PathBuf> {
    let legacy_database_path = app_data_directory.join(DATABASE_FILE_NAME);
    if cfg!(debug_assertions) {
        return Ok(legacy_database_path);
    }

    prepare_installed_database_path(&std::env::current_exe()?, &legacy_database_path)
}

fn prepare_installed_database_path(
    executable_path: &Path,
    legacy_database_path: &Path,
) -> io::Result<PathBuf> {
    let install_directory = executable_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "the application executable has no parent directory",
        )
    })?;
    let installed_database_path = install_directory.join(DATABASE_FILE_NAME);

    if installed_database_path.exists() || !legacy_database_path.exists() {
        return Ok(installed_database_path);
    }

    fs::create_dir_all(install_directory)?;
    let temporary_database_path = install_directory.join("app.db.migrating");
    fs::copy(legacy_database_path, &temporary_database_path)?;

    for suffix in SQLITE_SIDECAR_SUFFIXES {
        let source = sidecar_path(legacy_database_path, suffix);
        let destination = sidecar_path(&installed_database_path, suffix);
        if source.exists() && !destination.exists() {
            if let Err(error) = fs::copy(source, destination) {
                let _ = fs::remove_file(&temporary_database_path);
                return Err(error);
            }
        }
    }

    if let Err(error) = fs::rename(&temporary_database_path, &installed_database_path) {
        let _ = fs::remove_file(&temporary_database_path);
        return Err(error);
    }

    Ok(installed_database_path)
}

fn sidecar_path(database_path: &Path, suffix: &str) -> PathBuf {
    let mut value = OsString::from(database_path.as_os_str());
    value.push(suffix);
    PathBuf::from(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_location_is_the_executable_directory_and_migrates_sqlite_files() {
        let directory = tempfile::tempdir().unwrap();
        let legacy_directory = directory.path().join("legacy");
        let install_directory = directory.path().join("installed");
        fs::create_dir_all(&legacy_directory).unwrap();
        fs::create_dir_all(&install_directory).unwrap();
        let legacy_database = legacy_directory.join(DATABASE_FILE_NAME);
        fs::write(&legacy_database, "main").unwrap();
        fs::write(sidecar_path(&legacy_database, "-wal"), "wal").unwrap();
        fs::write(sidecar_path(&legacy_database, "-shm"), "shm").unwrap();

        let result = prepare_installed_database_path(
            &install_directory.join("bilibili-live-tool.exe"),
            &legacy_database,
        )
        .unwrap();

        assert_eq!(result, install_directory.join(DATABASE_FILE_NAME));
        assert_eq!(fs::read_to_string(&result).unwrap(), "main");
        assert_eq!(
            fs::read_to_string(sidecar_path(&result, "-wal")).unwrap(),
            "wal"
        );
        assert_eq!(
            fs::read_to_string(sidecar_path(&result, "-shm")).unwrap(),
            "shm"
        );
        assert_eq!(fs::read_to_string(&legacy_database).unwrap(), "main");
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
