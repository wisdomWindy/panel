use ed25519_dalek::{Signer, SigningKey};

#[tokio::test]
async fn ai_logs_are_queryable_without_membership_and_filter_by_uid_and_time() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("logs.db"))
        .await
        .unwrap();
    let repository = SqliteMemberRepository::new(pool.clone());
    assert!(repository
        .query_ai_logs(None, None, None)
        .await
        .unwrap()
        .is_empty());
    for (uid, time) in [
        ("123", "2026-01-01 00:00:00"),
        ("456", "2026-01-02 00:00:00"),
    ] {
        sqlx::query("INSERT INTO t_ai_log (operation_type, user_uid, generation_status, room_id, create_time) VALUES ('reply', ?, 'completed', '1', ?)")
            .bind(uid).bind(time).execute(&pool).await.unwrap();
    }
    let logs = repository.query_ai_logs(None, None, None).await.unwrap();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].user_uid.as_deref(), Some("456"));
    assert_eq!(logs[0].created_at, 1767312000000);
    assert_eq!(
        repository
            .query_ai_logs(Some("123"), None, None)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        repository
            .query_ai_logs(None, Some(1767312000000), Some(1767312000000))
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(repository
        .query_ai_logs(Some("789"), None, None)
        .await
        .unwrap()
        .is_empty());
}

use crate::{
    application::member::{
        AiConfigRecord, AiModelConnectionState, AiModelProfileRecord, AiReasoningEffort,
        ArtifactVerifier, AuthProvider, EntitlementVerifier, LlmProvider, MemberRepository,
        ModerationConfigRecord, ModerationProvider, OrderRecord, OrderStatus, StoredEntitlement,
    },
    domain::{
        live_data::UserUid,
        member::{
            Entitlement, EntitlementStatus, MemberAccount, MemberModule, MemberModuleStatus,
            ModuleId, OrderId,
        },
    },
    infrastructure::{
        database::open_database,
        member::{
            Ed25519TokenVerifier, LocalArtifactVerifier, SqliteMemberRepository,
            UnavailableAuthProvider, UnavailableLlmProvider, UnavailableModerationProvider,
        },
    },
};

#[test]
fn artifact_integrity_and_paths_fail_closed() {
    let directory = tempfile::tempdir().unwrap();
    let verifier = LocalArtifactVerifier::new(directory.path().join("modules"));
    let module_id = ModuleId::parse("ai-module").unwrap();

    verifier
        .verify_sha256(
            b"abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        )
        .unwrap();
    assert_eq!(
        verifier
            .verify_sha256(
                b"tampered",
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
            )
            .unwrap_err()
            .code,
        "MEMBER_MODULE_HASH_MISMATCH"
    );
    assert_eq!(
        verifier
            .module_path(&module_id, "../escape")
            .unwrap_err()
            .code,
        "MEMBER_MODULE_PATH_INVALID"
    );
    assert_eq!(
        verifier
            .archive_entry_path("../escape.dll")
            .unwrap_err()
            .code,
        "MEMBER_MODULE_ENTRY_INVALID"
    );
    assert!(verifier
        .archive_entry_path("bin/module.dll")
        .unwrap()
        .starts_with(directory.path().join("modules")));
}

#[test]
fn token_signature_rejects_tampering() {
    let signing_key = SigningKey::from_bytes(&[7; 32]);
    let verifier = Ed25519TokenVerifier::new(signing_key.verifying_key().to_bytes()).unwrap();
    let signature = signing_key.sign(b"signed entitlement");

    verifier
        .verify(b"signed entitlement", &signature.to_bytes())
        .unwrap();
    assert_eq!(
        verifier
            .verify(b"tampered entitlement", &signature.to_bytes())
            .unwrap_err()
            .code,
        "MEMBER_ENTITLEMENT_SIGNATURE_INVALID"
    );
}

#[tokio::test]
async fn repository_round_trips_member_records_without_plaintext_secrets() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("member.db"))
        .await
        .unwrap();
    let repository = SqliteMemberRepository::new(pool.clone());
    let user_uid = UserUid::parse("12345").unwrap();
    let account = MemberAccount {
        user_uid: user_uid.clone(),
        username: "主播".to_owned(),
        avatar_url: Some("https://example.invalid/avatar.png".to_owned()),
        logged_in_at: 10_000,
    };
    repository.save_account(&account).await.unwrap();
    assert_eq!(repository.load_account().await.unwrap(), Some(account));

    let stored = StoredEntitlement {
        entitlement: Entitlement {
            status: EntitlementStatus::Active,
            plan_id: "annual".to_owned(),
            issued_at: 10_000,
            expires_at: 20_000,
            last_verified_at: 11_000,
        },
        token_ciphertext: vec![1, 2, 3],
        token_signature: vec![4; 64],
    };
    repository
        .save_entitlement(&user_uid, &stored, 12_000)
        .await
        .unwrap();
    assert_eq!(repository.load_entitlement().await.unwrap(), Some(stored));

    let module = MemberModule {
        module_id: ModuleId::parse("ai").unwrap(),
        name: "AI 模块".to_owned(),
        version: "1.0.0".to_owned(),
        status: MemberModuleStatus::Installed,
        downloaded_bytes: 3,
        total_bytes: Some(3),
        error_code: None,
    };
    repository
        .upsert_module(
            &module,
            "ai/1.0.0",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            12_000,
        )
        .await
        .unwrap();
    assert_eq!(repository.list_modules().await.unwrap(), vec![module]);

    let order = OrderRecord {
        order_id: OrderId::parse("order-1").unwrap(),
        user_uid,
        plan_id: "annual".to_owned(),
        plan_label: "年付".to_owned(),
        amount_minor: 12_900,
        currency: "cny".to_owned(),
        status: OrderStatus::Pending,
        created_at: 12_000,
        paid_at: None,
    };
    repository.upsert_order(&order).await.unwrap();
    let mut expected_order = order;
    expected_order.currency = "CNY".to_owned();
    assert_eq!(
        repository.list_orders().await.unwrap(),
        vec![expected_order]
    );

    let ai_config = AiConfigRecord {
        identity: "专业主播".to_owned(),
        speaking_style: "short".to_owned(),
        system_prompt: "仅生成本地建议".to_owned(),
        provider: "compatible".to_owned(),
        endpoint: None,
        model: "model".to_owned(),
        api_key_ciphertext: Some(vec![9, 8, 7]),
        temperature: 0.7,
        max_length: 100,
        max_sentences: 2,
        is_sensitive_filter_enabled: true,
        banned_words: vec!["敏感".to_owned()],
        danmaku_context_count: 20,
        manual_context_count: 1,
        updated_at: 12_000,
    };
    repository.save_ai_config(&ai_config).await.unwrap();
    assert_eq!(repository.load_ai_config().await.unwrap(), Some(ai_config));

    let profile = AiModelProfileRecord {
        profile_id: "00000000-0000-4000-8000-000000000001".to_owned(),
        name: "Local model".to_owned(),
        provider_kind: "local".to_owned(),
        base_url: "http://127.0.0.1:11434/v1".to_owned(),
        model: "qwen".to_owned(),
        reasoning_effort: AiReasoningEffort::Medium,
        max_tokens: 512,
        timeout_seconds: 30,
        proxy_url: None,
        is_enabled: true,
        is_default: false,
        connection_state: AiModelConnectionState::Untested,
        has_api_key: false,
        updated_at: 13_000,
    };
    repository.save_ai_model_profile(&profile).await.unwrap();
    repository
        .set_default_ai_model_profile(&profile.profile_id, 14_000)
        .await
        .unwrap();
    let default = repository
        .load_default_ai_model_profile()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(default.profile_id, profile.profile_id);
    assert!(default.is_default);

    let voice = repository.load_ai_voice_config().await.unwrap();
    assert!(!voice.is_enabled);
    assert_eq!(voice.trigger_scope, "questions");
    assert_eq!(voice.queue_capacity, 20);
    assert_eq!(voice.voice_id, "gentle-peach");
    let mut updated_voice = voice.clone();
    updated_voice.voice_id = "system-voice-2".to_owned();
    updated_voice.updated_at += 1;
    repository
        .save_ai_voice_config(&updated_voice)
        .await
        .unwrap();
    assert_eq!(
        repository.load_ai_voice_config().await.unwrap().voice_id,
        "system-voice-2"
    );

    let moderation = ModerationConfigRecord {
        is_auto_enabled: false,
        default_mute_duration: "1h".to_owned(),
        frequency_window_seconds: 10,
        frequency_threshold: 5,
        repeat_window_seconds: 60,
        repeat_threshold: 3,
        similarity_window_seconds: 60,
        similarity_threshold: 5,
        similarity_ratio: 0.8,
        is_risk_confirmed: false,
        updated_at: 12_000,
    };
    repository
        .save_moderation_config(&moderation)
        .await
        .unwrap();
    assert_eq!(
        repository.load_moderation_config().await.unwrap(),
        Some(moderation)
    );

    let account_columns: Vec<String> =
        sqlx::query_scalar("SELECT name FROM pragma_table_info('t_account')")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert!(!account_columns
        .iter()
        .any(|column| column.contains("token")));
}

#[tokio::test]
async fn invalid_persisted_enum_is_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("invalid-member.db"))
        .await
        .unwrap();
    let mut connection = pool.acquire().await.unwrap();
    sqlx::query("PRAGMA ignore_check_constraints = ON")
        .execute(&mut *connection)
        .await
        .unwrap();
    sqlx::query("INSERT INTO t_module (module_id, name, version, relative_install_path, sha256_hex, status, downloaded_bytes, total_bytes, updated_at) VALUES ('ai', 'AI', '1', 'ai/1', ?, 'unknown', 0, 0, 1)")
        .bind("0".repeat(64))
        .execute(&mut *connection)
        .await
        .unwrap();
    drop(connection);
    let repository = SqliteMemberRepository::new(pool);

    assert_eq!(
        repository.list_modules().await.unwrap_err().code,
        "MEMBER_DATA_INVALID"
    );
}

#[tokio::test]
async fn unavailable_adapters_return_stable_redacted_errors() {
    let uid = UserUid::parse("1").unwrap();

    assert_eq!(
        UnavailableAuthProvider.generate_qr().await.unwrap_err(),
        crate::application::member::MemberServiceError::unavailable("MEMBER_AUTH_QR_UNAVAILABLE")
    );
    assert_eq!(
        UnavailableLlmProvider
            .generate("secret prompt")
            .await
            .unwrap_err()
            .code,
        "NET_LLM_UNAVAILABLE"
    );
    assert_eq!(
        UnavailableModerationProvider
            .mute(&uid, "1h")
            .await
            .unwrap_err()
            .code,
        "MEMBER_MODERATION_API_UNAVAILABLE"
    );
}
