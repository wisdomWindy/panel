use std::sync::{Arc, Mutex as StdMutex};

use ed25519_dalek::{Signer, SigningKey};
use tokio::sync::Notify;

use crate::{
    application::{
        live_data::AcceptedLiveEventConsumer,
        member::{
            AiOutputDeviceRecord, AiSpeechOutput, AiState, AiVoiceRecord, AsrProvider, LlmProvider,
            MemberRepository, MemberServiceError, MembershipState, ModerationState,
            StoredEntitlement, UnavailableAiSecretStore,
        },
    },
    domain::{
        live_data::{normalize_identity, AudienceIdentityInput, LiveEvent, LiveEventBase, RoomId},
        member::{
            AiCapabilityState, Entitlement, EntitlementStatus, MemberAccount, MemberModule,
            MemberModuleStatus, MemberShellState, ModuleId,
        },
    },
    infrastructure::{
        database::open_database,
        member::{
            Ed25519TokenVerifier, SqliteMemberRepository, UnavailableAsrProvider,
            UnavailableAuthProvider, UnavailableMembershipProvider, UnavailableModerationProvider,
        },
    },
};

use super::similarity;

struct BlockingLlm {
    entered: Notify,
    release: Notify,
}

#[async_trait::async_trait]
impl LlmProvider for BlockingLlm {
    async fn generate(&self, _: &str) -> Result<String, MemberServiceError> {
        self.entered.notify_one();
        self.release.notified().await;
        Ok("生成结果".to_owned())
    }
}

struct ImmediateLlm;

#[async_trait::async_trait]
impl LlmProvider for ImmediateLlm {
    async fn generate(&self, _: &str) -> Result<String, MemberServiceError> {
        Ok("设备断开测试".to_owned())
    }
}

#[derive(Default)]
struct MissingDeviceSpeech {
    spoken_voice_ids: StdMutex<Vec<String>>,
}

#[derive(Default)]
struct RecordingSpeech {
    spoken_voice_ids: StdMutex<Vec<String>>,
}

#[async_trait::async_trait]
impl AiSpeechOutput for RecordingSpeech {
    async fn devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
        Ok(vec![AiOutputDeviceRecord {
            device_id: "device-1".to_owned(),
            display_name: "Virtual cable".to_owned(),
            is_available: true,
            is_default: true,
        }])
    }

    async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
        Ok(vec![
            AiVoiceRecord {
                voice_id: "gentle-peach".to_owned(),
                display_name: "温柔桃子".to_owned(),
                is_available: true,
                is_default: true,
            },
            AiVoiceRecord {
                voice_id: "voice-2".to_owned(),
                display_name: "System voice 2".to_owned(),
                is_available: true,
                is_default: false,
            },
        ])
    }

    async fn speak(
        &self,
        _: &str,
        _: &str,
        voice_id: &str,
        _: f64,
    ) -> Result<(), MemberServiceError> {
        self.spoken_voice_ids
            .lock()
            .unwrap()
            .push(voice_id.to_owned());
        Ok(())
    }

    async fn stop(&self) -> Result<(), MemberServiceError> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl AiSpeechOutput for MissingDeviceSpeech {
    async fn devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
        Ok(Vec::new())
    }

    async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
        Ok(vec![AiVoiceRecord {
            voice_id: "gentle-peach".to_owned(),
            display_name: "温柔桃子".to_owned(),
            is_available: true,
            is_default: true,
        }])
    }

    async fn speak(
        &self,
        _: &str,
        _: &str,
        voice_id: &str,
        _: f64,
    ) -> Result<(), MemberServiceError> {
        self.spoken_voice_ids
            .lock()
            .unwrap()
            .push(voice_id.to_owned());
        Err(MemberServiceError::invalid("ai_audio_device_invalid"))
    }

    async fn stop(&self) -> Result<(), MemberServiceError> {
        Ok(())
    }
}

#[test]
fn similarity_is_unicode_safe_and_has_closed_boundaries() {
    assert_eq!(similarity("", ""), 1.0);
    assert_eq!(similarity("你好", "你好"), 1.0);
    assert_eq!(similarity("abc", "xyz"), 0.0);
    assert!(similarity("欢迎回来", "欢迎回來") >= 0.75);
}

#[tokio::test]
async fn signed_entitlement_and_installed_modules_derive_ready_capabilities() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("member-state.db"))
        .await
        .unwrap();
    let repository: Arc<dyn MemberRepository> = Arc::new(SqliteMemberRepository::new(pool.clone()));
    let uid = crate::domain::live_data::UserUid::parse("123").unwrap();
    repository
        .save_account(&MemberAccount {
            user_uid: uid.clone(),
            username: "member".to_owned(),
            avatar_url: None,
            logged_in_at: 1_000,
        })
        .await
        .unwrap();
    let signing_key = SigningKey::from_bytes(&[4; 32]);
    let payload = b"entitlement-v1".to_vec();
    repository
        .save_entitlement(
            &uid,
            &StoredEntitlement {
                entitlement: Entitlement {
                    status: EntitlementStatus::Active,
                    plan_id: "annual".to_owned(),
                    issued_at: 1_000,
                    expires_at: 100_000,
                    last_verified_at: 2_000,
                },
                token_signature: signing_key.sign(&payload).to_bytes().to_vec(),
                token_ciphertext: payload,
            },
            2_000,
        )
        .await
        .unwrap();
    for id in ["ai", "moderation"] {
        repository
            .upsert_module(
                &MemberModule {
                    module_id: ModuleId::parse(id).unwrap(),
                    name: id.to_owned(),
                    version: "1.0.0".to_owned(),
                    status: MemberModuleStatus::Installed,
                    downloaded_bytes: 3,
                    total_bytes: Some(3),
                    error_code: None,
                },
                &format!("{id}/1.0.0"),
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
                2_000,
            )
            .await
            .unwrap();
    }
    let state = MembershipState::new(
        repository,
        Arc::new(UnavailableAuthProvider),
        Arc::new(UnavailableMembershipProvider),
        Arc::new(Ed25519TokenVerifier::new(signing_key.verifying_key().to_bytes()).unwrap()),
    );

    let active = state.status(3_000, false).await.unwrap();
    assert_eq!(active.shell_state, MemberShellState::Active);
    assert_eq!(active.capabilities.moderation, AiCapabilityState::Ready);
    assert_eq!(
        state.status(100_000, true).await.unwrap().shell_state,
        MemberShellState::Expired
    );
}

#[tokio::test]
async fn ai_generation_rejects_a_concurrent_request() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("ai-state.db"))
        .await
        .unwrap();
    let llm = Arc::new(BlockingLlm {
        entered: Notify::new(),
        release: Notify::new(),
    });
    let state = Arc::new(AiState::new(
        Arc::new(SqliteMemberRepository::new(pool)),
        llm.clone(),
        Arc::new(UnavailableAsrProvider) as Arc<dyn AsrProvider>,
    ));
    let first_state = state.clone();
    let first =
        tokio::spawn(async move { first_state.generate("first".to_owned(), Vec::new()).await });
    llm.entered.notified().await;

    assert_eq!(
        state
            .generate("second".to_owned(), Vec::new())
            .await
            .unwrap_err()
            .code,
        "ai_generation_pending"
    );
    llm.release.notify_one();
    assert_eq!(first.await.unwrap().unwrap().result, "生成结果");
}

#[tokio::test]
async fn cancelling_an_ai_task_releases_the_generation_slot() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("ai-cancel.db"))
        .await
        .unwrap();
    let llm = Arc::new(BlockingLlm {
        entered: Notify::new(),
        release: Notify::new(),
    });
    let state = Arc::new(AiState::new(
        Arc::new(SqliteMemberRepository::new(pool)),
        llm.clone(),
        Arc::new(UnavailableAsrProvider),
    ));
    let first_state = state.clone();
    let first =
        tokio::spawn(async move { first_state.generate("first".to_owned(), Vec::new()).await });
    llm.entered.notified().await;
    first.abort();
    let _ = first.await;

    let next_state = state.clone();
    let next =
        tokio::spawn(async move { next_state.generate("next".to_owned(), Vec::new()).await });
    tokio::time::timeout(std::time::Duration::from_secs(1), llm.entered.notified())
        .await
        .unwrap();
    llm.release.notify_one();
    assert!(next.await.unwrap().is_ok());
}

#[tokio::test]
async fn voice_device_loss_disables_the_confirmed_route_without_fallback() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("ai-device-loss.db"))
        .await
        .unwrap();
    let repository = Arc::new(SqliteMemberRepository::new(pool));
    let mut voice = repository.load_ai_voice_config().await.unwrap();
    voice.is_enabled = true;
    voice.output_device_id = Some("removed-device".to_owned());
    voice.is_audio_route_confirmed = true;
    repository.save_ai_voice_config(&voice).await.unwrap();
    let speech = Arc::new(MissingDeviceSpeech::default());
    let state = AiState::new_with_services(
        repository.clone(),
        Arc::new(ImmediateLlm),
        Arc::new(UnavailableAiSecretStore),
        Arc::new(UnavailableAsrProvider),
        speech.clone(),
    );

    state.generate("测试".to_owned(), Vec::new()).await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(1), async {
        loop {
            if !repository.load_ai_voice_config().await.unwrap().is_enabled {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();

    let degraded = repository.load_ai_voice_config().await.unwrap();
    assert!(!degraded.is_enabled);
    assert!(!degraded.is_audio_route_confirmed);
    assert_eq!(degraded.output_device_id.as_deref(), Some("removed-device"));
    assert_eq!(
        speech.spoken_voice_ids.lock().unwrap().as_slice(),
        ["gentle-peach"]
    );
    assert_eq!(
        state
            .voice_queue_snapshot()
            .await
            .unwrap()
            .last_error_code
            .as_deref(),
        Some("ai_audio_device_missing")
    );
    state.shutdown().await;
}

#[tokio::test]
async fn voice_route_persists_and_tests_the_selected_voice() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("ai-voice-tone.db"))
        .await
        .unwrap();
    let repository = Arc::new(SqliteMemberRepository::new(pool));
    let speech = Arc::new(RecordingSpeech::default());
    let state = AiState::new_with_services(
        repository.clone(),
        Arc::new(ImmediateLlm),
        Arc::new(UnavailableAiSecretStore),
        Arc::new(UnavailableAsrProvider),
        speech.clone(),
    );

    state
        .set_output_device("device-1".to_owned(), "voice-2".to_owned(), 0.5, true)
        .await
        .unwrap();
    assert_eq!(
        repository.load_ai_voice_config().await.unwrap().voice_id,
        "voice-2"
    );
    state
        .test_output_device("device-1", "voice-2", 0.5)
        .await
        .unwrap();
    assert_eq!(
        speech.spoken_voice_ids.lock().unwrap().as_slice(),
        ["voice-2"]
    );
    let mut config = repository.load_ai_voice_config().await.unwrap();
    config.is_enabled = true;
    repository.save_ai_voice_config(&config).await.unwrap();
    state.generate("测试".to_owned(), Vec::new()).await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(1), async {
        loop {
            if speech.spoken_voice_ids.lock().unwrap().len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        speech.spoken_voice_ids.lock().unwrap().as_slice(),
        ["voice-2", "voice-2"]
    );
    assert_eq!(
        state
            .set_output_device("device-1".to_owned(), "missing".to_owned(), 0.5, true)
            .await
            .unwrap_err()
            .code,
        "ai_tts_voice_invalid"
    );
    assert_eq!(
        state
            .test_output_device("device-1", "missing", 0.5)
            .await
            .unwrap_err()
            .code,
        "ai_tts_voice_invalid"
    );
    assert_eq!(speech.spoken_voice_ids.lock().unwrap().len(), 2);
    state.shutdown().await;
}

#[tokio::test]
async fn moderation_frequency_alert_starts_after_the_sixth_message() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("moderation-state.db"))
        .await
        .unwrap();
    let repository: Arc<dyn MemberRepository> = Arc::new(SqliteMemberRepository::new(pool));
    let membership = Arc::new(MembershipState::new(
        repository.clone(),
        Arc::new(UnavailableAuthProvider),
        Arc::new(UnavailableMembershipProvider),
        Arc::new(crate::infrastructure::member::UnavailableEntitlementVerifier),
    ));
    let moderation = ModerationState::new(
        repository,
        Arc::new(UnavailableModerationProvider),
        membership,
    );

    for index in 0..5 {
        moderation
            .consume_accepted(&danmaku(index, &format!("独立消息-{index}")))
            .await
            .unwrap();
    }
    assert!(moderation.alerts().await.is_empty());
    moderation
        .consume_accepted(&danmaku(5, "完全不同的第六条"))
        .await
        .unwrap();

    let alerts = moderation.alerts().await;
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].violation_description, "frequency");
}

fn danmaku(index: i64, content: &str) -> LiveEvent {
    LiveEvent::Danmu {
        base: LiveEventBase {
            event_id: format!("event-{index}"),
            room_id: RoomId::parse("100").unwrap(),
            occurred_at: 10_000 + index * 1_000,
            audience: normalize_identity(AudienceIdentityInput {
                user_uid: "999".to_owned(),
                username: "viewer".to_owned(),
                user_level: None,
                medal_name: None,
                medal_level: None,
                is_admin: false,
                captain_level: None,
                is_vip: false,
            })
            .unwrap(),
        },
        content: content.to_owned(),
    }
}
