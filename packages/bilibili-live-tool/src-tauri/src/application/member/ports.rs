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
    pub is_sensitive_filter_enabled: bool,
    pub banned_words: Vec<String>,
    pub danmaku_context_count: u32,
    pub asr_context_count: u32,
    pub manual_context_count: u32,
    pub updated_at: i64,
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
pub struct AsrStatusRecord {
    pub state: AsrState,
    pub model_name: Option<String>,
    pub language: String,
    pub progress_percent: u8,
    pub error_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AsrState {
    Unavailable,
    ModelMissing,
    Downloading,
    Ready,
    Starting,
    Listening,
    Error,
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
    fn publish_asr_status(&self, status: &AsrStatusRecord);
    fn publish_moderation_alert(&self, alert: &ModerationAlertRecord);
}

pub struct NoopMemberEventSink;

impl MemberEventSink for NoopMemberEventSink {
    fn publish_status(&self, _: &crate::domain::member::MemberSnapshot) {}
    fn publish_ai_generation(&self, _: &AiGenerationRecord) {}
    fn publish_asr_status(&self, _: &AsrStatusRecord) {}
    fn publish_moderation_alert(&self, _: &ModerationAlertRecord) {}
}

#[async_trait::async_trait]
pub trait MemberRepository: Send + Sync {
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
}

#[async_trait::async_trait]
pub trait AsrProvider: Send + Sync {
    async fn start(&self) -> Result<(), MemberServiceError>;
    async fn stop(&self) -> Result<(), MemberServiceError>;
}

#[async_trait::async_trait]
pub trait ModerationProvider: Send + Sync {
    async fn mute(&self, user_uid: &UserUid, duration: &str) -> Result<(), MemberServiceError>;
}
