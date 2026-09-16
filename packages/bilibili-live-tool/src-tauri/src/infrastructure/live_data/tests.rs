use crate::{
    application::paths::PathPurpose,
    domain::live_data::{SensitiveMatchType, SensitiveRule, SensitiveWordCategory},
    infrastructure::{database::open_database, path_grants::PathGrantStore},
};

use super::{
    decode_blacklist_csv, decode_sensitive_words, encode_blacklist_csv, encode_sensitive_words,
    BlacklistFileRow, LiveDataRepository, SensitiveFileFormat,
};

fn sensitive_rule(id: i64, word: &str) -> SensitiveRule {
    SensitiveRule::new(
        id,
        word,
        SensitiveWordCategory::Custom,
        SensitiveMatchType::Exact,
        true,
    )
    .unwrap()
}

#[tokio::test]
async fn repository_crud_and_failed_batch_preserve_committed_snapshot() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("app.db"))
        .await
        .unwrap();
    let repository = LiveDataRepository::new(pool);

    let first = repository
        .insert_sensitive(&sensitive_rule(1, "first"))
        .await
        .unwrap();
    assert_eq!(first.word, "first");
    assert_eq!(repository.list_sensitive().await.unwrap().len(), 1);

    let result = repository
        .replace_sensitive(&[
            sensitive_rule(2, "duplicate"),
            sensitive_rule(3, "duplicate"),
        ])
        .await;
    assert!(result.is_err());
    assert_eq!(repository.list_sensitive().await.unwrap(), [first]);

    let entry = repository
        .upsert_blacklist("9", "viewer", Some("reason"))
        .await
        .unwrap();
    assert_eq!(entry.uid.as_str(), "9");
    let updated = repository
        .upsert_blacklist("9", "viewer-2", None)
        .await
        .unwrap();
    assert_eq!(updated.username, "viewer-2");
    assert_eq!(repository.list_blacklist().await.unwrap().len(), 1);
}

#[test]
fn sensitive_and_blacklist_codecs_round_trip_approved_fields() {
    let rules = [sensitive_rule(1, "comma,quote\"")];
    for format in [
        SensitiveFileFormat::Csv,
        SensitiveFileFormat::Json,
        SensitiveFileFormat::Txt,
    ] {
        let encoded = encode_sensitive_words(&rules, format).unwrap();
        let decoded = decode_sensitive_words(&encoded, format).unwrap();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].word, "comma,quote\"");
        assert_eq!(decoded[0].match_type, SensitiveMatchType::Exact);
    }

    let rows = [BlacklistFileRow {
        uid: "9".into(),
        username: "观众,九".into(),
        reason: Some("quoted \"reason\"".into()),
        created_at: Some(1_700_000_000_000),
    }];
    let decoded = decode_blacklist_csv(&encode_blacklist_csv(&rows).unwrap()).unwrap();
    assert_eq!(decoded, rows);
}

#[test]
fn save_file_grant_accepts_a_new_target_and_rejects_wrong_purpose() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("stats.csv");
    let store = PathGrantStore::new();
    let grant = store
        .grant_selection(Some(target.clone()), PathPurpose::StatExport)
        .unwrap()
        .unwrap();

    assert_eq!(
        store
            .resolve_root(grant.id(), PathPurpose::StatExport)
            .unwrap(),
        directory.path().canonicalize().unwrap().join("stats.csv")
    );
    assert!(store
        .resolve_root(grant.id(), PathPurpose::BlacklistExport)
        .is_err());
}
