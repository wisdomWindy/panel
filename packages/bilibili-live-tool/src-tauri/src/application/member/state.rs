use std::{
    collections::{HashMap, VecDeque},
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::sync::{watch, Mutex, Notify};

use crate::{
    application::{
        live_data::{AcceptedLiveEventConsumer, LiveRuntimeError},
        member::{
            AiConfigRecord, AiGenerationRecord, AiGenerationStatus, AiModelConnectionState,
            AiModelProfileRecord, AiOutputDeviceRecord, AiSecretStore, AiSpeechOutput,
            AiVoiceConfigRecord, AiVoiceQueueItemRecord, AiVoiceQueueSnapshotRecord,
            AiVoiceQueueState, AiVoiceRecord, AuthProvider, EntitlementVerifier, LlmProvider,
            MemberEventSink, MemberRepository, MemberServiceError, MembershipProvider,
            ModerationAlertRecord, ModerationAlertStatus, ModerationConfigRecord,
            ModerationProvider, NoopMemberEventSink, OrderRecord, UnavailableAiSecretStore,
            UnavailableAiSpeechOutput,
        },
    },
    domain::{
        live_data::{IdentityTag, LiveEvent},
        member::{
            derive_capability, evaluate_entitlement, filter_and_truncate, moderation_defaults,
            truncate_sentences, AiCapabilityState, EntitlementValidity, MemberAccount,
            MemberCapabilities, MemberModule, MemberModuleStatus, MemberShellState, MemberSnapshot,
            ModuleId, OrderId, DEFAULT_LLM_TIMEOUT_MILLIS,
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
    last_auto_at: Option<i64>,
    user_last_auto_at: HashMap<String, i64>,
    recent_auto_prompts: VecDeque<String>,
}

#[derive(Default)]
struct AiVoiceRuntimeState {
    queue: Vec<AiVoiceQueueItemRecord>,
    current: Option<AiVoiceQueueItemRecord>,
    is_paused: bool,
    last_error_code: Option<String>,
    next_sequence: u64,
}

struct AiVoiceRuntime {
    repository: Arc<dyn MemberRepository>,
    speech: Arc<dyn AiSpeechOutput>,
    state: Arc<Mutex<AiVoiceRuntimeState>>,
    wake: Arc<Notify>,
    shutdown: watch::Sender<bool>,
    worker: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl AiVoiceRuntime {
    fn start(repository: Arc<dyn MemberRepository>, speech: Arc<dyn AiSpeechOutput>) -> Arc<Self> {
        let (shutdown, receiver) = watch::channel(false);
        let runtime = Arc::new(Self {
            repository,
            speech,
            state: Arc::new(Mutex::new(AiVoiceRuntimeState::default())),
            wake: Arc::new(Notify::new()),
            shutdown,
            worker: Mutex::new(None),
        });
        let worker_runtime = runtime.clone();
        let worker = tokio::spawn(async move { worker_runtime.run(receiver).await });
        if let Ok(mut slot) = runtime.worker.try_lock() {
            slot.replace(worker);
        }
        runtime
    }

    async fn enqueue(&self, text: String, priority: u8) -> Result<(), MemberServiceError> {
        let config = self.repository.load_ai_voice_config().await?;
        if !config.is_enabled {
            return Ok(());
        }
        let mut state = self.state.lock().await;
        state.next_sequence += 1;
        let item = AiVoiceQueueItemRecord {
            item_id: format!("ai-voice-{}", state.next_sequence),
            text,
            priority,
            created_at: now_millis(),
        };
        if state.queue.len() >= config.queue_capacity as usize {
            if let Some((index, _)) = state
                .queue
                .iter()
                .enumerate()
                .min_by_key(|(_, queued)| (queued.priority, queued.created_at))
            {
                if state.queue[index].priority <= item.priority {
                    state.queue.remove(index);
                } else {
                    state.last_error_code = Some("ai_voice_queue_full".to_owned());
                    return Ok(());
                }
            }
        }
        let insert_at = state
            .queue
            .iter()
            .position(|queued| queued.priority < item.priority)
            .unwrap_or(state.queue.len());
        state.queue.insert(insert_at, item);
        drop(state);
        self.wake.notify_one();
        Ok(())
    }

    async fn set_paused(&self, is_paused: bool) {
        self.state.lock().await.is_paused = is_paused;
        if !is_paused {
            self.wake.notify_one();
        }
    }

    async fn skip(&self) -> Result<(), MemberServiceError> {
        self.speech.stop().await?;
        self.state.lock().await.current = None;
        self.wake.notify_one();
        Ok(())
    }

    async fn clear(&self) {
        self.state.lock().await.queue.clear();
    }

    async fn snapshot(&self, config: &AiVoiceConfigRecord) -> AiVoiceQueueSnapshotRecord {
        let state = self.state.lock().await;
        AiVoiceQueueSnapshotRecord {
            state: if state.is_paused {
                AiVoiceQueueState::Paused
            } else if state.current.is_some() {
                AiVoiceQueueState::Speaking
            } else if config.output_device_id.is_none() {
                AiVoiceQueueState::Unavailable
            } else {
                AiVoiceQueueState::Idle
            },
            current: state.current.clone(),
            pending: state.queue.clone(),
            capacity: config.queue_capacity,
            volume: config.volume,
            selected_device_id: config.output_device_id.clone(),
            selected_voice_id: config.voice_id.clone(),
            is_audio_route_confirmed: config.is_audio_route_confirmed,
            last_error_code: state.last_error_code.clone(),
        }
    }

    async fn run(self: Arc<Self>, mut shutdown: watch::Receiver<bool>) {
        loop {
            let config = match self.repository.load_ai_voice_config().await {
                Ok(config) => config,
                Err(_) => {
                    self.state.lock().await.last_error_code =
                        Some("ai_voice_config_unavailable".to_owned());
                    tokio::select! {
                        _ = self.wake.notified() => continue,
                        changed = shutdown.changed() => {
                            if changed.is_err() || *shutdown.borrow() { break; }
                        }
                    }
                    continue;
                }
            };
            let next = {
                let mut state = self.state.lock().await;
                if !config.is_enabled || state.is_paused || state.current.is_some() {
                    None
                } else {
                    state.queue.first().cloned().inspect(|item| {
                        state.queue.remove(0);
                        state.current = Some(item.clone());
                    })
                }
            };
            let Some(item) = next else {
                tokio::select! {
                    _ = self.wake.notified() => continue,
                    changed = shutdown.changed() => {
                        if changed.is_err() || *shutdown.borrow() { break; }
                    }
                }
                continue;
            };
            let result = match config.output_device_id.as_deref() {
                Some(device_id) if config.is_audio_route_confirmed => {
                    self.speech
                        .speak(&item.text, device_id, &config.voice_id, config.volume)
                        .await
                }
                _ => Err(MemberServiceError::invalid("ai_audio_route_unconfirmed")),
            };
            let error = result.err();
            let lost_device = error.as_ref().is_some_and(|error| {
                matches!(
                    error.code,
                    "ai_audio_device_invalid" | "ai_audio_device_unavailable"
                )
            });
            if lost_device {
                let mut degraded = config.clone();
                degraded.is_enabled = false;
                degraded.is_audio_route_confirmed = false;
                degraded.updated_at = now_millis();
                let _ = self.repository.save_ai_voice_config(&degraded).await;
            }
            let mut state = self.state.lock().await;
            state.current = None;
            state.last_error_code = error.map(|error| {
                if lost_device {
                    "ai_audio_device_missing".to_owned()
                } else {
                    error.code.to_owned()
                }
            });
            if *shutdown.borrow() {
                state.queue.clear();
                break;
            }
        }
    }

    async fn shutdown(&self) {
        let _ = self.shutdown.send(true);
        self.wake.notify_waiters();
        let _ = self.speech.stop().await;
        if let Some(worker) = self.worker.lock().await.take() {
            let _ = worker.await;
        }
    }
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
    secrets: Arc<dyn AiSecretStore>,
    speech: Arc<dyn AiSpeechOutput>,
    voice: Arc<AiVoiceRuntime>,
    event_sink: Arc<dyn MemberEventSink>,
    runtime: Mutex<AiRuntime>,
    active_generation: AtomicBool,
}

impl AiState {
    pub async fn query_logs(
        &self,
        user_uid: Option<&str>,
        started_at: Option<i64>,
        ended_at: Option<i64>,
    ) -> Result<Vec<super::AiLogRecord>, MemberServiceError> {
        self.repository
            .query_ai_logs(user_uid, started_at, ended_at)
            .await
    }
    pub fn new(repository: Arc<dyn MemberRepository>, llm: Arc<dyn LlmProvider>) -> Self {
        Self::new_with_services(
            repository,
            llm,
            Arc::new(UnavailableAiSecretStore),
            Arc::new(UnavailableAiSpeechOutput),
        )
    }

    pub fn new_with_services(
        repository: Arc<dyn MemberRepository>,
        llm: Arc<dyn LlmProvider>,
        secrets: Arc<dyn AiSecretStore>,
        speech: Arc<dyn AiSpeechOutput>,
    ) -> Self {
        let voice = AiVoiceRuntime::start(repository.clone(), speech.clone());
        Self {
            repository,
            llm,
            secrets,
            speech,
            voice,
            event_sink: Arc::new(NoopMemberEventSink),
            runtime: Mutex::new(AiRuntime::default()),
            active_generation: AtomicBool::new(false),
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

    pub async fn model_profiles(&self) -> Result<Vec<AiModelProfileRecord>, MemberServiceError> {
        self.repository.list_ai_model_profiles().await
    }

    pub async fn save_model_profile(
        &self,
        mut profile: AiModelProfileRecord,
        api_key_replacement: Option<String>,
    ) -> Result<AiModelProfileRecord, MemberServiceError> {
        let existing = self
            .repository
            .load_ai_model_profile(&profile.profile_id)
            .await?;
        profile.is_default = existing.as_ref().is_some_and(|value| value.is_default);
        profile.has_api_key = existing.as_ref().is_some_and(|value| value.has_api_key);
        if profile.provider_kind != "local" && api_key_replacement.is_none() && !profile.has_api_key
        {
            return Err(MemberServiceError::invalid("ai_api_key_required"));
        }
        if let Some(api_key) = api_key_replacement {
            let api_key = api_key.trim();
            if api_key.is_empty() {
                return Err(MemberServiceError::invalid("ai_api_key_invalid"));
            }
            self.secrets.set(&profile.profile_id, api_key).await?;
            profile.has_api_key = true;
        }
        self.repository.save_ai_model_profile(&profile).await?;
        Ok(profile)
    }

    pub async fn delete_model_profile(&self, profile_id: &str) -> Result<bool, MemberServiceError> {
        let profile = self
            .repository
            .load_ai_model_profile(profile_id)
            .await?
            .ok_or_else(|| MemberServiceError::invalid("ai_model_profile_not_found"))?;
        if profile.is_default {
            return Err(MemberServiceError::invalid(
                "ai_default_profile_delete_forbidden",
            ));
        }
        let deleted = self.repository.delete_ai_model_profile(profile_id).await?;
        if deleted {
            self.secrets.delete(profile_id).await?;
        }
        Ok(deleted)
    }

    pub async fn set_default_model_profile(
        &self,
        profile_id: &str,
    ) -> Result<Vec<AiModelProfileRecord>, MemberServiceError> {
        self.repository
            .set_default_ai_model_profile(profile_id, now_millis())
            .await?;
        self.repository.list_ai_model_profiles().await
    }

    pub async fn test_model_profile(
        &self,
        profile_id: &str,
    ) -> Result<AiModelProfileRecord, MemberServiceError> {
        let mut profile = self
            .repository
            .load_ai_model_profile(profile_id)
            .await?
            .ok_or_else(|| MemberServiceError::invalid("ai_model_profile_not_found"))?;
        let api_key = if profile.has_api_key {
            self.secrets.get(profile_id).await?
        } else {
            None
        };
        let result = self.llm.test_profile(&profile, api_key.as_deref()).await;
        profile.connection_state = if result.is_ok() {
            AiModelConnectionState::Available
        } else {
            AiModelConnectionState::Unavailable
        };
        profile.updated_at = now_millis();
        self.repository.save_ai_model_profile(&profile).await?;
        result.map(|_| profile)
    }

    pub async fn voice_config(&self) -> Result<AiVoiceConfigRecord, MemberServiceError> {
        self.repository.load_ai_voice_config().await
    }

    pub async fn set_voice_enabled(
        &self,
        is_enabled: bool,
        is_audio_route_confirmed: bool,
        is_data_disclosure_acknowledged: bool,
    ) -> Result<AiVoiceConfigRecord, MemberServiceError> {
        let mut config = self.repository.load_ai_voice_config().await?;
        if is_enabled {
            // PRD 12.2: before enabling, the user must accept that danmaku text
            // and the audience nickname leave the machine for the configured
            // model endpoint. The UI shows this disclosure on every enable, so
            // the flag is required rather than persisted.
            if !is_data_disclosure_acknowledged {
                return Err(MemberServiceError::invalid("ai_data_disclosure_required"));
            }
            let profile = self.repository.load_default_ai_model_profile().await?;
            if !profile.is_some_and(|profile| profile.is_enabled) {
                return Err(MemberServiceError::invalid("ai_model_not_configured"));
            }
            if config.output_device_id.is_none()
                || !config.is_audio_route_confirmed
                || !is_audio_route_confirmed
            {
                return Err(MemberServiceError::invalid("ai_audio_route_unconfirmed"));
            }
            let devices = self.speech.devices().await?;
            if !devices.iter().any(|device| {
                Some(device.device_id.as_str()) == config.output_device_id.as_deref()
                    && device.is_available
            }) {
                config.is_enabled = false;
                config.is_audio_route_confirmed = false;
                config.updated_at = now_millis();
                self.repository.save_ai_voice_config(&config).await?;
                return Err(MemberServiceError::unavailable("ai_audio_device_missing"));
            }
            let voices = self.speech.voices().await?;
            if !voices
                .iter()
                .any(|voice| voice.voice_id == config.voice_id && voice.is_available)
            {
                config.is_enabled = false;
                config.is_audio_route_confirmed = false;
                config.updated_at = now_millis();
                self.repository.save_ai_voice_config(&config).await?;
                return Err(MemberServiceError::unavailable("ai_tts_voice_invalid"));
            }
        }
        config.is_enabled = is_enabled;
        config.updated_at = now_millis();
        self.repository.save_ai_voice_config(&config).await?;
        if !is_enabled {
            self.voice.clear().await;
            let _ = self.speech.stop().await;
        } else {
            self.voice.wake.notify_one();
        }
        Ok(config)
    }

    pub async fn update_voice_rules(
        &self,
        mut rules: AiVoiceConfigRecord,
    ) -> Result<AiVoiceConfigRecord, MemberServiceError> {
        let current = self.repository.load_ai_voice_config().await?;
        rules.is_enabled = current.is_enabled;
        rules.output_device_id = current.output_device_id;
        rules.volume = current.volume;
        rules.is_audio_route_confirmed = current.is_audio_route_confirmed;
        rules.updated_at = now_millis();
        self.repository.save_ai_voice_config(&rules).await?;
        Ok(rules)
    }

    pub async fn output_devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
        self.speech.devices().await
    }

    pub async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
        self.speech.voices().await
    }

    pub async fn set_output_device(
        &self,
        device_id: String,
        voice_id: String,
        volume: f64,
        is_audio_route_confirmed: bool,
    ) -> Result<AiVoiceConfigRecord, MemberServiceError> {
        let devices = self.speech.devices().await?;
        let voices = self.speech.voices().await?;
        if !devices
            .iter()
            .any(|device| device.device_id == device_id && device.is_available)
            || !volume.is_finite()
            || !(0.0..=1.0).contains(&volume)
        {
            return Err(MemberServiceError::invalid("ai_audio_device_invalid"));
        }
        if !voices
            .iter()
            .any(|voice| voice.voice_id == voice_id && voice.is_available)
        {
            return Err(MemberServiceError::invalid("ai_tts_voice_invalid"));
        }
        let mut config = self.repository.load_ai_voice_config().await?;
        if config.output_device_id.as_deref() != Some(&device_id) || config.voice_id != voice_id {
            config.is_enabled = false;
        }
        config.output_device_id = Some(device_id);
        config.voice_id = voice_id;
        config.volume = volume;
        config.is_audio_route_confirmed = is_audio_route_confirmed;
        config.updated_at = now_millis();
        self.repository.save_ai_voice_config(&config).await?;
        Ok(config)
    }

    pub async fn test_output_device(
        &self,
        device_id: &str,
        voice_id: &str,
        volume: f64,
    ) -> Result<(), MemberServiceError> {
        let devices = self.speech.devices().await?;
        if !devices
            .iter()
            .any(|device| device.device_id == device_id && device.is_available)
            || !volume.is_finite()
            || !(0.0..=1.0).contains(&volume)
        {
            return Err(MemberServiceError::invalid("ai_audio_device_invalid"));
        }
        let voices = self.speech.voices().await?;
        if !voices
            .iter()
            .any(|voice| voice.voice_id == voice_id && voice.is_available)
        {
            return Err(MemberServiceError::invalid("ai_tts_voice_invalid"));
        }
        self.speech
            .speak(
                "AI 语音回复测试，音频路由正常。",
                device_id,
                voice_id,
                volume,
            )
            .await
    }

    pub async fn pause_voice(&self, is_paused: bool) {
        self.voice.set_paused(is_paused).await;
    }

    pub async fn skip_voice(&self) -> Result<(), MemberServiceError> {
        self.voice.skip().await
    }

    pub async fn clear_voice(&self) {
        self.voice.clear().await;
    }

    pub async fn voice_queue_snapshot(
        &self,
    ) -> Result<AiVoiceQueueSnapshotRecord, MemberServiceError> {
        let config = self.repository.load_ai_voice_config().await?;
        Ok(self.voice.snapshot(&config).await)
    }

    pub async fn generate(
        &self,
        prompt: String,
        context: Vec<String>,
    ) -> Result<AiGenerationRecord, MemberServiceError> {
        self.generate_with_priority(prompt, context, 1).await
    }

    async fn generate_with_priority(
        &self,
        prompt: String,
        context: Vec<String>,
        priority: u8,
    ) -> Result<AiGenerationRecord, MemberServiceError> {
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
        let input = context
            .iter()
            .chain(std::iter::once(&prompt))
            .filter(|value| !value.trim().is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");
        let assembled = format!(
            "主播身份：{}\n主播人设：{}\n说话风格：{}\n观众输入：{}",
            config.identity.trim(),
            config.system_prompt.trim(),
            config.speaking_style,
            input
        );
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
            .map(|value| truncate_sentences(&value, config.max_sentences as usize))
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
                runtime.last_request = Some((prompt, context.clone(), generation_id.clone()));
                let generation = AiGenerationRecord {
                    generation_id,
                    status: AiGenerationStatus::Completed,
                    result,
                    context_used: context,
                    last_sequence: 1,
                    error_code: None,
                };
                self.voice
                    .enqueue(generation.result.clone(), priority)
                    .await?;
                self.event_sink.publish_ai_generation(&generation);
                Ok(generation)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn regenerate(
        &self,
        generation_id: &str,
    ) -> Result<AiGenerationRecord, MemberServiceError> {
        let request = self
            .runtime
            .lock()
            .await
            .last_request
            .clone()
            .filter(|(_, _, id)| id == generation_id)
            .ok_or_else(|| MemberServiceError::invalid("ai_generation_not_found"))?;
        self.generate(request.0, request.1).await
    }

    pub async fn shutdown(&self) {
        self.voice.shutdown().await;
        self.active_generation.store(false, Ordering::Release);
    }
}

#[async_trait::async_trait]
impl AcceptedLiveEventConsumer for AiState {
    async fn consume_accepted(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError> {
        let LiveEvent::Danmu { base, content } = event else {
            return Ok(());
        };
        let config = match self.repository.load_ai_voice_config().await {
            Ok(config) if config.is_enabled => config,
            Ok(_) => return Ok(()),
            Err(error) => {
                log::warn!("AI voice config degraded: {}", error.code);
                return Ok(());
            }
        };
        if !trigger_matches(&config, event, content) {
            return Ok(());
        }
        let now = now_millis();
        let uid = base.audience.user_uid.as_str().to_owned();
        {
            let mut runtime = self.runtime.lock().await;
            if runtime
                .last_auto_at
                .is_some_and(|last| now - last < i64::from(config.global_cooldown_seconds) * 1_000)
                || runtime.user_last_auto_at.get(&uid).is_some_and(|last| {
                    now - *last < i64::from(config.user_cooldown_seconds) * 1_000
                })
            {
                return Ok(());
            }
            let normalized = content.trim().to_lowercase();
            if config.is_duplicate_filter_enabled
                && runtime
                    .recent_auto_prompts
                    .iter()
                    .any(|recent| recent == &normalized)
            {
                return Ok(());
            }
            runtime.last_auto_at = Some(now);
            runtime.user_last_auto_at.insert(uid, now);
            runtime.recent_auto_prompts.push_back(normalized);
            while runtime.recent_auto_prompts.len() > 20 {
                runtime.recent_auto_prompts.pop_front();
            }
        }
        let priority = ai_event_priority(event);
        if let Err(error) = self
            .generate_with_priority(
                content.clone(),
                vec![format!("{}：{}", base.audience.username, content)],
                priority,
            )
            .await
        {
            log::warn!("AI voice reply skipped: {}", error.code);
        }
        Ok(())
    }
}

fn trigger_matches(config: &AiVoiceConfigRecord, event: &LiveEvent, content: &str) -> bool {
    let tags = &event.base().audience.tags;
    match config.trigger_scope.as_str() {
        "all" => true,
        "questions" => {
            let value = content.trim();
            value.ends_with('?')
                || value.ends_with('？')
                || ["吗", "呢", "怎么", "为什么", "多少", "哪里", "谁"]
                    .iter()
                    .any(|marker| value.contains(marker))
        }
        "captains" => tags.iter().any(|tag| {
            matches!(
                tag,
                IdentityTag::Governor | IdentityTag::Admiral | IdentityTag::Captain
            )
        }),
        "keywords" => config
            .keywords
            .iter()
            .any(|keyword| content.contains(keyword)),
        "high_value" => tags.iter().any(|tag| {
            matches!(
                tag,
                IdentityTag::Governor
                    | IdentityTag::Admiral
                    | IdentityTag::Captain
                    | IdentityTag::Admin
                    | IdentityTag::Vip
            )
        }),
        _ => false,
    }
}

fn ai_event_priority(event: &LiveEvent) -> u8 {
    let tags = &event.base().audience.tags;
    if tags.contains(&IdentityTag::Governor) || tags.contains(&IdentityTag::Admiral) {
        5
    } else if tags.contains(&IdentityTag::Captain) {
        4
    } else if tags.contains(&IdentityTag::Admin) {
        3
    } else if tags.contains(&IdentityTag::Vip) {
        2
    } else {
        1
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
    pub moderation: Arc<ModerationState>,
    validation_task: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl MemberStates {
    pub fn new(membership: Arc<MembershipState>, moderation: Arc<ModerationState>) -> Self {
        Self {
            membership,
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
        max_sentences: 2,
        is_sensitive_filter_enabled: true,
        banned_words: Vec::new(),
        danmaku_context_count: 20,
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
