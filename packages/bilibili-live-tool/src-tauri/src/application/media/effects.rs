use std::{collections::VecDeque, sync::Arc, time::Duration};

use tokio::sync::{watch, Mutex, Notify, RwLock};

use crate::{
    application::media::{AudioOutputPort, AudioRequest, EffectWindowPort, EffectWindowSpec},
    domain::{
        live_data::{AudienceIdentity, CaptainAction, CaptainLevel, LiveEvent},
        media::{CaptainEffectConfig, MediaSoundBinding},
    },
};

const EFFECT_QUEUE_CAPACITY: usize = 64;
const EFFECT_WINDOW_LABEL: &str = "effect-captain";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectItem {
    pub event_id: String,
    pub username: String,
    pub audience: Option<AudienceIdentity>,
    pub action: CaptainAction,
    pub level: CaptainLevel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveCaptainEffect {
    pub item: EffectItem,
    pub config: CaptainEffectConfig,
    pub is_preview: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectRuntimeSnapshot {
    pub current: Option<EffectItem>,
    pub pending_count: usize,
    pub fallback_current: Option<EffectItem>,
    pub dropped_count: u64,
}

#[derive(Default)]
struct EffectQueueState {
    pending: VecDeque<EffectQueueEntry>,
    snapshot: EffectRuntimeSnapshot,
    active: Option<ActiveCaptainEffect>,
}

struct EffectQueueEntry {
    item: EffectItem,
    config_override: Option<CaptainEffectConfig>,
}

struct EffectWorker {
    queue: Arc<Mutex<EffectQueueState>>,
    config: Arc<RwLock<CaptainEffectConfig>>,
    notify: Arc<Notify>,
    audio: Arc<dyn AudioOutputPort>,
    window: Arc<dyn EffectWindowPort>,
}

pub struct EffectRuntime {
    config: Arc<RwLock<CaptainEffectConfig>>,
    queue: Arc<Mutex<EffectQueueState>>,
    notify: Arc<Notify>,
    shutdown: watch::Sender<bool>,
    worker: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl EffectRuntime {
    pub async fn start(
        config: CaptainEffectConfig,
        audio: Arc<dyn AudioOutputPort>,
        window: Arc<dyn EffectWindowPort>,
    ) -> Arc<Self> {
        let (shutdown, receiver) = watch::channel(false);
        let runtime = Arc::new(Self {
            config: Arc::new(RwLock::new(config)),
            queue: Arc::new(Mutex::new(EffectQueueState::default())),
            notify: Arc::new(Notify::new()),
            shutdown,
            worker: Mutex::new(None),
        });
        let worker = tokio::spawn(
            EffectWorker {
                queue: runtime.queue.clone(),
                config: runtime.config.clone(),
                notify: runtime.notify.clone(),
                audio,
                window,
            }
            .run(receiver),
        );
        runtime.worker.lock().await.replace(worker);
        runtime
    }

    pub async fn enqueue(&self, event: &LiveEvent) {
        let LiveEvent::Captain {
            base,
            action,
            level,
            ..
        } = event
        else {
            return;
        };
        let item = EffectItem {
            event_id: base.event_id.clone(),
            username: base.audience.username.clone(),
            audience: Some(base.audience.clone()),
            action: *action,
            level: *level,
        };
        self.enqueue_item(item, None).await;
    }

    pub async fn preview(&self, config: CaptainEffectConfig, event_id: String) {
        self.enqueue_item(
            EffectItem {
                event_id,
                username: "用户A".into(),
                audience: None,
                action: CaptainAction::Purchase,
                level: CaptainLevel::Captain,
            },
            Some(config),
        )
        .await;
    }

    async fn enqueue_item(&self, item: EffectItem, config_override: Option<CaptainEffectConfig>) {
        let mut queue = self.queue.lock().await;
        if queue.pending.len() >= EFFECT_QUEUE_CAPACITY {
            queue.pending.pop_front();
            queue.snapshot.dropped_count += 1;
        }
        queue.pending.push_back(EffectQueueEntry {
            item,
            config_override,
        });
        queue.snapshot.pending_count = queue.pending.len();
        drop(queue);
        self.notify.notify_one();
    }

    pub async fn snapshot(&self) -> EffectRuntimeSnapshot {
        self.queue.lock().await.snapshot.clone()
    }

    pub async fn active_effect(&self) -> Option<ActiveCaptainEffect> {
        self.queue.lock().await.active.clone()
    }

    pub async fn set_config(&self, config: CaptainEffectConfig) {
        *self.config.write().await = config;
    }

    pub async fn config(&self) -> CaptainEffectConfig {
        self.config.read().await.clone()
    }

    pub async fn shutdown(&self) {
        let _ = self.shutdown.send(true);
        self.notify.notify_waiters();
        if let Some(worker) = self.worker.lock().await.take() {
            let _ = worker.await;
        }
    }
}

impl EffectWorker {
    async fn run(self, mut shutdown: watch::Receiver<bool>) {
        loop {
            let next = {
                let mut state = self.queue.lock().await;
                let next = state.pending.pop_front();
                state.snapshot.pending_count = state.pending.len();
                next
            };
            let Some(entry) = next else {
                tokio::select! {
                    _ = self.notify.notified() => continue,
                    changed = shutdown.changed() => {
                        if changed.is_err() || *shutdown.borrow() { break; }
                    }
                }
                continue;
            };
            if *shutdown.borrow() {
                break;
            }
            let item = entry.item;
            let is_preview = entry.config_override.is_some();
            let current_config = match entry.config_override {
                Some(config) => config,
                None => self.config.read().await.clone(),
            };
            {
                let mut state = self.queue.lock().await;
                state.snapshot.current = Some(item.clone());
                state.active = Some(ActiveCaptainEffect {
                    item: item.clone(),
                    config: current_config.clone(),
                    is_preview,
                });
            }
            let spec = EffectWindowSpec {
                label: EFFECT_WINDOW_LABEL.into(),
                width: current_config.window_width,
                height: current_config.window_height,
                position: current_config.position,
            };
            let uses_fallback = self.window.ensure(&spec).await.is_err()
                || self.window.show(EFFECT_WINDOW_LABEL).await.is_err();
            if uses_fallback {
                self.queue.lock().await.snapshot.fallback_current = Some(item.clone());
            }
            let binding = effect_sound(&current_config, item.action);
            let playback = if let Some(binding) = binding {
                self.audio
                    .play(AudioRequest {
                        source: binding.source.clone(),
                        volume: current_config.volume,
                    })
                    .await
                    .ok()
            } else {
                None
            };
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(u64::from(current_config.duration_seconds))) => {}
                changed = shutdown.changed() => {
                    if changed.is_err() || *shutdown.borrow() {}
                }
            }
            if let Some(playback) = playback {
                playback.stop();
            }
            let _ = self.window.close(EFFECT_WINDOW_LABEL).await;
            let mut state = self.queue.lock().await;
            state.snapshot.current = None;
            state.snapshot.fallback_current = None;
            state.active = None;
            if *shutdown.borrow() {
                state.pending.clear();
                state.snapshot.pending_count = 0;
                break;
            }
        }
    }
}

fn effect_sound(config: &CaptainEffectConfig, action: CaptainAction) -> Option<&MediaSoundBinding> {
    match action {
        CaptainAction::Purchase => config.first_sound.as_ref(),
        CaptainAction::Renewal => config.renewal_sound.as_ref(),
    }
}
