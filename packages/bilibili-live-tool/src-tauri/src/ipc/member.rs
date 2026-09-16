use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, EventTarget, State};

use crate::{
    application::member::{
        now_millis, AiConfigRecord, AiGenerationRecord, AiGenerationStatus, AsrState,
        AsrStatusRecord, MemberEventSink, MemberServiceError, ModerationAlertRecord,
        ModerationAlertStatus, ModerationConfigRecord, OrderRecord, OrderStatus,
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
    "ai_generation_delta",
    "ai_generation_finished",
    "asr_status_changed",
    "moderation_alert",
    "moderation_action_result",
];

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
    pub ai: MemberCapabilityStateDto,
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
    Short,
    Humorous,
    Formal,
    Casual,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiConfigDto {
    pub identity: String,
    pub speaking_style: AiSpeakingStyleDto,
    pub system_prompt: String,
    pub provider: String,
    pub model: String,
    pub temperature: f64,
    pub max_length: u32,
    pub is_sensitive_filter_enabled: bool,
    pub banned_words: Vec<String>,
    pub danmaku_context_count: u32,
    pub asr_context_count: u32,
    pub manual_context_count: u32,
    pub is_api_key_configured: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiConfigSaveRequestDto {
    pub config: AiConfigWritableDto,
    pub api_key_replacement: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AiConfigWritableDto {
    pub identity: String,
    pub speaking_style: AiSpeakingStyleDto,
    pub system_prompt: String,
    pub provider: String,
    pub model: String,
    pub temperature: f64,
    pub max_length: u32,
    pub is_sensitive_filter_enabled: bool,
    pub banned_words: Vec<String>,
    pub danmaku_context_count: u32,
    pub asr_context_count: u32,
    pub manual_context_count: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AiContextSourceDto {
    Danmaku,
    Asr,
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
pub enum AsrStateDto {
    Unavailable,
    ModelMissing,
    Downloading,
    Ready,
    Starting,
    Listening,
    Error,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AsrStatusDto {
    pub state: AsrStateDto,
    pub model_name: Option<String>,
    pub language: String,
    pub progress_percent: u8,
    pub error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AsrModelManifestDto {
    pub model_name: String,
    pub size_bytes: u64,
    pub license: String,
    pub source_label: String,
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
pub struct AsrDownloadRequestDto {
    pub manifest: AsrModelManifestDto,
    pub is_license_accepted: bool,
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

    fn publish_asr_status(&self, status: &AsrStatusRecord) {
        self.emit("asr_status_changed", asr_status_dto(status.clone()));
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
        .member()?
        .ai
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
    if request.api_key_replacement.is_some() {
        return Err(MemberServiceError::unavailable("secret_store_unavailable").into());
    }
    let current = state.member()?.ai.config().await?;
    let config = ai_config_from_dto(request.config, current.api_key_ciphertext);
    state
        .member()?
        .ai
        .save_config(config)
        .await
        .map(ai_config_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ai_generate(
    state: State<'_, AppState>,
    request: AiGenerateRequestDto,
) -> Result<AiGenerationDto, DesktopError> {
    let capability = state
        .member()?
        .membership
        .status(now_millis(), false)
        .await?
        .capabilities
        .ai;
    state
        .member()?
        .ai
        .generate(
            request.prompt,
            request
                .context
                .into_iter()
                .map(|item| item.content)
                .collect(),
            capability,
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
    let capability = state
        .member()?
        .membership
        .status(now_millis(), false)
        .await?
        .capabilities
        .ai;
    state
        .member()?
        .ai
        .regenerate(&request.generation_id, capability)
        .await
        .map(ai_generation_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ai_log_query(
    _state: State<'_, AppState>,
    request: LogQueryRequestDto,
) -> Result<Vec<AiLogEntryDto>, DesktopError> {
    validate_log_query(&request)?;
    Err(MemberServiceError::unavailable("ai_log_unavailable").into())
}

#[tauri::command]
pub async fn asr_status(state: State<'_, AppState>) -> Result<AsrStatusDto, DesktopError> {
    Ok(asr_status_dto(state.member()?.ai.asr_status().await))
}

#[tauri::command]
pub async fn asr_model_download(
    _state: State<'_, AppState>,
    request: AsrDownloadRequestDto,
) -> Result<AsrStatusDto, DesktopError> {
    if !request.is_license_accepted {
        return Err(MemberServiceError::invalid("asr_license_required").into());
    }
    if request.manifest.model_name.trim().is_empty() || request.manifest.size_bytes == 0 {
        return Err(MemberServiceError::invalid("asr_manifest_invalid").into());
    }
    Err(MemberServiceError::unavailable("asr_manifest_unavailable").into())
}

#[tauri::command]
pub async fn asr_start(state: State<'_, AppState>) -> Result<AsrStatusDto, DesktopError> {
    let capability = state
        .member()?
        .membership
        .status(now_millis(), false)
        .await?
        .capabilities
        .ai;
    state
        .member()?
        .ai
        .start_asr(capability)
        .await
        .map(asr_status_dto)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn asr_stop(state: State<'_, AppState>) -> Result<AsrStatusDto, DesktopError> {
    state
        .member()?
        .ai
        .stop_asr()
        .await
        .map(asr_status_dto)
        .map_err(Into::into)
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
            ai: capability_dto(snapshot.capabilities.ai),
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
        provider: config.provider,
        model: config.model,
        temperature: config.temperature,
        max_length: config.max_length,
        is_sensitive_filter_enabled: config.is_sensitive_filter_enabled,
        banned_words: config.banned_words,
        danmaku_context_count: config.danmaku_context_count,
        asr_context_count: config.asr_context_count,
        manual_context_count: config.manual_context_count,
        is_api_key_configured: config.api_key_ciphertext.is_some(),
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
        provider: config.provider,
        endpoint: None,
        model: config.model,
        api_key_ciphertext,
        temperature: config.temperature,
        max_length: config.max_length,
        is_sensitive_filter_enabled: config.is_sensitive_filter_enabled,
        banned_words: config.banned_words,
        danmaku_context_count: config.danmaku_context_count,
        asr_context_count: config.asr_context_count,
        manual_context_count: config.manual_context_count,
        updated_at: now_millis(),
    }
}

fn speaking_style_dto(value: &str) -> AiSpeakingStyleDto {
    match value {
        "humorous" => AiSpeakingStyleDto::Humorous,
        "formal" => AiSpeakingStyleDto::Formal,
        "casual" => AiSpeakingStyleDto::Casual,
        _ => AiSpeakingStyleDto::Short,
    }
}

const fn speaking_style_name(value: AiSpeakingStyleDto) -> &'static str {
    match value {
        AiSpeakingStyleDto::Short => "short",
        AiSpeakingStyleDto::Humorous => "humorous",
        AiSpeakingStyleDto::Formal => "formal",
        AiSpeakingStyleDto::Casual => "casual",
    }
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

fn asr_status_dto(status: AsrStatusRecord) -> AsrStatusDto {
    AsrStatusDto {
        state: match status.state {
            AsrState::Unavailable => AsrStateDto::Unavailable,
            AsrState::ModelMissing => AsrStateDto::ModelMissing,
            AsrState::Downloading => AsrStateDto::Downloading,
            AsrState::Ready => AsrStateDto::Ready,
            AsrState::Starting => AsrStateDto::Starting,
            AsrState::Listening => AsrStateDto::Listening,
            AsrState::Error => AsrStateDto::Error,
        },
        model_name: status.model_name,
        language: status.language,
        progress_percent: status.progress_percent,
        error_code: status.error_code,
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
