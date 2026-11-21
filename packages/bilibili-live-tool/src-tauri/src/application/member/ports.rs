use std::path::PathBuf;

use crate::domain::{
    live_data::UserUid,
    member::{Entitlement, MemberAccount, MemberModule, ModuleId, OrderId},
};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{code}")]
pub struct MemberServiceError {
    pub code: &'static str,
    pub retryable: bool,
}

impl MemberServiceError {
    pub const fn unavailable(code: &'static str) -> Self {
        Self {
            code,
            retryable: true,
        }
    }

    pub const fn invalid(code: &'static str) -> Self {
        Self {
            code,
            retryable: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredEntitlement {
    pub entitlement: Entitlement,
    pub token_ciphertext: Vec<u8>,
    pub token_signature: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrderStatus {
    Pending,
    Paid,
    Cancelled,
    Failed,
    TimedOut,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderRecord {
    pub order_id: OrderId,
    pub user_uid: UserUid,
    pub plan_id: String,
    pub plan_label: String,
    pub amount_minor: u64,
    pub currency: String,
    pub status: OrderStatus,
    pub created_at: i64,
    pub paid_at: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AiConfigRecord {
    pub identity: String,
    pub speaking_style: String,
    pub system_prompt: String,
    pub provider: String,
    pub endpoint: Option<String>,
    pub model: String,
    pub api_key_ciphertext: Option<Vec<u8>>,
    pub temperature: f64,
    pub max_length: u32,
    pub max_sentences: u32,
    pub is_sensitive_filter_enabled: bool,
    pub banned_words: Vec<String>,
    pub danmaku_context_count: u32,
    pub manual_context_count: u32,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiLogRecord {
    pub id: i64,
    pub operation_type: String,
    pub username: Option<String>,
    pub user_uid: Option<String>,
    pub original_content: Option<String>,
    pub ai_content: Option<String>,
    pub generation_status: String,
    pub reject_reason: Option<String>,
    pub room_id: String,
    pub created_at: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiModelConnectionState {
    Untested,
    Available,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiReasoningEffort {
    None,
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
    Max,
}

impl AiReasoningEffort {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Minimal => "minimal",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::XHigh => "xhigh",
            Self::Max => "max",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AiModelProfileRecord {
    pub profile_id: String,
    pub name: String,
    pub provider_kind: String,
    pub base_url: String,
    pub model: String,
    pub reasoning_effort: AiReasoningEffort,
    pub max_tokens: u32,
    pub timeout_seconds: u32,
    pub proxy_url: Option<String>,
    pub is_enabled: bool,
    pub is_default: bool,
    pub connection_state: AiModelConnectionState,
    pub has_api_key: bool,
    pub updated_at: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AiVoiceConfigRecord {
    pub is_enabled: bool,
    pub trigger_scope: String,
    pub keywords: Vec<String>,
    pub global_cooldown_seconds: u32,
    pub user_cooldown_seconds: u32,
    pub queue_capacity: u32,
    pub is_duplicate_filter_enabled: bool,
    pub output_device_id: Option<String>,
    pub voice_id: String,
    pub volume: f64,
    pub is_audio_route_confirmed: bool,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiOutputDeviceRecord {
    pub device_id: String,
    pub display_name: String,
    pub is_available: bool,
    pub is_default: bool,
}

pub const DEFAULT_AI_VOICE_ID: &str = "gentle-peach";
pub const DEFAULT_AI_VOICE_DISPLAY_NAME: &str = "温柔桃子";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiVoiceRecord {
    pub voice_id: String,
    pub display_name: String,
    pub is_available: bool,
    pub is_default: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiVoiceQueueItemRecord {
    pub item_id: String,
    pub text: String,
    pub priority: u8,
    pub created_at: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiVoiceQueueState {
    Idle,
    Speaking,
    Paused,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AiVoiceQueueSnapshotRecord {
    pub state: AiVoiceQueueState,
    pub current: Option<AiVoiceQueueItemRecord>,
    pub pending: Vec<AiVoiceQueueItemRecord>,
    pub capacity: u32,
    pub volume: f64,
    pub selected_device_id: Option<String>,
    pub selected_voice_id: String,
    pub is_audio_route_confirmed: bool,
    pub last_error_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModerationConfigRecord {
    pub is_auto_enabled: bool,
    pub default_mute_duration: String,
    pub frequency_window_seconds: u32,
    pub frequency_threshold: u32,
    pub repeat_window_seconds: u32,
    pub repeat_threshold: u32,
    pub similarity_window_seconds: u32,
    pub similarity_threshold: u32,
    pub similarity_ratio: f64,
    pub is_risk_confirmed: bool,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiGenerationRecord {
    pub generation_id: String,
    pub status: AiGenerationStatus,
    pub result: String,
    /// PRD 9.13 UI sketch: the context actually sent to the model.
    pub context_used: Vec<String>,
    pub last_sequence: u64,
    pub error_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiGenerationStatus {
    Assembling,
    Streaming,
    Completed,
    Failed,
    Rejected,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModerationAlertRecord {
    pub alert_id: String,
    pub user_uid: UserUid,
    pub username: String,
    pub violation_description: String,
    pub content_example: String,
    pub occurred_at: i64,
    pub status: ModerationAlertStatus,
    pub error_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModerationAlertStatus {
    Pending,
    Executing,
    Muted,
    Ignored,
    Failed,
}

pub trait MemberEventSink: Send + Sync {
    fn publish_status(&self, snapshot: &crate::domain::member::MemberSnapshot);
    fn publish_ai_generation(&self, generation: &AiGenerationRecord);
    fn publish_moderation_alert(&self, alert: &ModerationAlertRecord);
}

pub struct NoopMemberEventSink;

impl MemberEventSink for NoopMemberEventSink {
    fn publish_status(&self, _: &crate::domain::member::MemberSnapshot) {}
    fn publish_ai_generation(&self, _: &AiGenerationRecord) {}
    fn publish_moderation_alert(&self, _: &ModerationAlertRecord) {}
}

#[async_trait::async_trait]
pub trait MemberRepository: Send + Sync {
    async fn query_ai_logs(
        &self,
        user_uid: Option<&str>,
        started_at: Option<i64>,
        ended_at: Option<i64>,
    ) -> Result<Vec<AiLogRecord>, MemberServiceError>;
    async fn load_account(&self) -> Result<Option<MemberAccount>, MemberServiceError>;
    async fn save_account(&self, account: &MemberAccount) -> Result<(), MemberServiceError>;
    async fn remove_account(&self) -> Result<bool, MemberServiceError>;
    async fn load_entitlement(&self) -> Result<Option<StoredEntitlement>, MemberServiceError>;
    async fn save_entitlement(
        &self,
        user_uid: &UserUid,
        entitlement: &StoredEntitlement,
        updated_at: i64,
    ) -> Result<(), MemberServiceError>;
    async fn list_modules(&self) -> Result<Vec<MemberModule>, MemberServiceError>;
    async fn upsert_module(
        &self,
        module: &MemberModule,
        relative_install_path: &str,
        sha256_hex: &str,
        updated_at: i64,
    ) -> Result<(), MemberServiceError>;
    async fn remove_module(&self, module_id: &ModuleId) -> Result<bool, MemberServiceError>;
    async fn list_orders(&self) -> Result<Vec<OrderRecord>, MemberServiceError>;
    async fn upsert_order(&self, order: &OrderRecord) -> Result<(), MemberServiceError>;
    async fn load_ai_config(&self) -> Result<Option<AiConfigRecord>, MemberServiceError>;
    async fn save_ai_config(&self, config: &AiConfigRecord) -> Result<(), MemberServiceError>;
    async fn list_ai_model_profiles(
        &self,
    ) -> Result<Vec<AiModelProfileRecord>, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_model_storage_unavailable",
        ))
    }
    async fn load_ai_model_profile(
        &self,
        _profile_id: &str,
    ) -> Result<Option<AiModelProfileRecord>, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_model_storage_unavailable",
        ))
    }
    async fn load_default_ai_model_profile(
        &self,
    ) -> Result<Option<AiModelProfileRecord>, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_model_storage_unavailable",
        ))
    }
    async fn save_ai_model_profile(
        &self,
        _profile: &AiModelProfileRecord,
    ) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_model_storage_unavailable",
        ))
    }
    async fn delete_ai_model_profile(&self, _profile_id: &str) -> Result<bool, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_model_storage_unavailable",
        ))
    }
    async fn set_default_ai_model_profile(
        &self,
        _profile_id: &str,
        _updated_at: i64,
    ) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_model_storage_unavailable",
        ))
    }
    async fn load_ai_voice_config(&self) -> Result<AiVoiceConfigRecord, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_voice_storage_unavailable",
        ))
    }
    async fn save_ai_voice_config(
        &self,
        _config: &AiVoiceConfigRecord,
    ) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_voice_storage_unavailable",
        ))
    }
    async fn load_moderation_config(
        &self,
    ) -> Result<Option<ModerationConfigRecord>, MemberServiceError>;
    async fn save_moderation_config(
        &self,
        config: &ModerationConfigRecord,
    ) -> Result<(), MemberServiceError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthQrChallenge {
    pub provider_key: String,
    pub url: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthPollState {
    Pending,
    Scanned,
    Expired,
    Success,
}

#[async_trait::async_trait]
pub trait AuthProvider: Send + Sync {
    async fn generate_qr(&self) -> Result<AuthQrChallenge, MemberServiceError>;
    async fn poll_qr(&self, provider_key: &str) -> Result<AuthPollState, MemberServiceError>;
    async fn current_account(&self) -> Result<MemberAccount, MemberServiceError>;
    async fn query_uid(&self, user_uid: &UserUid) -> Result<MemberAccount, MemberServiceError>;
    async fn clear_session(&self) -> Result<(), MemberServiceError>;
}

#[async_trait::async_trait]
pub trait MembershipProvider: Send + Sync {
    async fn plans(&self) -> Result<Vec<(String, String, u64, String)>, MemberServiceError>;
    async fn purchase(&self, plan_id: &str) -> Result<OrderRecord, MemberServiceError>;
    async fn cancel_order(&self, order_id: &OrderId) -> Result<OrderRecord, MemberServiceError>;
}

pub trait EntitlementVerifier: Send + Sync {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), MemberServiceError>;
}

pub trait ArtifactVerifier: Send + Sync {
    fn verify_sha256(&self, bytes: &[u8], expected_hex: &str) -> Result<(), MemberServiceError>;
    fn module_path(
        &self,
        module_id: &ModuleId,
        version: &str,
    ) -> Result<PathBuf, MemberServiceError>;
    fn archive_entry_path(&self, entry: &str) -> Result<PathBuf, MemberServiceError>;
}

#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, MemberServiceError>;
    async fn test_profile(
        &self,
        _profile: &AiModelProfileRecord,
        _api_key: Option<&str>,
    ) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable("llm_unavailable"))
    }
}

#[async_trait::async_trait]
pub trait AiSecretStore: Send + Sync {
    async fn get(&self, profile_id: &str) -> Result<Option<String>, MemberServiceError>;
    async fn set(&self, profile_id: &str, secret: &str) -> Result<(), MemberServiceError>;
    async fn delete(&self, profile_id: &str) -> Result<(), MemberServiceError>;
}

#[async_trait::async_trait]
pub trait AiSpeechOutput: Send + Sync {
    async fn devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError>;
    async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError>;
    async fn speak(
        &self,
        text: &str,
        device_id: &str,
        voice_id: &str,
        volume: f64,
    ) -> Result<(), MemberServiceError>;
    async fn stop(&self) -> Result<(), MemberServiceError>;
}

pub struct UnavailableAiSecretStore;

#[async_trait::async_trait]
impl AiSecretStore for UnavailableAiSecretStore {
    async fn get(&self, _profile_id: &str) -> Result<Option<String>, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_secret_store_unavailable",
        ))
    }

    async fn set(&self, _profile_id: &str, _secret: &str) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_secret_store_unavailable",
        ))
    }

    async fn delete(&self, _profile_id: &str) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_secret_store_unavailable",
        ))
    }
}

pub struct UnavailableAiSpeechOutput;

#[async_trait::async_trait]
impl AiSpeechOutput for UnavailableAiSpeechOutput {
    async fn devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
        Err(MemberServiceError::unavailable(
            "ai_audio_device_unavailable",
        ))
    }

    async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
        Err(MemberServiceError::unavailable("ai_tts_unavailable"))
    }

    async fn speak(
        &self,
        _text: &str,
        _device_id: &str,
        _voice_id: &str,
        _volume: f64,
    ) -> Result<(), MemberServiceError> {
        Err(MemberServiceError::unavailable("ai_tts_unavailable"))
    }

    async fn stop(&self) -> Result<(), MemberServiceError> {
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait ModerationProvider: Send + Sync {
    async fn mute(&self, user_uid: &UserUid, duration: &str) -> Result<(), MemberServiceError>;
}
