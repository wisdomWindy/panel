use std::sync::Arc;

use tokio::sync::{watch, Mutex, Notify, RwLock};

use crate::{
    application::media::{SpeechEnginePort, SpeechRequest},
    domain::{
        live_data::{CaptainLevel, IdentityTag, LiveEvent},
        media::{
            enqueue_tts, TtsConfig, TtsEventKind, TtsIdentityKind, TtsPriority, TtsQueueItem,
            TtsSelectedEngine,
        },
    },
};

const TTS_QUEUE_CAPACITY: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveSpeechEngine {
    System,
    Cloud,
    LocalFallback,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TtsRuntimeSnapshot {
    pub current_text: Option<String>,
    pub pending_count: usize,
    pub is_paused: bool,
    pub active_engine: ActiveSpeechEngine,
    pub fallback_reason_code: Option<&'static str>,
    pub dropped_count: u64,
}

struct TtsRuntimeState {
    queue: Vec<TtsQueueItem>,
    current: Option<TtsQueueItem>,
    is_paused: bool,
    active_engine: ActiveSpeechEngine,
    fallback_reason_code: Option<&'static str>,
    next_sequence: u64,
    dropped_count: u64,
}

struct TtsWorker {
    config: Arc<RwLock<TtsConfig>>,
    state: Arc<Mutex<TtsRuntimeState>>,
    wake: Arc<Notify>,
    system: Arc<dyn SpeechEnginePort>,
    cloud: Arc<dyn SpeechEnginePort>,
    local: Arc<dyn SpeechEnginePort>,
}

pub struct TtsRuntime {
    config: Arc<RwLock<TtsConfig>>,
    state: Arc<Mutex<TtsRuntimeState>>,
    wake: Arc<Notify>,
    shutdown: watch::Sender<bool>,
    worker: Mutex<Option<tokio::task::JoinHandle<()>>>,
    system: Arc<dyn SpeechEnginePort>,
    cloud: Arc<dyn SpeechEnginePort>,
    local_fallback: Arc<dyn SpeechEnginePort>,
}

impl TtsRuntime {
    pub async fn start(
        config: TtsConfig,
        system: Arc<dyn SpeechEnginePort>,
        cloud: Arc<dyn SpeechEnginePort>,
        local_fallback: Arc<dyn SpeechEnginePort>,
    ) -> Arc<Self> {
        let selected = selected_engine(config.selected_engine);
        let (shutdown, receiver) = watch::channel(false);
        let runtime = Arc::new(Self {
            config: Arc::new(RwLock::new(config)),
            state: Arc::new(Mutex::new(TtsRuntimeState {
                queue: Vec::new(),
                current: None,
                is_paused: false,
                active_engine: selected,
                fallback_reason_code: None,
                next_sequence: 0,
                dropped_count: 0,
            })),
            wake: Arc::new(Notify::new()),
            shutdown,
            worker: Mutex::new(None),
            system,
            cloud,
            local_fallback,
        });
        let worker = tokio::spawn(
            TtsWorker {
                config: runtime.config.clone(),
                state: runtime.state.clone(),
                wake: runtime.wake.clone(),
                system: runtime.system.clone(),
                cloud: runtime.cloud.clone(),
                local: runtime.local_fallback.clone(),
            }
            .run(receiver),
        );
        runtime.worker.lock().await.replace(worker);
        runtime
    }

    pub async fn enqueue_event(&self, event: &LiveEvent) {
        let config = self.config.read().await.clone();
        let Some((text, priority)) = render_event(&config, event) else {
            return;
        };
        let mut state = self.state.lock().await;
        state.next_sequence += 1;
        let sequence = state.next_sequence;
        let before = state.queue.len();
        let duplicate = state
            .queue
            .iter()
            .any(|queued| queued.id == event.base().event_id);
        state.queue = enqueue_tts(
            &state.queue,
            TtsQueueItem {
                id: event.base().event_id.clone(),
                text,
                priority,
                sequence,
            },
            TTS_QUEUE_CAPACITY,
        );
        if !duplicate && before == TTS_QUEUE_CAPACITY && state.queue.len() == TTS_QUEUE_CAPACITY {
            state.dropped_count += 1;
        }
        drop(state);
        self.wake.notify_one();
    }

    pub async fn set_paused(&self, paused: bool) {
        self.state.lock().await.is_paused = paused;
        if !paused {
            self.wake.notify_one();
        }
    }

    pub async fn clear_pending(&self) {
        self.state.lock().await.queue.clear();
    }

    pub async fn set_config(&self, config: TtsConfig) {
        let selected = selected_engine(config.selected_engine);
        *self.config.write().await = config;
        self.state.lock().await.active_engine = selected;
    }

    pub async fn config(&self) -> TtsConfig {
        self.config.read().await.clone()
    }

    pub async fn voices(
        &self,
    ) -> Vec<(
        ActiveSpeechEngine,
        Result<
            Vec<crate::application::media::SpeechVoice>,
            crate::application::media::MediaIoError,
        >,
    )> {
        vec![
            (ActiveSpeechEngine::System, self.system.voices().await),
            (ActiveSpeechEngine::Cloud, self.cloud.voices().await),
            (
                ActiveSpeechEngine::LocalFallback,
                self.local_fallback.voices().await,
            ),
        ]
    }

    pub async fn test(
        &self,
        config: &TtsConfig,
        text: String,
    ) -> Result<(), crate::application::media::MediaIoError> {
        if text.trim().is_empty() {
            return Err(crate::application::media::MediaIoError::invalid(
                "speech test text is required",
            ));
        }
        let request = SpeechRequest {
            text,
            voice_id: config.selected_voice_id.clone(),
            rate: config.rate,
            volume: config.volume,
        };
        let candidates: Vec<&Arc<dyn SpeechEnginePort>> = match config.selected_engine {
            TtsSelectedEngine::System => {
                vec![&self.system, &self.cloud, &self.local_fallback]
            }
            // The explicit test paths deliberately exclude the system engine:
            // a "test" must prove the *selected* engine works, not that some
            // fallback saved us. Live playback keeps the wider chain below.
            TtsSelectedEngine::Cloud => vec![&self.cloud, &self.local_fallback],
        };
        let mut last_error = None;
        for engine in candidates {
            match engine.speak(request.clone()).await {
                Ok(()) => return Ok(()),
                Err(error) => last_error = Some(error),
            }
        }
        Err(last_error.unwrap_or_else(|| {
            crate::application::media::MediaIoError::unavailable("speech engine is unavailable")
        }))
    }

    pub async fn snapshot(&self) -> TtsRuntimeSnapshot {
        let state = self.state.lock().await;
        TtsRuntimeSnapshot {
            current_text: state.current.as_ref().map(|item| item.text.clone()),
            pending_count: state.queue.len(),
            is_paused: state.is_paused,
            active_engine: state.active_engine,
            fallback_reason_code: state.fallback_reason_code,
            dropped_count: state.dropped_count,
        }
    }

    pub async fn shutdown(&self) {
        let _ = self.shutdown.send(true);
        self.wake.notify_waiters();
        let _ = self.system.stop().await;
        let _ = self.cloud.stop().await;
        let _ = self.local_fallback.stop().await;
        if let Some(worker) = self.worker.lock().await.take() {
            let _ = worker.await;
        }
    }
}

impl TtsWorker {
    async fn run(self, mut shutdown: watch::Receiver<bool>) {
        loop {
            let next = {
                let mut state = self.state.lock().await;
                if state.is_paused || state.queue.is_empty() {
                    None
                } else {
                    let item = state.queue.remove(0);
                    state.current = Some(item.clone());
                    Some(item)
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
            if *shutdown.borrow() {
                break;
            }
            let current_config = self.config.read().await.clone();
            let request = SpeechRequest {
                text: item.text,
                voice_id: current_config.selected_voice_id.clone(),
                rate: current_config.rate,
                volume: current_config.volume,
            };
            let candidates: Vec<(ActiveSpeechEngine, Arc<dyn SpeechEnginePort>)> =
                match current_config.selected_engine {
                    TtsSelectedEngine::System => vec![
                        (ActiveSpeechEngine::System, self.system.clone()),
                        (ActiveSpeechEngine::Cloud, self.cloud.clone()),
                        (ActiveSpeechEngine::LocalFallback, self.local.clone()),
                    ],
                    TtsSelectedEngine::Cloud => vec![
                        (ActiveSpeechEngine::Cloud, self.cloud.clone()),
                        // A dead remote service must not silence the stream: the
                        // system voice still speaks, and the snapshot reports the
                        // degraded engine so the operator can react.
                        (ActiveSpeechEngine::System, self.system.clone()),
                        (ActiveSpeechEngine::LocalFallback, self.local.clone()),
                    ],
                };
            let mut active = None;
            for (kind, engine) in candidates {
                if engine.speak(request.clone()).await.is_ok() {
                    active = Some(kind);
                    break;
                }
            }
            let mut runtime = self.state.lock().await;
            runtime.current = None;
            match active {
                Some(active) => {
                    runtime.active_engine = active;
                    runtime.fallback_reason_code = (active
                        != selected_engine(current_config.selected_engine))
                    .then_some("tts_engine_fallback");
                }
                None => runtime.fallback_reason_code = Some("TTS_UNAVAILABLE_ENGINE"),
            }
            if *shutdown.borrow() {
                runtime.queue.clear();
                break;
            }
        }
    }
}

fn render_event(config: &TtsConfig, event: &LiveEvent) -> Option<(String, TtsPriority)> {
    if !config.is_enabled || !identity_allowed(config, event) {
        return None;
    }
    let username = &event.base().audience.username;
    match event {
        LiveEvent::Danmu { content, .. } if config.event_kinds.contains(&TtsEventKind::Danmu) => {
            Some((
                config
                    .templates
                    .danmu
                    .replace("{用户}", username)
                    .replace("{内容}", content),
                identity_priority(event),
            ))
        }
        LiveEvent::Gift {
            gift_name,
            quantity,
            ..
        } if config.event_kinds.contains(&TtsEventKind::Gift) => Some((
            config
                .templates
                .gift
                .replace("{用户}", username)
                .replace("{礼物}", gift_name)
                .replace("{数量}", &quantity.to_string()),
            TtsPriority::Gift,
        )),
        LiveEvent::Captain { level, .. } if config.event_kinds.contains(&TtsEventKind::Captain) => {
            Some((
                config
                    .templates
                    .captain
                    .replace("{用户}", username)
                    .replace("{等级}", captain_level_name(*level)),
                TtsPriority::Captain,
            ))
        }
        LiveEvent::Enter { .. } if config.event_kinds.contains(&TtsEventKind::Entry) => {
            Some((format!("{username}进入直播间"), identity_priority(event)))
        }
        _ => None,
    }
}

fn identity_allowed(config: &TtsConfig, event: &LiveEvent) -> bool {
    let tags = &event.base().audience.tags;
    (tags.contains(&IdentityTag::Admin) && config.identity_kinds.contains(&TtsIdentityKind::Admin))
        || (tags.iter().any(|tag| {
            matches!(
                tag,
                IdentityTag::Governor | IdentityTag::Admiral | IdentityTag::Captain
            )
        }) && config.identity_kinds.contains(&TtsIdentityKind::Captain))
        || (tags.contains(&IdentityTag::Vip)
            && config.identity_kinds.contains(&TtsIdentityKind::Vip))
        || (tags.is_empty() && config.identity_kinds.contains(&TtsIdentityKind::Ordinary))
}

fn identity_priority(event: &LiveEvent) -> TtsPriority {
    if event.base().audience.tags.contains(&IdentityTag::Admin) {
        TtsPriority::Admin
    } else {
        TtsPriority::Ordinary
    }
}

const fn selected_engine(engine: TtsSelectedEngine) -> ActiveSpeechEngine {
    match engine {
        TtsSelectedEngine::System => ActiveSpeechEngine::System,
        TtsSelectedEngine::Cloud => ActiveSpeechEngine::Cloud,
    }
}

const fn captain_level_name(level: CaptainLevel) -> &'static str {
    match level {
        CaptainLevel::Governor => "总督",
        CaptainLevel::Admiral => "提督",
        CaptainLevel::Captain => "舰长",
    }
}
