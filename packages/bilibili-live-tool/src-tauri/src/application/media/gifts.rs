use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use tokio::sync::{watch, Mutex, Notify, RwLock};

use crate::{
    application::media::{
        AudioOutputPort, AudioRequest, GiftSoundBindingRecord, NotificationPort,
        NotificationRequest,
    },
    domain::{
        live_data::LiveEvent,
        media::{reduce_gift_stats, GiftNotifyConfig, GiftRecord, GiftSoundPolicy, GiftStats},
    },
};

const GIFT_WORK_QUEUE_CAPACITY: usize = 128;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GiftRuntimeSnapshot {
    pub stats: GiftStats,
    pub popup_items: VecDeque<GiftRecord>,
    pub pending_count: usize,
    pub dropped_count: u64,
}

#[derive(Default)]
struct GiftRuntimeState {
    snapshot: GiftRuntimeSnapshot,
    pending: VecDeque<GiftWorkItem>,
}

struct GiftWorkItem {
    gift: GiftRecord,
    config_override: Option<GiftNotifyConfig>,
}

struct GiftWorker {
    notify_config: Arc<RwLock<GiftNotifyConfig>>,
    sound_policy: Arc<RwLock<GiftSoundPolicy>>,
    bindings: Arc<RwLock<HashMap<String, GiftSoundBindingRecord>>>,
    state: Arc<Mutex<GiftRuntimeState>>,
    wake: Arc<Notify>,
    audio: Arc<dyn AudioOutputPort>,
    notifications: Arc<dyn NotificationPort>,
}

pub struct GiftMediaRuntime {
    notify_config: Arc<RwLock<GiftNotifyConfig>>,
    sound_policy: Arc<RwLock<GiftSoundPolicy>>,
    bindings: Arc<RwLock<HashMap<String, GiftSoundBindingRecord>>>,
    state: Arc<Mutex<GiftRuntimeState>>,
    wake: Arc<Notify>,
    shutdown: watch::Sender<bool>,
    worker: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl GiftMediaRuntime {
    pub async fn start(
        notify_config: GiftNotifyConfig,
        sound_policy: GiftSoundPolicy,
        bindings: Vec<GiftSoundBindingRecord>,
        audio: Arc<dyn AudioOutputPort>,
        notifications: Arc<dyn NotificationPort>,
    ) -> Arc<Self> {
        let (shutdown, receiver) = watch::channel(false);
        let runtime = Arc::new(Self {
            notify_config: Arc::new(RwLock::new(notify_config)),
            sound_policy: Arc::new(RwLock::new(sound_policy)),
            bindings: Arc::new(RwLock::new(
                bindings
                    .into_iter()
                    .map(|binding| (binding.gift_id.clone(), binding))
                    .collect(),
            )),
            state: Arc::new(Mutex::new(GiftRuntimeState::default())),
            wake: Arc::new(Notify::new()),
            shutdown,
            worker: Mutex::new(None),
        });
        let worker = tokio::spawn(
            GiftWorker {
                notify_config: runtime.notify_config.clone(),
                sound_policy: runtime.sound_policy.clone(),
                bindings: runtime.bindings.clone(),
                state: runtime.state.clone(),
                wake: runtime.wake.clone(),
                audio,
                notifications,
            }
            .run(receiver),
        );
        runtime.worker.lock().await.replace(worker);
        runtime
    }

    pub async fn consume(&self, event: &LiveEvent) {
        let LiveEvent::Gift {
            base,
            gift_id,
            gift_name,
            gift_icon_url,
            quantity,
            value_minor,
        } = event
        else {
            return;
        };
        let record = GiftRecord {
            event_id: base.event_id.clone(),
            room_id: base.room_id.as_str().to_owned(),
            user_uid: base.audience.user_uid.as_str().to_owned(),
            username: base.audience.username.clone(),
            gift_id: gift_id.clone(),
            gift_name: gift_name.clone(),
            gift_icon_url: gift_icon_url.clone(),
            quantity: *quantity,
            value_minor: *value_minor,
            occurred_at: base.occurred_at,
        };
        let mut state = self.state.lock().await;
        if state
            .snapshot
            .stats
            .seen_event_ids
            .contains(&record.event_id)
        {
            return;
        }
        if let Ok(stats) = reduce_gift_stats(&state.snapshot.stats, record.clone()) {
            state.snapshot.stats = stats;
        } else {
            return;
        }
        if state.pending.len() >= GIFT_WORK_QUEUE_CAPACITY {
            state.pending.pop_front();
            state.snapshot.dropped_count += 1;
        }
        state.pending.push_back(GiftWorkItem {
            gift: record,
            config_override: None,
        });
        state.snapshot.pending_count = state.pending.len();
        drop(state);
        self.wake.notify_one();
    }

    pub async fn preview(&self, config: GiftNotifyConfig, event_id: String) -> GiftRecord {
        let gift = GiftRecord {
            event_id,
            room_id: "1".into(),
            user_uid: "0".into(),
            username: "用户A".into(),
            gift_id: "preview".into(),
            gift_name: "礼物".into(),
            gift_icon_url: None,
            quantity: 1,
            value_minor: config.high_value_threshold_minor,
            occurred_at: 1,
        };
        let mut state = self.state.lock().await;
        if state.pending.len() >= GIFT_WORK_QUEUE_CAPACITY {
            state.pending.pop_front();
            state.snapshot.dropped_count += 1;
        }
        state.pending.push_back(GiftWorkItem {
            gift: gift.clone(),
            config_override: Some(config),
        });
        state.snapshot.pending_count = state.pending.len();
        drop(state);
        self.wake.notify_one();
        gift
    }

    pub async fn snapshot(&self) -> GiftRuntimeSnapshot {
        self.state.lock().await.snapshot.clone()
    }

    pub async fn notify_config(&self) -> GiftNotifyConfig {
        self.notify_config.read().await.clone()
    }

    pub async fn set_notify_config(&self, config: GiftNotifyConfig) {
        *self.notify_config.write().await = config;
    }

    pub async fn sound_policy(&self) -> GiftSoundPolicy {
        self.sound_policy.read().await.clone()
    }

    pub async fn set_sound_policy(&self, policy: GiftSoundPolicy) {
        *self.sound_policy.write().await = policy;
    }

    pub async fn bindings(&self) -> Vec<GiftSoundBindingRecord> {
        self.bindings.read().await.values().cloned().collect()
    }

    pub async fn replace_bindings(&self, bindings: Vec<GiftSoundBindingRecord>) {
        *self.bindings.write().await = bindings
            .into_iter()
            .map(|binding| (binding.gift_id.clone(), binding))
            .collect();
    }

    pub async fn shutdown(&self) {
        let _ = self.shutdown.send(true);
        self.wake.notify_waiters();
        if let Some(worker) = self.worker.lock().await.take() {
            let _ = worker.await;
        }
    }
}

impl GiftWorker {
    async fn run(self, mut shutdown: watch::Receiver<bool>) {
        loop {
            let next = {
                let mut state = self.state.lock().await;
                let next = state.pending.pop_front();
                state.snapshot.pending_count = state.pending.len();
                next
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
            let gift = item.gift;
            let config = match item.config_override {
                Some(config) => config,
                None => self.notify_config.read().await.clone(),
            };
            let is_high_value = gift.value_minor >= config.high_value_threshold_minor;
            if config.is_enabled && is_high_value {
                let notification_failed = if config.methods.system_notification {
                    self.notifications
                        .show(NotificationRequest {
                            title: "礼物提醒".into(),
                            body: format!(
                                "{} 送出 {} ×{}",
                                gift.username, gift.gift_name, gift.quantity
                            ),
                            image_url: gift.gift_icon_url.clone(),
                        })
                        .await
                        .is_err()
                } else {
                    false
                };
                if config.methods.local_popup || notification_failed {
                    let mut state = self.state.lock().await;
                    state.snapshot.popup_items.push_back(gift.clone());
                    while state.snapshot.popup_items.len() > 20 {
                        state.snapshot.popup_items.pop_front();
                    }
                }
            }
            if config.methods.sound {
                let global_volume = self.sound_policy.read().await.global_volume;
                let binding = self.bindings.read().await.get(&gift.gift_id).cloned();
                let playback = if let Some(binding) = binding.filter(|binding| binding.is_enabled) {
                    if let Some(source) = binding.sound_path {
                        let volume = (binding.volume * global_volume).min(1.0);
                        match self.audio.play(AudioRequest { source, volume }).await {
                            Ok(playback) => Some(playback),
                            Err(error) => {
                                log::warn!("gift sound binding is degraded: {}", error.code);
                                self.audio
                                    .play_alert(global_volume)
                                    .await
                                    .map_err(|error| {
                                        log::warn!(
                                            "default gift alert sound is degraded: {}",
                                            error.code
                                        )
                                    })
                                    .ok()
                            }
                        }
                    } else {
                        self.audio
                            .play_alert(global_volume)
                            .await
                            .map_err(|error| {
                                log::warn!("default gift alert sound is degraded: {}", error.code)
                            })
                            .ok()
                    }
                } else {
                    self.audio
                        .play_alert(global_volume)
                        .await
                        .map_err(|error| {
                            log::warn!("default gift alert sound is degraded: {}", error.code)
                        })
                        .ok()
                };
                if let Some(playback) = playback {
                    while !playback.is_finished() && !*shutdown.borrow() {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                    if *shutdown.borrow() {
                        playback.stop();
                    }
                }
            }
        }
    }
}
