use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, EventTarget, State};

use crate::{
    application::member::{
        now_millis, AiConfigRecord, AiGenerationRecord, AiGenerationStatus, AiModelConnectionState,
        AiModelProfileRecord, AiOutputDeviceRecord, AiReasoningEffort, AiVoiceConfigRecord,
        AiVoiceQueueItemRecord, AiVoiceQueueSnapshotRecord, AiVoiceQueueState, AiVoiceRecord,
        MemberEventSink, MemberServiceError, ModerationAlertRecord, ModerationAlertStatus,
        ModerationConfigRecord, OrderRecord, OrderStatus,
    },
    domain::{
        live_data::UserUid,
        member::{
            evaluate_entitlement, module_progress, AiCapabilityState, EntitlementStatus,
            EntitlementValidity, MemberAccount, MemberModule, MemberModuleStatus, MemberShellState,
            MemberSnapshot,
        },
    },
    ipc::error::DesktopError,
    AppState,
};

pub const MEMBER_EVENT_NAMES: &[&str] = &[
    "auth_qrcode_status",
    "auth_logged_in",
    "membership_expired",
    "membership_renewed",
    "module_download_progress",
    "module_installed",
    "moderation_alert",
    "moderation_action_result",
];

pub const AI_EVENT_NAMES: &[&str] = &["ai_generation_delta", "ai_generation_finished"];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MemberShellStateDto {
    Unavailable,
    LoggedOut,
    Unpaid,
    Active,
    Expired,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MemberCapabilityStateDto {
    Unavailable,
    MissingModule,
    Ready,
    Expired,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberAccountDto {
    pub user_uid: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub logged_in_at: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberEntitlementDto {
    pub status: MemberEntitlementStateDto,
    pub plan_id: String,
    pub expires_at: i64,
    pub remaining_days: u32,
    pub last_verified_at: i64,
    pub is_offline_valid: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MemberEntitlementStateDto {
    Active,
    Expired,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberCapabilitiesDto {
    pub moderation: MemberCapabilityStateDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberStatusDto {
    pub shell_state: MemberShellStateDto,
    pub account: Option<MemberAccountDto>,
    pub entitlement: Option<MemberEntitlementDto>,
    pub capabilities: MemberCapabilitiesDto,
    pub service_error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "state",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum AuthQrStatusDto {
    Pending {
        session_id: String,
        image_base64: String,
        expires_in_seconds: u16,
    },
    Scanned {
        session_id: String,
        image_base64: String,
        expires_in_seconds: u16,
    },
    Success {
        session_id: String,
        account: MemberAccountDto,
    },
    Expired {
        session_id: String,
    },
    Error {
        session_id: String,
        error_code: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BooleanResultDto {
    pub cancelled: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MembershipOrderStatusDto {
    Pending,
    Paid,
    Cancelled,
    Failed,
    TimedOut,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MembershipOrderDto {
    pub order_id: String,
    pub plan_id: String,
    pub plan_label: String,
    pub amount_minor: u64,
    pub currency: String,
    pub status: MembershipOrderStatusDto,
    pub payment_qr_base64: Option<String>,
    pub payment_url: Option<String>,
    pub created_at: i64,
    pub paid_at: Option<i64>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MemberModuleStatusDto {
    Missing,
    Downloading,
    Verifying,
    Installed,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberModuleDto {
    pub module_id: String,
    pub name: String,
    pub version: String,
    pub status: MemberModuleStatusDto,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub progress_percent: u8,
    pub error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModuleOperationDto {
    pub operation_id: String,
    pub modules: Vec<MemberModuleDto>,
    pub total_progress_percent: u8,
    pub is_terminal: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModuleCleanupResultDto {
    pub removed: bool,
    pub modules: Vec<MemberModuleDto>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiSpeakingStyleDto {
    Concise,
    Natural,
    Detailed,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiConfigDto {
    pub identity: String,
    pub speaking_style: AiSpeakingStyleDto,
    pub system_prompt: String,
    pub max_length: u32,
    pub max_sentences: u32,
    pub is_sensitive_filter_enabled: bool,
    pub banned_words: Vec<String>,
    pub danmaku_context_count: u32,
    pub manual_context_count: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiConfigSaveRequestDto {
    pub config: AiConfigWritableDto,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiConfigWritableDto {
    pub identity: String,
    pub speaking_style: AiSpeakingStyleDto,
    pub system_prompt: String,
    pub max_length: u32,
    pub max_sentences: u32,
    pub is_sensitive_filter_enabled: bool,
    pub banned_words: Vec<String>,
    pub danmaku_context_count: u32,
    pub asr_context_count: u32,
    pub manual_context_count: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiProviderKindDto {
    Cloud,
    Aggregator,
    Local,
    CustomOpenAi,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiModelConnectionStateDto {
    Untested,
    Available,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiReasoningEffortDto {
    None,
    Minimal,
    Low,
    Medium,
    High,
    #[serde(rename = "xhigh")]
    XHigh,
    Max,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiModelProfileDto {
    pub profile_id: String,
    pub name: String,
    pub provider_kind: AiProviderKindDto,
    pub base_url: String,
    pub model: String,
    pub reasoning_effort: AiReasoningEffortDto,
    pub max_tokens: u32,
    pub timeout_seconds: u32,
    pub proxy_url: Option<String>,
    pub is_enabled: bool,
    pub is_default: bool,
    pub connection_state: AiModelConnectionStateDto,
    pub is_api_key_configured: bool,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiModelProfileWritableDto {
    pub profile_id: Option<String>,
    pub name: String,
    pub provider_kind: AiProviderKindDto,
    pub base_url: String,
    pub model: String,
    pub reasoning_effort: AiReasoningEffortDto,
    pub max_tokens: u32,
    pub timeout_seconds: u32,
    pub proxy_url: Option<String>,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiModelProfileSaveRequestDto {
    pub profile: AiModelProfileWritableDto,
    pub api_key_replacement: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiProfileIdRequestDto {
    pub profile_id: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiTriggerScopeDto {
    All,
    Questions,
    Captains,
    Keywords,
    HighValue,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiVoiceReplyRulesDto {
    pub trigger_scope: AiTriggerScopeDto,
    pub keywords: Vec<String>,
    pub global_cooldown_seconds: u32,
    pub user_cooldown_seconds: u32,
    pub queue_capacity: u32,
    pub is_duplicate_filter_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiOutputDeviceDto {
    pub device_id: String,
    pub display_name: String,
    pub is_available: bool,
    pub is_default: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiVoiceDto {
    pub voice_id: String,
    pub display_name: String,
    pub is_available: bool,
    pub is_default: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiVoiceQueueItemDto {
    pub item_id: String,
    pub text: String,
    pub priority: u8,
    pub created_at: i64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiVoiceQueueStateDto {
    Idle,
    Speaking,
    Paused,
    Unavailable,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiVoiceQueueSnapshotDto {
    pub state: AiVoiceQueueStateDto,
    pub current: Option<AiVoiceQueueItemDto>,
    pub pending: Vec<AiVoiceQueueItemDto>,
    pub capacity: u32,
    pub volume: f64,
    pub selected_device_id: Option<String>,
    pub selected_voice_id: String,
    pub is_audio_route_confirmed: bool,
    pub last_error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiVoiceReplySnapshotDto {
    pub is_enabled: bool,
    pub default_profile_id: Option<String>,
    pub rules: AiVoiceReplyRulesDto,
    pub devices: Vec<AiOutputDeviceDto>,
    pub voices: Vec<AiVoiceDto>,
    pub queue: AiVoiceQueueSnapshotDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiVoiceEnabledRequestDto {
    pub is_enabled: bool,
    pub is_audio_route_confirmed: bool,
    /// PRD 12.2: the first enable has to acknowledge that danmaku text and the
    /// audience nickname are sent to the configured model endpoint.
    #[serde(default)]
    pub is_data_disclosure_acknowledged: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiVoiceRulesRequestDto {
    pub rules: AiVoiceReplyRulesDto,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiOutputDeviceRequestDto {
    pub device_id: String,
    pub voice_id: String,
    pub volume: f64,
    pub is_audio_route_confirmed: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiTtsTestRequestDto {
    pub device_id: String,
    pub voice_id: String,
    pub volume: f64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiTtsPauseRequestDto {
    pub is_paused: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeletedResultDto {
    pub deleted: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiContextSourceDto {
    Danmaku,
    Manual,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiContextItemDto {
    pub id: String,
    pub source: AiContextSourceDto,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiGenerateRequestDto {
    pub prompt: String,
    pub context: Vec<AiContextItemDto>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiGenerationStatusDto {
    Assembling,
    Streaming,
    Completed,
    Failed,
    Rejected,
    Cancelled,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiGenerationDto {
    pub generation_id: String,
    pub status: AiGenerationStatusDto,
    pub result: String,
    pub last_sequence: u64,
    pub error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiLogEntryDto {
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MuteDurationDto {
    #[serde(rename = "10m")]
    TenMinutes,
    #[serde(rename = "1h")]
    OneHour,
    Permanent,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModerationRulesDto {
    pub frequency_window_seconds: u32,
    pub frequency_threshold: u32,
    pub repeat_window_seconds: u32,
    pub repeat_threshold: u32,
    pub similarity_window_seconds: u32,
    pub similarity_threshold: u32,
    pub similarity_ratio: f64,
    pub default_mute_duration: MuteDurationDto,
    pub is_risk_confirmed: bool,
    pub is_auto_enabled: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ModerationAlertStatusDto {
    Pending,
    Executing,
    Muted,
    Ignored,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModerationAlertDto {
    pub alert_id: String,
    pub user_uid: String,
    pub username: String,
    pub violation_description: String,
    pub content_example: String,
    pub occurred_at: i64,
    pub status: ModerationAlertStatusDto,
    pub error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModerationSummaryDto {
    pub rules: ModerationRulesDto,
    pub alerts: Vec<ModerationAlertDto>,
    pub recognized_today: u64,
    pub auto_muted_today: u64,
    pub manual_muted_today: u64,
    pub is_credential_available: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModerationLogEntryDto {
    pub id: i64,
    pub user_uid: String,
    pub username: String,
    pub action: String,
    pub occurred_at: i64,
    pub is_false_positive: bool,
}

macro_rules! id_request {
    ($name:ident, $field:ident) => {
        #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct $name {
            pub $field: String,
        }
    };
}

id_request!(AuthCancelRequestDto, session_id);
id_request!(UidRequestDto, user_uid);
id_request!(PurchaseRequestDto, plan_id);
id_request!(OrderRequestDto, order_id);
id_request!(ModuleRequestDto, module_id);
id_request!(RegenerateRequestDto, generation_id);
id_request!(ModerationIgnoreRequestDto, alert_id);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BindAccountRequestDto {
    pub account: MemberAccountDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetAutoRequestDto {
    pub is_enabled: bool,
    pub is_risk_accepted: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateRulesRequestDto {
    pub rules: ModerationRulesDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MuteRequestDto {
    pub alert_id: String,
    pub duration: MuteDurationDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogQueryRequestDto {
    pub user_uid: Option<String>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FalsePositiveRequestDto {
    pub log_id: i64,
}

pub struct TauriMemberEventSink {
    app: AppHandle,
    previous_shell: std::sync::Mutex<Option<MemberShellState>>,
}

impl TauriMemberEventSink {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            previous_shell: std::sync::Mutex::new(None),
        }
    }

    fn emit<T: Serialize + Clone>(&self, event: &str, payload: T) {
        if self
            .app
            .emit_to(EventTarget::webview_window("main"), event, payload)
            .is_err()
        {
            log::warn!("member event delivery failed: {event}");
        }
    }
}

impl MemberEventSink for TauriMemberEventSink {
    fn publish_status(&self, snapshot: &MemberSnapshot) {
        let Ok(mut previous) = self.previous_shell.lock() else {
            return;
        };
        let event = match (*previous, snapshot.shell_state) {
            (Some(MemberShellState::Active), MemberShellState::Expired) => {
                Some("membership_expired")
            }
            (Some(MemberShellState::Expired), MemberShellState::Active) => {
                Some("membership_renewed")
            }
            _ => None,
        };
        *previous = Some(snapshot.shell_state);
        if let Some(event) = event {
            self.emit(event, member_status_dto(snapshot.clone(), now_millis()));
        }
    }

    fn publish_ai_generation(&self, generation: &AiGenerationRecord) {
        self.emit(
            "ai_generation_finished",
            ai_generation_dto(generation.clone()),
        );
    }

    fn publish_moderation_alert(&self, alert: &ModerationAlertRecord) {
        self.emit("moderation_alert", moderation_alert_dto(alert.clone()));
    }
}

#[tauri::command]
pub async fn member_status(state: State<'_, AppState>) -> Result<MemberStatusDto, DesktopError> {
    let snapshot = state
        .member()?
        .membership
        .status(now_millis(), false)
        .await?;
    Ok(member_status_dto(snapshot, now_millis()))
}

#[tauri::command]
pub async fn auth_qrcode_generate() -> Result<AuthQrStatusDto, DesktopError> {
    Err(MemberServiceError::unavailable("auth_qr_unavailable").into())
}

#[tauri::command]
pub async fn auth_qrcode_cancel(
    request: AuthCancelRequestDto,
) -> Result<BooleanResultDto, DesktopError> {
    if request.session_id.trim().is_empty() {
        return Err(MemberServiceError::invalid("auth_session_invalid").into());
    }
    Ok(BooleanResultDto { cancelled: false })
}

#[tauri::command]
pub async fn auth_query_uid(
    state: State<'_, AppState>,
    request: UidRequestDto,
) -> Result<MemberAccountDto, DesktopError> {
    state
        .member()?
        .membership
        .query_uid(&request.user_uid)
        .await
        .map(account_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn auth_bind_uid(
    state: State<'_, AppState>,
    request: BindAccountRequestDto,
) -> Result<MemberStatusDto, DesktopError> {
    let account = account_from_dto(request.account)?;
    let snapshot = state.member()?.membership.bind_account(account).await?;
    Ok(member_status_dto(snapshot, now_millis()))
}

#[tauri::command]
pub async fn auth_logout(state: State<'_, AppState>) -> Result<MemberStatusDto, DesktopError> {
    let snapshot = state.member()?.membership.logout().await?;
    Ok(member_status_dto(snapshot, now_millis()))
}

#[tauri::command]
pub async fn membership_purchase(
    state: State<'_, AppState>,
    request: PurchaseRequestDto,
) -> Result<MembershipOrderDto, DesktopError> {
    state
        .member()?
        .membership
        .purchase(&request.plan_id)
        .await
        .map(order_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn membership_cancel_order(
    state: State<'_, AppState>,
    request: OrderRequestDto,
) -> Result<MembershipOrderDto, DesktopError> {
    state
        .member()?
        .membership
        .cancel_order(&request.order_id)
        .await
        .map(order_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn membership_orders(
    state: State<'_, AppState>,
) -> Result<Vec<MembershipOrderDto>, DesktopError> {
    state
        .member()?
        .membership
        .orders()
        .await
        .map(|orders| orders.into_iter().map(order_dto).collect())
        .map_err(Into::into)
}

#[tauri::command]
pub async fn module_list(state: State<'_, AppState>) -> Result<Vec<MemberModuleDto>, DesktopError> {
    map_modules(state.member()?.membership.modules().await?)
}

#[tauri::command]
pub async fn module_update(
    _state: State<'_, AppState>,
    request: ModuleRequestDto,
) -> Result<ModuleOperationDto, DesktopError> {
    if request.module_id.trim().is_empty() {
        return Err(MemberServiceError::invalid("module_id_invalid").into());
    }
    Err(MemberServiceError::unavailable("module_manifest_unavailable").into())
}

#[tauri::command]
pub async fn module_cleanup(
    state: State<'_, AppState>,
    request: ModuleRequestDto,
) -> Result<ModuleCleanupResultDto, DesktopError> {
    let removed = state
        .member()?
        .membership
        .cleanup_module(&request.module_id)
        .await?;
    let modules = map_modules(state.member()?.membership.modules().await?)?;
    Ok(ModuleCleanupResultDto { removed, modules })
}

#[tauri::command]
pub async fn ai_config_get(state: State<'_, AppState>) -> Result<AiConfigDto, DesktopError> {
    state
        .ai()?
        .config()
        .await
        .map(ai_config_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ai_config_save(
    state: State<'_, AppState>,
    request: AiConfigSaveRequestDto,
) -> Result<AiConfigDto, DesktopError> {
    let current = state.ai()?.config().await?;
    let config = ai_config_from_dto(request.config, current.api_key_ciphertext);
    state
        .ai()?
        .save_config(config)
        .await
        .map(ai_config_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ai_model_profile_list(
    state: State<'_, AppState>,
) -> Result<Vec<AiModelProfileDto>, DesktopError> {
    Ok(state
        .ai()?
        .model_profiles()
        .await?
        .into_iter()
        .map(ai_model_profile_dto)
        .collect())
}

#[tauri::command]
pub async fn ai_model_profile_save(
    state: State<'_, AppState>,
    request: AiModelProfileSaveRequestDto,
) -> Result<AiModelProfileDto, DesktopError> {
    let profile = ai_model_profile_from_dto(request.profile)?;
    state
        .ai()?
        .save_model_profile(profile, request.api_key_replacement)
        .await
        .map(ai_model_profile_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ai_model_profile_delete(
    state: State<'_, AppState>,
    request: AiProfileIdRequestDto,
) -> Result<DeletedResultDto, DesktopError> {
    validate_profile_id(&request.profile_id)?;
    Ok(DeletedResultDto {
        deleted: state
            .ai()?
            .delete_model_profile(&request.profile_id)
            .await?,
    })
}

#[tauri::command]
pub async fn ai_model_set_default(
    state: State<'_, AppState>,
    request: AiProfileIdRequestDto,
) -> Result<Vec<AiModelProfileDto>, DesktopError> {
    validate_profile_id(&request.profile_id)?;
    Ok(state
        .ai()?
        .set_default_model_profile(&request.profile_id)
        .await?
        .into_iter()
        .map(ai_model_profile_dto)
        .collect())
}

#[tauri::command]
pub async fn ai_model_test(
    state: State<'_, AppState>,
    request: AiProfileIdRequestDto,
) -> Result<AiModelProfileDto, DesktopError> {
    validate_profile_id(&request.profile_id)?;
    state
        .ai()?
        .test_model_profile(&request.profile_id)
        .await
        .map(ai_model_profile_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ai_voice_reply_get(
    state: State<'_, AppState>,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    ai_voice_reply_snapshot(state.ai()?).await
}

#[tauri::command]
pub async fn ai_voice_reply_set_enabled(
    state: State<'_, AppState>,
    request: AiVoiceEnabledRequestDto,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    state
        .ai()?
        .set_voice_enabled(
            request.is_enabled,
            request.is_audio_route_confirmed,
            request.is_data_disclosure_acknowledged,
        )
        .await?;
    ai_voice_reply_snapshot(state.ai()?).await
}

#[tauri::command]
pub async fn ai_voice_reply_update_rules(
    state: State<'_, AppState>,
    request: AiVoiceRulesRequestDto,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    let current = state.ai()?.voice_config().await?;
    state
        .ai()?
        .update_voice_rules(ai_voice_config_from_rules(request.rules, current))
        .await?;
    ai_voice_reply_snapshot(state.ai()?).await
}

#[tauri::command]
pub async fn ai_tts_list_output_devices(
    state: State<'_, AppState>,
) -> Result<Vec<AiOutputDeviceDto>, DesktopError> {
    Ok(state
        .ai()?
        .output_devices()
        .await?
        .into_iter()
        .map(ai_output_device_dto)
        .collect())
}

#[tauri::command]
pub async fn ai_tts_set_output_device(
    state: State<'_, AppState>,
    request: AiOutputDeviceRequestDto,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    state
        .ai()?
        .set_output_device(
            request.device_id,
            request.voice_id,
            request.volume,
            request.is_audio_route_confirmed,
        )
        .await?;
    ai_voice_reply_snapshot(state.ai()?).await
}

#[tauri::command]
pub async fn ai_tts_test(
    state: State<'_, AppState>,
    request: AiTtsTestRequestDto,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    state
        .ai()?
        .test_output_device(&request.device_id, &request.voice_id, request.volume)
        .await?;
    ai_voice_reply_snapshot(state.ai()?).await
}

#[tauri::command]
pub async fn ai_tts_pause(
    state: State<'_, AppState>,
    request: AiTtsPauseRequestDto,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    state.ai()?.pause_voice(request.is_paused).await;
    ai_voice_reply_snapshot(state.ai()?).await
}

#[tauri::command]
pub async fn ai_tts_skip(
    state: State<'_, AppState>,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    state.ai()?.skip_voice().await?;
    ai_voice_reply_snapshot(state.ai()?).await
}

#[tauri::command]
pub async fn ai_tts_clear_queue(
    state: State<'_, AppState>,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    state.ai()?.clear_voice().await;
    ai_voice_reply_snapshot(state.ai()?).await
}

#[tauri::command]
pub async fn ai_generate(
    state: State<'_, AppState>,
    request: AiGenerateRequestDto,
) -> Result<AiGenerationDto, DesktopError> {
    state
        .ai()?
        .generate(
            request.prompt,
            request
                .context
                .into_iter()
                .map(|item| item.content)
                .collect(),
        )
        .await
        .map(ai_generation_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ai_regenerate(
    state: State<'_, AppState>,
    request: RegenerateRequestDto,
) -> Result<AiGenerationDto, DesktopError> {
    state
        .ai()?
        .regenerate(&request.generation_id)
        .await
        .map(ai_generation_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ai_log_query(
    state: State<'_, AppState>,
    request: LogQueryRequestDto,
) -> Result<Vec<AiLogEntryDto>, DesktopError> {
    validate_log_query(&request)?;
    let logs = state
        .ai()?
        .query_logs(
            request.user_uid.as_deref(),
            request.started_at,
            request.ended_at,
        )
        .await?;
    Ok(logs
        .into_iter()
        .map(|entry| AiLogEntryDto {
            id: entry.id,
            operation_type: entry.operation_type,
            username: entry.username,
            user_uid: entry.user_uid,
            original_content: entry.original_content,
            ai_content: entry.ai_content,
            generation_status: entry.generation_status,
            reject_reason: entry.reject_reason,
            room_id: entry.room_id,
            created_at: entry.created_at,
        })
        .collect())
}

#[tauri::command]
pub async fn moderate_status(
    state: State<'_, AppState>,
) -> Result<ModerationSummaryDto, DesktopError> {
    moderation_summary(state.member()?.as_ref()).await
}

#[tauri::command]
pub async fn moderate_set_auto(
    state: State<'_, AppState>,
    request: SetAutoRequestDto,
) -> Result<ModerationSummaryDto, DesktopError> {
    let capability = state
        .member()?
        .membership
        .status(now_millis(), false)
        .await?
        .capabilities
        .moderation;
    state
        .member()?
        .moderation
        .set_auto(request.is_enabled, request.is_risk_accepted, capability)
        .await?;
    moderation_summary(state.member()?.as_ref()).await
}

#[tauri::command]
pub async fn moderate_update_rules(
    state: State<'_, AppState>,
    request: UpdateRulesRequestDto,
) -> Result<ModerationSummaryDto, DesktopError> {
    let current = state.member()?.moderation.config().await?;
    state
        .member()?
        .moderation
        .update_rules(moderation_from_dto(request.rules, current.updated_at))
        .await?;
    moderation_summary(state.member()?.as_ref()).await
}

#[tauri::command]
pub async fn moderate_mute(
    state: State<'_, AppState>,
    request: MuteRequestDto,
) -> Result<ModerationAlertDto, DesktopError> {
    let capability = state
        .member()?
        .membership
        .status(now_millis(), false)
        .await?
        .capabilities
        .moderation;
    state
        .member()?
        .moderation
        .mute(
            &request.alert_id,
            mute_duration_name(request.duration),
            capability,
        )
        .await
        .map(moderation_alert_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn moderate_ignore(
    state: State<'_, AppState>,
    request: ModerationIgnoreRequestDto,
) -> Result<ModerationSummaryDto, DesktopError> {
    state.member()?.moderation.ignore(&request.alert_id).await?;
    moderation_summary(state.member()?.as_ref()).await
}

#[tauri::command]
pub async fn moderate_log_query(
    _state: State<'_, AppState>,
    request: LogQueryRequestDto,
) -> Result<Vec<ModerationLogEntryDto>, DesktopError> {
    validate_log_query(&request)?;
    Err(MemberServiceError::unavailable("moderation_log_unavailable").into())
}

#[tauri::command]
pub async fn moderate_mark_false_positive(
    _state: State<'_, AppState>,
    request: FalsePositiveRequestDto,
) -> Result<Vec<ModerationLogEntryDto>, DesktopError> {
    if request.log_id <= 0 {
        return Err(MemberServiceError::invalid("moderation_log_invalid").into());
    }
    Err(MemberServiceError::unavailable("moderation_log_unavailable").into())
}

fn member_status_dto(snapshot: MemberSnapshot, now: i64) -> MemberStatusDto {
    let validity = snapshot
        .entitlement
        .as_ref()
        .map(|entitlement| evaluate_entitlement(entitlement, now, false));
    MemberStatusDto {
        shell_state: match snapshot.shell_state {
            MemberShellState::Unavailable => MemberShellStateDto::Unavailable,
            MemberShellState::LoggedOut => MemberShellStateDto::LoggedOut,
            MemberShellState::Unpaid => MemberShellStateDto::Unpaid,
            MemberShellState::Active => MemberShellStateDto::Active,
            MemberShellState::Expired => MemberShellStateDto::Expired,
        },
        account: snapshot.account.map(account_dto),
        entitlement: snapshot
            .entitlement
            .map(|entitlement| MemberEntitlementDto {
                status: match entitlement.status {
                    EntitlementStatus::Active => MemberEntitlementStateDto::Active,
                    EntitlementStatus::Expired => MemberEntitlementStateDto::Expired,
                },
                plan_id: entitlement.plan_id,
                expires_at: entitlement.expires_at,
                remaining_days: entitlement
                    .expires_at
                    .saturating_sub(now)
                    .checked_div(86_400_000)
                    .and_then(|days| u32::try_from(days).ok())
                    .unwrap_or(0),
                last_verified_at: entitlement.last_verified_at,
                is_offline_valid: matches!(validity, Some(EntitlementValidity::ActiveOffline)),
            }),
        capabilities: MemberCapabilitiesDto {
            moderation: capability_dto(snapshot.capabilities.moderation),
        },
        service_error_code: snapshot.service_error_code,
    }
}

const fn capability_dto(value: AiCapabilityState) -> MemberCapabilityStateDto {
    match value {
        AiCapabilityState::Unavailable => MemberCapabilityStateDto::Unavailable,
        AiCapabilityState::MissingModule => MemberCapabilityStateDto::MissingModule,
        AiCapabilityState::Ready => MemberCapabilityStateDto::Ready,
        AiCapabilityState::Expired => MemberCapabilityStateDto::Expired,
    }
}

fn account_dto(account: MemberAccount) -> MemberAccountDto {
    MemberAccountDto {
        user_uid: account.user_uid.as_str().to_owned(),
        username: account.username,
        avatar_url: account.avatar_url,
        logged_in_at: account.logged_in_at,
    }
}

fn account_from_dto(account: MemberAccountDto) -> Result<MemberAccount, DesktopError> {
    let user_uid = UserUid::parse(&account.user_uid)
        .map_err(|_| MemberServiceError::invalid("auth_uid_invalid"))?;
    if account.username.trim().is_empty() || account.logged_in_at <= 0 {
        return Err(MemberServiceError::invalid("auth_account_invalid").into());
    }
    Ok(MemberAccount {
        user_uid,
        username: account.username,
        avatar_url: account.avatar_url,
        logged_in_at: account.logged_in_at,
    })
}

fn order_dto(order: OrderRecord) -> MembershipOrderDto {
    MembershipOrderDto {
        order_id: order.order_id.as_str().to_owned(),
        plan_id: order.plan_id,
        plan_label: order.plan_label,
        amount_minor: order.amount_minor,
        currency: order.currency,
        status: match order.status {
            OrderStatus::Pending => MembershipOrderStatusDto::Pending,
            OrderStatus::Paid => MembershipOrderStatusDto::Paid,
            OrderStatus::Cancelled => MembershipOrderStatusDto::Cancelled,
            OrderStatus::Failed => MembershipOrderStatusDto::Failed,
            OrderStatus::TimedOut => MembershipOrderStatusDto::TimedOut,
        },
        payment_qr_base64: None,
        payment_url: None,
        created_at: order.created_at,
        paid_at: order.paid_at,
    }
}

fn map_modules(modules: Vec<MemberModule>) -> Result<Vec<MemberModuleDto>, DesktopError> {
    modules
        .into_iter()
        .map(|module| {
            Ok(MemberModuleDto {
                module_id: module.module_id.as_str().to_owned(),
                name: module.name,
                version: module.version,
                status: match module.status {
                    MemberModuleStatus::Missing => MemberModuleStatusDto::Missing,
                    MemberModuleStatus::Downloading => MemberModuleStatusDto::Downloading,
                    MemberModuleStatus::Verifying => MemberModuleStatusDto::Verifying,
                    MemberModuleStatus::Installed => MemberModuleStatusDto::Installed,
                    MemberModuleStatus::Failed => MemberModuleStatusDto::Failed,
                },
                progress_percent: module_progress(module.downloaded_bytes, module.total_bytes)
                    .map_err(|_| MemberServiceError::invalid("module_progress_invalid"))?,
                downloaded_bytes: module.downloaded_bytes,
                total_bytes: module.total_bytes,
                error_code: module.error_code,
            })
        })
        .collect()
}

fn ai_config_dto(config: AiConfigRecord) -> AiConfigDto {
    AiConfigDto {
        identity: config.identity,
        speaking_style: speaking_style_dto(&config.speaking_style),
        system_prompt: config.system_prompt,
        max_length: config.max_length,
        max_sentences: config.max_sentences,
        is_sensitive_filter_enabled: config.is_sensitive_filter_enabled,
        banned_words: config.banned_words,
        danmaku_context_count: config.danmaku_context_count,
        manual_context_count: config.manual_context_count,
    }
}

fn ai_config_from_dto(
    config: AiConfigWritableDto,
    api_key_ciphertext: Option<Vec<u8>>,
) -> AiConfigRecord {
    AiConfigRecord {
        identity: config.identity,
        speaking_style: speaking_style_name(config.speaking_style).to_owned(),
        system_prompt: config.system_prompt,
        provider: String::new(),
        endpoint: None,
        model: String::new(),
        api_key_ciphertext,
        temperature: 0.7,
        max_length: config.max_length,
        max_sentences: config.max_sentences,
        is_sensitive_filter_enabled: config.is_sensitive_filter_enabled,
        banned_words: config.banned_words,
        danmaku_context_count: config.danmaku_context_count,
        manual_context_count: config.manual_context_count,
        updated_at: now_millis(),
    }
}

fn speaking_style_dto(value: &str) -> AiSpeakingStyleDto {
    match value {
        "short" => AiSpeakingStyleDto::Concise,
        "formal" => AiSpeakingStyleDto::Detailed,
        _ => AiSpeakingStyleDto::Natural,
    }
}

const fn speaking_style_name(value: AiSpeakingStyleDto) -> &'static str {
    match value {
        AiSpeakingStyleDto::Concise => "short",
        AiSpeakingStyleDto::Natural => "casual",
        AiSpeakingStyleDto::Detailed => "formal",
    }
}

fn ai_model_profile_dto(profile: AiModelProfileRecord) -> AiModelProfileDto {
    AiModelProfileDto {
        profile_id: profile.profile_id,
        name: profile.name,
        provider_kind: match profile.provider_kind.as_str() {
            "cloud" => AiProviderKindDto::Cloud,
            "aggregator" => AiProviderKindDto::Aggregator,
            "local" => AiProviderKindDto::Local,
            _ => AiProviderKindDto::CustomOpenAi,
        },
        base_url: profile.base_url,
        model: profile.model,
        reasoning_effort: match profile.reasoning_effort {
            AiReasoningEffort::None => AiReasoningEffortDto::None,
            AiReasoningEffort::Minimal => AiReasoningEffortDto::Minimal,
            AiReasoningEffort::Low => AiReasoningEffortDto::Low,
            AiReasoningEffort::Medium => AiReasoningEffortDto::Medium,
            AiReasoningEffort::High => AiReasoningEffortDto::High,
            AiReasoningEffort::XHigh => AiReasoningEffortDto::XHigh,
            AiReasoningEffort::Max => AiReasoningEffortDto::Max,
        },
        max_tokens: profile.max_tokens,
        timeout_seconds: profile.timeout_seconds,
        proxy_url: profile.proxy_url,
        is_enabled: profile.is_enabled,
        is_default: profile.is_default,
        connection_state: match profile.connection_state {
            AiModelConnectionState::Untested => AiModelConnectionStateDto::Untested,
            AiModelConnectionState::Available => AiModelConnectionStateDto::Available,
            AiModelConnectionState::Unavailable => AiModelConnectionStateDto::Unavailable,
        },
        is_api_key_configured: profile.has_api_key,
        updated_at: profile.updated_at,
    }
}

fn ai_model_profile_from_dto(
    profile: AiModelProfileWritableDto,
) -> Result<AiModelProfileRecord, DesktopError> {
    let profile_id = profile
        .profile_id
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    validate_profile_id(&profile_id)?;
    Ok(AiModelProfileRecord {
        profile_id,
        name: profile.name.trim().to_owned(),
        provider_kind: match profile.provider_kind {
            AiProviderKindDto::Cloud => "cloud",
            AiProviderKindDto::Aggregator => "aggregator",
            AiProviderKindDto::Local => "local",
            AiProviderKindDto::CustomOpenAi => "custom_openai",
        }
        .to_owned(),
        base_url: profile.base_url.trim().to_owned(),
        model: profile.model.trim().to_owned(),
        reasoning_effort: match profile.reasoning_effort {
            AiReasoningEffortDto::None => AiReasoningEffort::None,
            AiReasoningEffortDto::Minimal => AiReasoningEffort::Minimal,
            AiReasoningEffortDto::Low => AiReasoningEffort::Low,
            AiReasoningEffortDto::Medium => AiReasoningEffort::Medium,
            AiReasoningEffortDto::High => AiReasoningEffort::High,
            AiReasoningEffortDto::XHigh => AiReasoningEffort::XHigh,
            AiReasoningEffortDto::Max => AiReasoningEffort::Max,
        },
        max_tokens: profile.max_tokens,
        timeout_seconds: profile.timeout_seconds,
        proxy_url: profile
            .proxy_url
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty()),
        is_enabled: profile.is_enabled,
        is_default: false,
        connection_state: AiModelConnectionState::Untested,
        has_api_key: false,
        updated_at: now_millis(),
    })
}

fn validate_profile_id(profile_id: &str) -> Result<(), DesktopError> {
    uuid::Uuid::parse_str(profile_id)
        .map(|_| ())
        .map_err(|_| MemberServiceError::invalid("ai_model_profile_invalid").into())
}

fn ai_voice_rules_dto(config: &AiVoiceConfigRecord) -> AiVoiceReplyRulesDto {
    AiVoiceReplyRulesDto {
        trigger_scope: match config.trigger_scope.as_str() {
            "all" => AiTriggerScopeDto::All,
            "captains" => AiTriggerScopeDto::Captains,
            "keywords" => AiTriggerScopeDto::Keywords,
            "high_value" => AiTriggerScopeDto::HighValue,
            _ => AiTriggerScopeDto::Questions,
        },
        keywords: config.keywords.clone(),
        global_cooldown_seconds: config.global_cooldown_seconds,
        user_cooldown_seconds: config.user_cooldown_seconds,
        queue_capacity: config.queue_capacity,
        is_duplicate_filter_enabled: config.is_duplicate_filter_enabled,
    }
}

fn ai_voice_config_from_rules(
    rules: AiVoiceReplyRulesDto,
    current: AiVoiceConfigRecord,
) -> AiVoiceConfigRecord {
    AiVoiceConfigRecord {
        trigger_scope: match rules.trigger_scope {
            AiTriggerScopeDto::All => "all",
            AiTriggerScopeDto::Questions => "questions",
            AiTriggerScopeDto::Captains => "captains",
            AiTriggerScopeDto::Keywords => "keywords",
            AiTriggerScopeDto::HighValue => "high_value",
        }
        .to_owned(),
        keywords: rules
            .keywords
            .into_iter()
            .map(|keyword| keyword.trim().to_owned())
            .filter(|keyword| !keyword.is_empty())
            .collect(),
        global_cooldown_seconds: rules.global_cooldown_seconds,
        user_cooldown_seconds: rules.user_cooldown_seconds,
        queue_capacity: rules.queue_capacity,
        is_duplicate_filter_enabled: rules.is_duplicate_filter_enabled,
        ..current
    }
}

fn ai_output_device_dto(device: AiOutputDeviceRecord) -> AiOutputDeviceDto {
    AiOutputDeviceDto {
        device_id: device.device_id,
        display_name: device.display_name,
        is_available: device.is_available,
        is_default: device.is_default,
    }
}

fn ai_voice_dto(voice: AiVoiceRecord) -> AiVoiceDto {
    AiVoiceDto {
        voice_id: voice.voice_id,
        display_name: voice.display_name,
        is_available: voice.is_available,
        is_default: voice.is_default,
    }
}

fn ai_voice_queue_item_dto(item: AiVoiceQueueItemRecord) -> AiVoiceQueueItemDto {
    AiVoiceQueueItemDto {
        item_id: item.item_id,
        text: item.text,
        priority: item.priority,
        created_at: item.created_at,
    }
}

fn ai_voice_queue_dto(queue: AiVoiceQueueSnapshotRecord) -> AiVoiceQueueSnapshotDto {
    AiVoiceQueueSnapshotDto {
        state: match queue.state {
            AiVoiceQueueState::Idle => AiVoiceQueueStateDto::Idle,
            AiVoiceQueueState::Speaking => AiVoiceQueueStateDto::Speaking,
            AiVoiceQueueState::Paused => AiVoiceQueueStateDto::Paused,
            AiVoiceQueueState::Unavailable => AiVoiceQueueStateDto::Unavailable,
        },
        current: queue.current.map(ai_voice_queue_item_dto),
        pending: queue
            .pending
            .into_iter()
            .map(ai_voice_queue_item_dto)
            .collect(),
        capacity: queue.capacity,
        volume: queue.volume,
        selected_device_id: queue.selected_device_id,
        selected_voice_id: queue.selected_voice_id,
        is_audio_route_confirmed: queue.is_audio_route_confirmed,
        last_error_code: queue.last_error_code,
    }
}

async fn ai_voice_reply_snapshot(
    state: &crate::application::member::AiState,
) -> Result<AiVoiceReplySnapshotDto, DesktopError> {
    let config = state.voice_config().await?;
    let profiles = state.model_profiles().await?;
    let devices = state.output_devices().await.unwrap_or_default();
    let voices = state.voices().await.unwrap_or_default();
    Ok(AiVoiceReplySnapshotDto {
        is_enabled: config.is_enabled,
        default_profile_id: profiles
            .iter()
            .find(|profile| profile.is_default)
            .map(|profile| profile.profile_id.clone()),
        rules: ai_voice_rules_dto(&config),
        devices: devices.into_iter().map(ai_output_device_dto).collect(),
        voices: voices.into_iter().map(ai_voice_dto).collect(),
        queue: ai_voice_queue_dto(state.voice_queue_snapshot().await?),
    })
}

fn ai_generation_dto(generation: AiGenerationRecord) -> AiGenerationDto {
    AiGenerationDto {
        generation_id: generation.generation_id,
        status: match generation.status {
            AiGenerationStatus::Assembling => AiGenerationStatusDto::Assembling,
            AiGenerationStatus::Streaming => AiGenerationStatusDto::Streaming,
            AiGenerationStatus::Completed => AiGenerationStatusDto::Completed,
            AiGenerationStatus::Failed => AiGenerationStatusDto::Failed,
            AiGenerationStatus::Rejected => AiGenerationStatusDto::Rejected,
            AiGenerationStatus::Cancelled => AiGenerationStatusDto::Cancelled,
        },
        result: generation.result,
        last_sequence: generation.last_sequence,
        error_code: generation.error_code,
    }
}

async fn moderation_summary(
    states: &crate::application::member::MemberStates,
) -> Result<ModerationSummaryDto, DesktopError> {
    let config = states.moderation.config().await?;
    let alerts = states.moderation.alerts().await;
    Ok(ModerationSummaryDto {
        rules: moderation_rules_dto(config),
        recognized_today: alerts.len() as u64,
        auto_muted_today: 0,
        manual_muted_today: alerts
            .iter()
            .filter(|alert| alert.status == ModerationAlertStatus::Muted)
            .count() as u64,
        alerts: alerts.into_iter().map(moderation_alert_dto).collect(),
        is_credential_available: false,
    })
}

fn moderation_rules_dto(config: ModerationConfigRecord) -> ModerationRulesDto {
    ModerationRulesDto {
        frequency_window_seconds: config.frequency_window_seconds,
        frequency_threshold: config.frequency_threshold,
        repeat_window_seconds: config.repeat_window_seconds,
        repeat_threshold: config.repeat_threshold,
        similarity_window_seconds: config.similarity_window_seconds,
        similarity_threshold: config.similarity_threshold,
        similarity_ratio: config.similarity_ratio,
        default_mute_duration: mute_duration_dto(&config.default_mute_duration),
        is_risk_confirmed: config.is_risk_confirmed,
        is_auto_enabled: config.is_auto_enabled,
    }
}

fn moderation_from_dto(rules: ModerationRulesDto, updated_at: i64) -> ModerationConfigRecord {
    ModerationConfigRecord {
        is_auto_enabled: rules.is_auto_enabled,
        default_mute_duration: mute_duration_name(rules.default_mute_duration).to_owned(),
        frequency_window_seconds: rules.frequency_window_seconds,
        frequency_threshold: rules.frequency_threshold,
        repeat_window_seconds: rules.repeat_window_seconds,
        repeat_threshold: rules.repeat_threshold,
        similarity_window_seconds: rules.similarity_window_seconds,
        similarity_threshold: rules.similarity_threshold,
        similarity_ratio: rules.similarity_ratio,
        is_risk_confirmed: rules.is_risk_confirmed,
        updated_at,
    }
}

fn moderation_alert_dto(alert: ModerationAlertRecord) -> ModerationAlertDto {
    ModerationAlertDto {
        alert_id: alert.alert_id,
        user_uid: alert.user_uid.as_str().to_owned(),
        username: alert.username,
        violation_description: alert.violation_description,
        content_example: alert.content_example,
        occurred_at: alert.occurred_at,
        status: match alert.status {
            ModerationAlertStatus::Pending => ModerationAlertStatusDto::Pending,
            ModerationAlertStatus::Executing => ModerationAlertStatusDto::Executing,
            ModerationAlertStatus::Muted => ModerationAlertStatusDto::Muted,
            ModerationAlertStatus::Ignored => ModerationAlertStatusDto::Ignored,
            ModerationAlertStatus::Failed => ModerationAlertStatusDto::Failed,
        },
        error_code: alert.error_code,
    }
}

const fn mute_duration_name(value: MuteDurationDto) -> &'static str {
    match value {
        MuteDurationDto::TenMinutes => "10m",
        MuteDurationDto::OneHour => "1h",
        MuteDurationDto::Permanent => "permanent",
    }
}

fn mute_duration_dto(value: &str) -> MuteDurationDto {
    match value {
        "10m" => MuteDurationDto::TenMinutes,
        "permanent" => MuteDurationDto::Permanent,
        _ => MuteDurationDto::OneHour,
    }
}

fn validate_log_query(request: &LogQueryRequestDto) -> Result<(), DesktopError> {
    if request.started_at.is_some_and(|value| value <= 0)
        || request.ended_at.is_some_and(|value| value <= 0)
        || matches!((request.started_at, request.ended_at), (Some(start), Some(end)) if start > end)
    {
        return Err(MemberServiceError::invalid("member_log_range_invalid").into());
    }
    if let Some(uid) = &request.user_uid {
        UserUid::parse(uid).map_err(|_| MemberServiceError::invalid("auth_uid_invalid"))?;
    }
    Ok(())
}
