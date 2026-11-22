use super::database::open_database;
use sqlx::Row;
#[tokio::test]
async fn an_incompatible_database_is_set_aside_and_recreated() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("app.db");
    // Not a SQLite file at all: migrations cannot run against it.
    std::fs::write(&path, b"this is not a database").unwrap();

    let pool = open_database(&path).await.unwrap();
    let tables: i64 = sqlx::query("SELECT COUNT(*) AS total FROM t_schema_state")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get("total");
    assert!(tables > 0, "a fresh database was expected");

    let quarantined = std::fs::read_dir(directory.path())
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().contains(".broken-"))
        .count();
    assert_eq!(quarantined, 1, "the broken file must be kept for inspection");
}
