use std::{
    collections::{HashMap, VecDeque},
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::sync::Mutex;

use crate::{
    application::{
        live_data::{AcceptedLiveEventConsumer, LiveRuntimeError},
        member::{
            AiConfigRecord, AiGenerationRecord, AiGenerationStatus, AsrProvider, AsrState,
            AsrStatusRecord, AuthProvider, EntitlementVerifier, LlmProvider, MemberEventSink,
            MemberRepository, MemberServiceError, MembershipProvider, ModerationAlertRecord,
            ModerationAlertStatus, ModerationConfigRecord, ModerationProvider, NoopMemberEventSink,
            OrderRecord,
        },
    },
    domain::{
        live_data::LiveEvent,
        member::{
            derive_capability, evaluate_entitlement, filter_and_truncate, moderation_defaults,
            AiCapabilityState, EntitlementValidity, MemberAccount, MemberCapabilities,
            MemberModule, MemberModuleStatus, MemberShellState, MemberSnapshot, ModuleId, OrderId,
            DEFAULT_LLM_TIMEOUT_MILLIS,
        },
    },
};

pub struct MembershipState {
    repository: Arc<dyn MemberRepository>,
    auth: Arc<dyn AuthProvider>,
    commerce: Arc<dyn MembershipProvider>,
    verifier: Arc<dyn EntitlementVerifier>,
    event_sink: Arc<dyn MemberEventSink>,
}

impl MembershipState {
    pub fn new(
        repository: Arc<dyn MemberRepository>,
        auth: Arc<dyn AuthProvider>,
        commerce: Arc<dyn MembershipProvider>,
        verifier: Arc<dyn EntitlementVerifier>,
    ) -> Self {
        Self {
            repository,
            auth,
            commerce,
            verifier,
            event_sink: Arc::new(NoopMemberEventSink),
        }
    }

    pub fn with_event_sink(mut self, event_sink: Arc<dyn MemberEventSink>) -> Self {
        self.event_sink = event_sink;
        self
    }

    pub async fn status(
        &self,
        now_millis: i64,
        is_online: bool,
    ) -> Result<MemberSnapshot, MemberServiceError> {
        let account = self.repository.load_account().await?;
        let Some(account) = account else {
            return Ok(MemberSnapshot {
                shell_state: MemberShellState::LoggedOut,
                account: None,
                entitlement: None,
                modules: Vec::new(),
                capabilities: unavailable_capabilities(),
                service_error_code: None,
            });
        };
        let modules = self.repository.list_modules().await?;
        let stored = self.repository.load_entitlement().await?;
        let Some(stored) = stored else {
            return Ok(MemberSnapshot {
                shell_state: MemberShellState::Unpaid,
                account: Some(account),
                entitlement: None,
                modules,
                capabilities: unavailable_capabilities(),
                service_error_code: None,
            });
        };

        if let Err(error) = self
            .verifier
            .verify(&stored.token_ciphertext, &stored.token_signature)
        {
            return Ok(MemberSnapshot {
                shell_state: MemberShellState::Unavailable,
                account: Some(account),
                entitlement: Some(stored.entitlement),
                modules,
                capabilities: unavailable_capabilities(),
                service_error_code: Some(error.code.to_owned()),
            });
        }

        let validity = evaluate_entitlement(&stored.entitlement, now_millis, is_online);
        let snapshot = MemberSnapshot {
            shell_state: shell_state(validity),
            account: Some(account),
            entitlement: Some(stored.entitlement),
            capabilities: MemberCapabilities {
                ai: derive_capability(validity, module_status(&modules, "ai")),
                moderation: derive_capability(validity, module_status(&modules, "moderation")),
            },
            modules,
            service_error_code: match validity {
                EntitlementValidity::VerificationRequired => {
                    Some("entitlement_online_verification_required".to_owned())
                }
                EntitlementValidity::InvalidClock => Some("entitlement_clock_invalid".to_owned()),
                _ => None,
            },
        };
        self.event_sink.publish_status(&snapshot);
        Ok(snapshot)
    }

    pub async fn query_uid(&self, uid: &str) -> Result<MemberAccount, MemberServiceError> {
        let uid = crate::domain::live_data::UserUid::parse(uid)
            .map_err(|_| MemberServiceError::invalid("auth_uid_invalid"))?;
        self.auth.query_uid(&uid).await
    }

    pub async fn bind_account(
        &self,
        account: MemberAccount,
    ) -> Result<MemberSnapshot, MemberServiceError> {
        self.repository.save_account(&account).await?;
        self.status(now_millis(), true).await
    }

    pub async fn logout(&self) -> Result<MemberSnapshot, MemberServiceError> {
        self.auth.clear_session().await?;
        self.repository.remove_account().await?;
        self.status(now_millis(), false).await
    }

    pub async fn purchase(&self, plan_id: &str) -> Result<OrderRecord, MemberServiceError> {
        if plan_id.trim().is_empty() {
            return Err(MemberServiceError::invalid("membership_plan_invalid"));
        }
        let order = self.commerce.purchase(plan_id).await?;
        self.repository.upsert_order(&order).await?;
        Ok(order)
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<OrderRecord, MemberServiceError> {
        let order_id = OrderId::parse(order_id)
            .map_err(|_| MemberServiceError::invalid("membership_order_invalid"))?;
        let order = self.commerce.cancel_order(&order_id).await?;
        self.repository.upsert_order(&order).await?;
        Ok(order)
    }

    pub async fn orders(&self) -> Result<Vec<OrderRecord>, MemberServiceError> {
        self.repository.list_orders().await
    }

    pub async fn modules(&self) -> Result<Vec<MemberModule>, MemberServiceError> {
        self.repository.list_modules().await
    }

    pub async fn cleanup_module(&self, module_id: &str) -> Result<bool, MemberServiceError> {
        ModuleId::parse(module_id).map_err(|_| MemberServiceError::invalid("module_id_invalid"))?;
        Err(MemberServiceError::unavailable(
            "module_cleanup_unavailable",
        ))
    }
}

#[derive(Default)]
struct AiRuntime {
    last_request: Option<(String, Vec<String>, String)>,
}

struct ActiveGenerationGuard<'a>(&'a AtomicBool);

impl Drop for ActiveGenerationGuard<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

pub struct AiState {
    repository: Arc<dyn MemberRepository>,
    llm: Arc<dyn LlmProvider>,
    asr: Arc<dyn AsrProvider>,
    event_sink: Arc<dyn MemberEventSink>,
    runtime: Mutex<AiRuntime>,
    active_generation: AtomicBool,
    asr_status: Mutex<AsrStatusRecord>,
}

impl AiState {
    pub fn new(
        repository: Arc<dyn MemberRepository>,
        llm: Arc<dyn LlmProvider>,
        asr: Arc<dyn AsrProvider>,
    ) -> Self {
        Self {
            repository,
            llm,
            asr,
            event_sink: Arc::new(NoopMemberEventSink),
            runtime: Mutex::new(AiRuntime::default()),
            active_generation: AtomicBool::new(false),
            asr_status: Mutex::new(AsrStatusRecord {
                state: AsrState::Unavailable,
                model_name: None,
                language: "zh".to_owned(),
                progress_percent: 0,
                error_code: Some("asr_unavailable".to_owned()),
            }),
        }
    }

    pub fn with_event_sink(mut self, event_sink: Arc<dyn MemberEventSink>) -> Self {
        self.event_sink = event_sink;
        self
    }

    pub async fn config(&self) -> Result<AiConfigRecord, MemberServiceError> {
        Ok(self
            .repository
            .load_ai_config()
            .await?
            .unwrap_or_else(default_ai_config))
    }

    pub async fn save_config(
        &self,
        config: AiConfigRecord,
    ) -> Result<AiConfigRecord, MemberServiceError> {
        self.repository.save_ai_config(&config).await?;
        Ok(config)
    }

    pub async fn generate(
        &self,
        prompt: String,
        context: Vec<String>,
        capability: AiCapabilityState,
    ) -> Result<AiGenerationRecord, MemberServiceError> {
        if capability != AiCapabilityState::Ready {
            return Err(MemberServiceError::invalid("ai_capability_unavailable"));
        }
        if prompt.trim().is_empty() && context.is_empty() {
            return Err(MemberServiceError::invalid("ai_input_required"));
        }
        let config = self.config().await?;
        if self
            .active_generation
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(MemberServiceError::invalid("ai_generation_pending"));
        }
        let _active = ActiveGenerationGuard(&self.active_generation);

        let generation_id = uuid::Uuid::new_v4().to_string();
        let assembled = context
            .iter()
            .chain(std::iter::once(&prompt))
            .filter(|value| !value.trim().is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");
        let generated = tokio::time::timeout(
            std::time::Duration::from_millis(u64::from(DEFAULT_LLM_TIMEOUT_MILLIS)),
            self.llm.generate(&assembled),
        )
        .await
        .map_err(|_| MemberServiceError::unavailable("ai_timeout"))
        .and_then(|result| result);
        let result = match generated {
            Ok(value) => filter_and_truncate(
                &value,
                if config.is_sensitive_filter_enabled {
                    config.banned_words.as_slice()
                } else {
                    &[]
                },
                config.max_length as usize,
            )
            .map_err(|error| match error {
                crate::domain::member::AiTextError::SensitiveTerm => {
                    MemberServiceError::invalid("ai_sensitive_output")
                }
                crate::domain::member::AiTextError::InvalidMaximumLength => {
                    MemberServiceError::invalid("ai_config_invalid")
                }
            }),
            Err(error) => Err(error),
        };

        match result {
            Ok(result) => {
                let mut runtime = self.runtime.lock().await;
                runtime.last_request = Some((prompt, context, generation_id.clone()));
                let generation = AiGenerationRecord {
                    generation_id,
                    status: AiGenerationStatus::Completed,
                    result,
                    last_sequence: 1,
                    error_code: None,
                };
                self.event_sink.publish_ai_generation(&generation);
                Ok(generation)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn regenerate(
        &self,
        generation_id: &str,
        capability: AiCapabilityState,
    ) -> Result<AiGenerationRecord, MemberServiceError> {
        let request = self
            .runtime
            .lock()
            .await
            .last_request
            .clone()
            .filter(|(_, _, id)| id == generation_id)
            .ok_or_else(|| MemberServiceError::invalid("ai_generation_not_found"))?;
        self.generate(request.0, request.1, capability).await
    }

    pub async fn asr_status(&self) -> AsrStatusRecord {
        self.asr_status.lock().await.clone()
    }

    pub async fn start_asr(
        &self,
        capability: AiCapabilityState,
    ) -> Result<AsrStatusRecord, MemberServiceError> {
        if capability != AiCapabilityState::Ready {
            return Err(MemberServiceError::invalid("asr_capability_unavailable"));
        }
        self.asr.start().await?;
        let status = AsrStatusRecord {
            state: AsrState::Listening,
            model_name: self.asr_status.lock().await.model_name.clone(),
            language: "zh".to_owned(),
            progress_percent: 100,
            error_code: None,
        };
        *self.asr_status.lock().await = status.clone();
        self.event_sink.publish_asr_status(&status);
        Ok(status)
    }

    pub async fn stop_asr(&self) -> Result<AsrStatusRecord, MemberServiceError> {
        self.asr.stop().await?;
        let mut status = self.asr_status.lock().await;
        status.state = if status.model_name.is_some() {
            AsrState::Ready
        } else {
            AsrState::ModelMissing
        };
        status.error_code = None;
        self.event_sink.publish_asr_status(&status);
        Ok(status.clone())
    }

    pub async fn shutdown(&self) {
        let _ = self.asr.stop().await;
        self.active_generation.store(false, Ordering::Release);
    }
}

struct UserWindow {
    entries: VecDeque<(i64, String)>,
}

pub struct ModerationState {
    repository: Arc<dyn MemberRepository>,
    provider: Arc<dyn ModerationProvider>,
    membership: Arc<MembershipState>,
    event_sink: Arc<dyn MemberEventSink>,
    alerts: Mutex<Vec<ModerationAlertRecord>>,
    windows: Mutex<HashMap<String, UserWindow>>,
}

impl ModerationState {
    pub fn new(
        repository: Arc<dyn MemberRepository>,
        provider: Arc<dyn ModerationProvider>,
        membership: Arc<MembershipState>,
    ) -> Self {
        Self {
            repository,
            provider,
            membership,
            event_sink: Arc::new(NoopMemberEventSink),
            alerts: Mutex::new(Vec::new()),
            windows: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_event_sink(mut self, event_sink: Arc<dyn MemberEventSink>) -> Self {
        self.event_sink = event_sink;
        self
    }

    pub async fn config(&self) -> Result<ModerationConfigRecord, MemberServiceError> {
        Ok(self
            .repository
            .load_moderation_config()
            .await?
            .unwrap_or_else(default_moderation_config))
    }

    pub async fn alerts(&self) -> Vec<ModerationAlertRecord> {
        self.alerts.lock().await.clone()
    }

    pub async fn set_auto(
        &self,
        is_enabled: bool,
        is_risk_accepted: bool,
        capability: AiCapabilityState,
    ) -> Result<ModerationConfigRecord, MemberServiceError> {
        if is_enabled && capability != AiCapabilityState::Ready {
            return Err(MemberServiceError::invalid(
                "moderation_capability_unavailable",
            ));
        }
        let mut config = self.config().await?;
        if is_enabled && !(config.is_risk_confirmed || is_risk_accepted) {
            return Err(MemberServiceError::invalid("moderation_risk_required"));
        }
        config.is_auto_enabled = is_enabled;
        config.is_risk_confirmed |= is_risk_accepted;
        config.updated_at = now_millis();
        self.repository.save_moderation_config(&config).await?;
        Ok(config)
    }

    pub async fn update_rules(
        &self,
        mut config: ModerationConfigRecord,
    ) -> Result<ModerationConfigRecord, MemberServiceError> {
        let current = self.config().await?;
        config.is_auto_enabled = current.is_auto_enabled;
        config.is_risk_confirmed = current.is_risk_confirmed;
        config.updated_at = now_millis();
        self.repository.save_moderation_config(&config).await?;
        Ok(config)
    }

    pub async fn mute(
        &self,
        alert_id: &str,
        duration: &str,
        capability: AiCapabilityState,
    ) -> Result<ModerationAlertRecord, MemberServiceError> {
        if capability != AiCapabilityState::Ready {
            return Err(MemberServiceError::invalid(
                "moderation_capability_unavailable",
            ));
        }
        if !matches!(duration, "10m" | "1h" | "permanent") {
            return Err(MemberServiceError::invalid("mute_duration_invalid"));
        }
        let alert = self
            .alerts
            .lock()
            .await
            .iter()
            .find(|alert| alert.alert_id == alert_id)
            .cloned()
            .ok_or_else(|| MemberServiceError::invalid("moderation_alert_not_found"))?;
        if self.provider.mute(&alert.user_uid, duration).await.is_err() {
            if let Err(error) = self.provider.mute(&alert.user_uid, duration).await {
                let mut alerts = self.alerts.lock().await;
                if let Some(target) = alerts
                    .iter_mut()
                    .find(|candidate| candidate.alert_id == alert_id)
                {
                    target.status = ModerationAlertStatus::Failed;
                    target.error_code = Some(error.code.to_owned());
                    self.event_sink.publish_moderation_alert(target);
                }
                return Err(error);
            }
        }
        let mut alerts = self.alerts.lock().await;
        let target = alerts
            .iter_mut()
            .find(|candidate| candidate.alert_id == alert_id)
            .ok_or_else(|| MemberServiceError::invalid("moderation_alert_not_found"))?;
        target.status = ModerationAlertStatus::Muted;
        target.error_code = None;
        self.event_sink.publish_moderation_alert(target);
        Ok(target.clone())
    }

    pub async fn ignore(
        &self,
        alert_id: &str,
    ) -> Result<Vec<ModerationAlertRecord>, MemberServiceError> {
        let mut alerts = self.alerts.lock().await;
        let target = alerts
            .iter_mut()
            .find(|alert| alert.alert_id == alert_id)
            .ok_or_else(|| MemberServiceError::invalid("moderation_alert_not_found"))?;
        target.status = ModerationAlertStatus::Ignored;
        Ok(alerts.clone())
    }

    async fn inspect(&self, event: &LiveEvent) -> Result<(), MemberServiceError> {
        let LiveEvent::Danmu { base, content } = event else {
            return Ok(());
        };
        let config = self.config().await?;
        let uid = base.audience.user_uid.as_str().to_owned();
        let mut windows = self.windows.lock().await;
        let window = windows.entry(uid).or_insert_with(|| UserWindow {
            entries: VecDeque::new(),
        });
        let maximum_window = i64::from(
            config
                .frequency_window_seconds
                .max(config.repeat_window_seconds)
                .max(config.similarity_window_seconds),
        ) * 1_000;
        while window
            .entries
            .front()
            .is_some_and(|(at, _)| base.occurred_at - *at > maximum_window)
        {
            window.entries.pop_front();
        }
        window
            .entries
            .push_back((base.occurred_at, content.clone()));
        let reason = detect_violation(window, &config, base.occurred_at, content);
        drop(windows);
        let Some(reason) = reason else {
            return Ok(());
        };
        let alert = ModerationAlertRecord {
            alert_id: uuid::Uuid::new_v4().to_string(),
            user_uid: base.audience.user_uid.clone(),
            username: base.audience.username.clone(),
            violation_description: reason,
            content_example: content.clone(),
            occurred_at: base.occurred_at,
            status: ModerationAlertStatus::Pending,
            error_code: None,
        };
        self.alerts.lock().await.push(alert.clone());
        self.event_sink.publish_moderation_alert(&alert);

        if config.is_auto_enabled {
            let capability = self
                .membership
                .status(base.occurred_at, false)
                .await?
                .capabilities
                .moderation;
            if capability == AiCapabilityState::Ready {
                let _ = self
                    .mute(&alert.alert_id, &config.default_mute_duration, capability)
                    .await;
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl AcceptedLiveEventConsumer for ModerationState {
    async fn consume_accepted(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError> {
        self.inspect(event).await.map_err(|error| {
            LiveRuntimeError::new(error.code, "member moderation failed", error.retryable)
        })
    }
}

pub struct MemberStates {
    pub membership: Arc<MembershipState>,
    pub ai: Arc<AiState>,
    pub moderation: Arc<ModerationState>,
    validation_task: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl MemberStates {
    pub fn new(
        membership: Arc<MembershipState>,
        ai: Arc<AiState>,
        moderation: Arc<ModerationState>,
    ) -> Self {
        Self {
            membership,
            ai,
            moderation,
            validation_task: Mutex::new(None),
        }
    }

    pub async fn start_hourly_validation(self: &Arc<Self>) {
        let mut task = self.validation_task.lock().await;
        if task.is_some() {
            return;
        }
        let states = Arc::downgrade(self);
        *task = Some(tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(3_600));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            interval.tick().await;
            loop {
                interval.tick().await;
                let Some(states) = states.upgrade() else {
                    return;
                };
                if let Err(error) = states.membership.status(now_millis(), false).await {
                    log::warn!("hourly membership validation degraded: {}", error.code);
                }
            }
        }));
    }

    pub async fn shutdown(&self) {
        if let Some(task) = self.validation_task.lock().await.take() {
            task.abort();
        }
        self.ai.shutdown().await;
    }
}

fn detect_violation(
    window: &UserWindow,
    config: &ModerationConfigRecord,
    now: i64,
    current: &str,
) -> Option<String> {
    let count_since = |seconds: u32| {
        window
            .entries
            .iter()
            .filter(|(at, _)| now - *at <= i64::from(seconds) * 1_000)
            .count()
    };
    if count_since(config.frequency_window_seconds) > config.frequency_threshold as usize {
        return Some("frequency".to_owned());
    }
    let recent_repeats = window
        .entries
        .iter()
        .filter(|(at, value)| {
            now - *at <= i64::from(config.repeat_window_seconds) * 1_000 && value == current
        })
        .count();
    if recent_repeats > config.repeat_threshold as usize {
        return Some("duplicate".to_owned());
    }
    let recent_similar = window
        .entries
        .iter()
        .filter(|(at, value)| {
            now - *at <= i64::from(config.similarity_window_seconds) * 1_000
                && similarity(value, current) > config.similarity_ratio
        })
        .count();
    (recent_similar > config.similarity_threshold as usize).then(|| "similarity".to_owned())
}

pub(crate) fn similarity(left: &str, right: &str) -> f64 {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    let maximum = left.len().max(right.len());
    if maximum == 0 {
        return 1.0;
    }
    let mut previous: Vec<usize> = (0..=right.len()).collect();
    for (left_index, left_char) in left.iter().enumerate() {
        let mut current = vec![left_index + 1];
        for (right_index, right_char) in right.iter().enumerate() {
            current.push(
                (previous[right_index + 1] + 1)
                    .min(current[right_index] + 1)
                    .min(previous[right_index] + usize::from(left_char != right_char)),
            );
        }
        previous = current;
    }
    1.0 - previous[right.len()] as f64 / maximum as f64
}

fn module_status(modules: &[MemberModule], id: &str) -> MemberModuleStatus {
    modules
        .iter()
        .find(|module| module.module_id.as_str() == id)
        .map(|module| module.status)
        .unwrap_or(MemberModuleStatus::Missing)
}

const fn shell_state(validity: EntitlementValidity) -> MemberShellState {
    match validity {
        EntitlementValidity::ActiveOnline | EntitlementValidity::ActiveOffline => {
            MemberShellState::Active
        }
        EntitlementValidity::Expired => MemberShellState::Expired,
        EntitlementValidity::VerificationRequired | EntitlementValidity::InvalidClock => {
            MemberShellState::Unavailable
        }
    }
}

const fn unavailable_capabilities() -> MemberCapabilities {
    MemberCapabilities {
        ai: AiCapabilityState::Unavailable,
        moderation: AiCapabilityState::Unavailable,
    }
}

fn default_ai_config() -> AiConfigRecord {
    AiConfigRecord {
        identity: String::new(),
        speaking_style: "short".to_owned(),
        system_prompt: String::new(),
        provider: String::new(),
        endpoint: None,
        model: String::new(),
        api_key_ciphertext: None,
        temperature: 0.7,
        max_length: 100,
        is_sensitive_filter_enabled: true,
        banned_words: Vec::new(),
        danmaku_context_count: 20,
        asr_context_count: 0,
        manual_context_count: 0,
        updated_at: now_millis(),
    }
}

fn default_moderation_config() -> ModerationConfigRecord {
    let defaults = moderation_defaults();
    ModerationConfigRecord {
        is_auto_enabled: defaults.auto_mute_enabled,
        default_mute_duration: "1h".to_owned(),
        frequency_window_seconds: u32::from(defaults.frequency_window_seconds),
        frequency_threshold: u32::from(defaults.frequency_limit),
        repeat_window_seconds: u32::from(defaults.duplicate_window_seconds),
        repeat_threshold: u32::from(defaults.duplicate_limit),
        similarity_window_seconds: u32::from(defaults.similarity_window_seconds),
        similarity_threshold: u32::from(defaults.similarity_limit),
        similarity_ratio: f64::from(defaults.similarity_threshold_percent) / 100.0,
        is_risk_confirmed: false,
        updated_at: now_millis(),
    }
}

pub fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(i64::MAX)
}
