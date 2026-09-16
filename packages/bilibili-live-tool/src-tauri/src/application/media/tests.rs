use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex as StdMutex,
    },
};

use tokio::sync::Mutex;

use super::*;
use crate::domain::{
    live_data::{
        AudienceIdentity, CaptainAction, CaptainLevel, IdentityTag, LiveEvent, LiveEventBase,
        RoomId, UserUid,
    },
    media::{MediaSoundBinding, RecordingState, TtsIdentityKind},
};

struct FinishedPlayback;

impl AudioPlayback for FinishedPlayback {
    fn stop(&self) {}

    fn is_finished(&self) -> bool {
        true
    }
}

#[derive(Default)]
struct RecordingAudio {
    sources: Mutex<Vec<PathBuf>>,
}

#[async_trait::async_trait]
impl AudioOutputPort for RecordingAudio {
    async fn play(&self, request: AudioRequest) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
        self.sources.lock().await.push(request.source);
        Ok(Box::new(FinishedPlayback))
    }
}

struct FailingWindow;

#[async_trait::async_trait]
impl EffectWindowPort for FailingWindow {
    async fn ensure(&self, _: &EffectWindowSpec) -> Result<(), MediaIoError> {
        Err(MediaIoError::unavailable("effect window unavailable"))
    }

    async fn show(&self, _: &str) -> Result<(), MediaIoError> {
        Ok(())
    }

    async fn close(&self, _: &str) -> Result<(), MediaIoError> {
        Ok(())
    }
}

struct NoopWindow;

#[async_trait::async_trait]
impl EffectWindowPort for NoopWindow {
    async fn ensure(&self, _: &EffectWindowSpec) -> Result<(), MediaIoError> {
        Ok(())
    }

    async fn show(&self, _: &str) -> Result<(), MediaIoError> {
        Ok(())
    }

    async fn close(&self, _: &str) -> Result<(), MediaIoError> {
        Ok(())
    }
}

struct NoopNotification;

#[async_trait::async_trait]
impl NotificationPort for NoopNotification {
    async fn permission(&self) -> Result<NotificationPermission, MediaIoError> {
        Ok(NotificationPermission::Granted)
    }

    async fn request_permission(&self) -> Result<NotificationPermission, MediaIoError> {
        Ok(NotificationPermission::Granted)
    }

    async fn show(&self, _: NotificationRequest) -> Result<(), MediaIoError> {
        Ok(())
    }
}

struct RecordingSpeech {
    fails: bool,
    texts: Mutex<Vec<String>>,
}

impl RecordingSpeech {
    fn new(fails: bool) -> Self {
        Self {
            fails,
            texts: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait::async_trait]
impl SpeechEnginePort for RecordingSpeech {
    async fn voices(&self) -> Result<Vec<SpeechVoice>, MediaIoError> {
        Ok(Vec::new())
    }

    async fn speak(&self, request: SpeechRequest) -> Result<(), MediaIoError> {
        self.texts.lock().await.push(request.text);
        if self.fails {
            Err(MediaIoError::unavailable("speech unavailable"))
        } else {
            Ok(())
        }
    }

    async fn stop(&self) -> Result<(), MediaIoError> {
        Ok(())
    }
}

struct FakeRecorderProcess {
    stops: Arc<AtomicUsize>,
    waits: Arc<AtomicUsize>,
}

#[async_trait::async_trait]
impl RecorderProcess for FakeRecorderProcess {
    async fn stop(&mut self) -> Result<(), MediaIoError> {
        self.stops.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    async fn wait(&mut self) -> Result<RecorderExit, MediaIoError> {
        self.waits.fetch_add(1, Ordering::SeqCst);
        Ok(RecorderExit {
            success: true,
            code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
        })
    }
}

struct RetryingRecorderPort {
    attempts: AtomicUsize,
    fail_before: usize,
    stops: Arc<AtomicUsize>,
    waits: Arc<AtomicUsize>,
}

#[async_trait::async_trait]
impl RecorderProcessPort for RetryingRecorderPort {
    async fn spawn(&self, _: RecorderLaunch) -> Result<Box<dyn RecorderProcess>, MediaIoError> {
        let attempt = self.attempts.fetch_add(1, Ordering::SeqCst);
        if attempt < self.fail_before {
            return Err(MediaIoError::operation("spawn failed", true));
        }
        Ok(Box::new(FakeRecorderProcess {
            stops: self.stops.clone(),
            waits: self.waits.clone(),
        }))
    }
}

struct FakeHotkeyRegistration {
    unregistered: Arc<AtomicBool>,
}

impl HotkeyRegistration for FakeHotkeyRegistration {
    fn unregister(&mut self) -> Result<(), MediaIoError> {
        self.unregistered.store(true, Ordering::SeqCst);
        Ok(())
    }
}

struct FakeHotkeyPort {
    callback: Arc<StdMutex<Option<HotkeyCallback>>>,
    unregistered: Arc<AtomicBool>,
}

impl HotkeyPort for FakeHotkeyPort {
    fn register(
        &self,
        _: &str,
        callback: HotkeyCallback,
    ) -> Result<Box<dyn HotkeyRegistration>, MediaIoError> {
        self.callback.lock().unwrap().replace(callback);
        Ok(Box::new(FakeHotkeyRegistration {
            unregistered: self.unregistered.clone(),
        }))
    }
}

fn event_base(id: &str, occurred_at: i64, tags: Vec<IdentityTag>) -> LiveEventBase {
    LiveEventBase {
        event_id: id.into(),
        room_id: RoomId::parse("100").unwrap(),
        occurred_at,
        audience: AudienceIdentity {
            user_uid: UserUid::parse("200").unwrap(),
            username: "用户A".into(),
            user_level: None,
            medal: None,
            tags,
        },
    }
}

fn captain_event(id: &str, action: CaptainAction) -> LiveEvent {
    LiveEvent::Captain {
        base: event_base(id, 10_000, vec![IdentityTag::Captain]),
        action,
        level: CaptainLevel::Captain,
        value_minor: 19_800,
    }
}

fn danmu_event(id: &str, occurred_at: i64, tags: Vec<IdentityTag>) -> LiveEvent {
    LiveEvent::Danmu {
        base: event_base(id, occurred_at, tags),
        content: format!("内容{id}"),
    }
}

fn gift_event(id: &str) -> LiveEvent {
    LiveEvent::Gift {
        base: event_base(id, 10_000, Vec::new()),
        gift_id: "gift-1".into(),
        gift_name: "礼物".into(),
        quantity: 2,
        value_minor: 20_000,
    }
}

#[tokio::test(start_paused = true)]
async fn captain_effects_are_fifo_and_keep_a_local_fallback() {
    let audio = Arc::new(RecordingAudio::default());
    let mut config = default_captain_effect_config();
    config.duration_seconds = 3;
    config.first_sound = Some(MediaSoundBinding {
        binding_id: "first".into(),
        file_name: "first.mp3".into(),
        source: PathBuf::from("first.mp3"),
    });
    config.renewal_sound = Some(MediaSoundBinding {
        binding_id: "renewal".into(),
        file_name: "renewal.mp3".into(),
        source: PathBuf::from("renewal.mp3"),
    });
    let runtime = EffectRuntime::start(config, audio.clone(), Arc::new(FailingWindow)).await;

    runtime
        .enqueue(&captain_event("captain-1", CaptainAction::Purchase))
        .await;
    runtime
        .enqueue(&captain_event("captain-2", CaptainAction::Renewal))
        .await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    assert_eq!(
        runtime.snapshot().await.fallback_current.unwrap().event_id,
        "captain-1"
    );

    tokio::time::advance(std::time::Duration::from_secs(3)).await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    tokio::time::advance(std::time::Duration::from_secs(3)).await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    assert_eq!(
        *audio.sources.lock().await,
        vec![PathBuf::from("first.mp3"), PathBuf::from("renewal.mp3")]
    );
    assert_eq!(runtime.snapshot().await, EffectRuntimeSnapshot::default());
    runtime.shutdown().await;
}

#[tokio::test]
async fn tts_prioritizes_events_falls_back_and_preserves_paused_items() {
    let system = Arc::new(RecordingSpeech::new(true));
    let cloud = Arc::new(RecordingSpeech::new(true));
    let local = Arc::new(RecordingSpeech::new(false));
    let mut config = default_tts_config();
    config.is_enabled = true;
    let runtime = TtsRuntime::start(config.clone(), system, cloud, local.clone()).await;
    runtime.set_paused(true).await;

    runtime
        .enqueue_event(&danmu_event("danmu", 10_001, Vec::new()))
        .await;
    runtime.enqueue_event(&gift_event("gift")).await;
    runtime
        .enqueue_event(&captain_event("captain", CaptainAction::Purchase))
        .await;
    assert_eq!(runtime.snapshot().await.pending_count, 3);
    runtime.set_paused(false).await;
    for _ in 0..100 {
        if local.texts.lock().await.len() == 3 {
            break;
        }
        tokio::task::yield_now().await;
    }
    let texts = local.texts.lock().await.clone();
    assert!(texts[0].contains("舰长"));
    assert!(texts[1].contains("礼物"));
    assert!(texts[2].contains("内容danmu"));
    let snapshot = runtime.snapshot().await;
    assert_eq!(snapshot.active_engine, ActiveSpeechEngine::LocalFallback);
    assert_eq!(snapshot.fallback_reason_code, Some("tts_engine_fallback"));

    runtime.set_paused(true).await;
    runtime
        .enqueue_event(&danmu_event("pending", 10_002, Vec::new()))
        .await;
    assert_eq!(runtime.snapshot().await.pending_count, 1);
    runtime.clear_pending().await;
    assert_eq!(runtime.snapshot().await.pending_count, 0);
    config.identity_kinds = vec![TtsIdentityKind::Ordinary];
    runtime.set_config(config).await;
    runtime
        .enqueue_event(&danmu_event("vip", 10_003, vec![IdentityTag::Vip]))
        .await;
    assert_eq!(runtime.snapshot().await.pending_count, 0);
    runtime.shutdown().await;
}

#[tokio::test]
async fn recording_retries_marks_heat_and_stops_at_the_disk_waterline() {
    let stops = Arc::new(AtomicUsize::new(0));
    let waits = Arc::new(AtomicUsize::new(0));
    let process_port = Arc::new(RetryingRecorderPort {
        attempts: AtomicUsize::new(0),
        fail_before: 2,
        stops: stops.clone(),
        waits: waits.clone(),
    });
    let mut config = default_recording_config();
    config.danmaku_threshold = 2;
    let runtime = RecordingRuntime::new(config.clone(), process_port.clone());
    let callback = Arc::new(StdMutex::new(None));
    let unregistered = Arc::new(AtomicBool::new(false));
    runtime
        .register_hotkey(&FakeHotkeyPort {
            callback: callback.clone(),
            unregistered: unregistered.clone(),
        })
        .await
        .unwrap();
    runtime
        .start(
            "100",
            RecorderLaunch {
                executable: PathBuf::from("ffmpeg"),
                arguments: vec!["-version".into()],
                working_directory: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(process_port.attempts.load(Ordering::SeqCst), 3);
    assert_eq!(runtime.snapshot().await.state, RecordingState::Recording);

    assert!(runtime
        .consume_event(&danmu_event("heat-1", 10_000, Vec::new()))
        .await
        .is_none());
    assert!(runtime
        .consume_event(&danmu_event("heat-2", 10_001, Vec::new()))
        .await
        .is_some());
    callback.lock().unwrap().as_ref().unwrap()();
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    assert_eq!(runtime.snapshot().await.highlights.len(), 2);

    assert!(runtime
        .update_disk_remaining(config.waterline_bytes - 1)
        .await
        .unwrap());
    assert_eq!(runtime.snapshot().await.state, RecordingState::Idle);
    assert_eq!(stops.load(Ordering::SeqCst), 1);
    assert_eq!(waits.load(Ordering::SeqCst), 1);
    runtime.shutdown().await;
    assert!(unregistered.load(Ordering::SeqCst));
}

#[tokio::test(start_paused = true)]
async fn media_state_routes_an_accepted_event_only_once() {
    let audio = Arc::new(RecordingAudio::default());
    let effects = EffectRuntime::start(
        default_captain_effect_config(),
        audio.clone(),
        Arc::new(NoopWindow),
    )
    .await;
    let gifts = GiftMediaRuntime::start(
        default_gift_notify_config(),
        default_gift_sound_policy(),
        Vec::new(),
        audio,
        Arc::new(NoopNotification),
    )
    .await;
    let unavailable = Arc::new(RecordingSpeech::new(true));
    let tts = TtsRuntime::start(
        default_tts_config(),
        unavailable.clone(),
        unavailable.clone(),
        unavailable,
    )
    .await;
    let recorder = Arc::new(RetryingRecorderPort {
        attempts: AtomicUsize::new(0),
        fail_before: usize::MAX,
        stops: Arc::new(AtomicUsize::new(0)),
        waits: Arc::new(AtomicUsize::new(0)),
    });
    let recording = RecordingRuntime::new(default_recording_config(), recorder);
    let state = MediaState::new(effects, gifts, tts, recording);
    let event = captain_event("same-event", CaptainAction::Purchase);

    state.consume(&event).await;
    state.consume(&event).await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    let snapshot = state.effects().snapshot().await;
    assert_eq!(
        usize::from(snapshot.current.is_some()) + snapshot.pending_count,
        1
    );
    state.shutdown().await;
}
