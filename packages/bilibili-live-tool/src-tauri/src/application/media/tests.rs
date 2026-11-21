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
    media::{MediaSoundBinding, RecordingState, TtsIdentityKind, TtsSelectedEngine},
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
    alert_volumes: Mutex<Vec<f64>>,
}

#[async_trait::async_trait]
impl AudioOutputPort for RecordingAudio {
    async fn play(&self, request: AudioRequest) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
        self.sources.lock().await.push(request.source);
        Ok(Box::new(FinishedPlayback))
    }

    async fn play_alert(&self, volume: f64) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
        self.alert_volumes.lock().await.push(volume);
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

#[derive(Default)]
struct RecordingWindow {
    specs: Mutex<Vec<EffectWindowSpec>>,
}

#[async_trait::async_trait]
impl EffectWindowPort for RecordingWindow {
    async fn ensure(&self, spec: &EffectWindowSpec) -> Result<(), MediaIoError> {
        self.specs.lock().await.push(spec.clone());
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

#[derive(Default)]
struct RecordingNotification {
    requests: Mutex<Vec<NotificationRequest>>,
}

#[async_trait::async_trait]
impl NotificationPort for RecordingNotification {
    async fn permission(&self) -> Result<NotificationPermission, MediaIoError> {
        Ok(NotificationPermission::Granted)
    }

    async fn request_permission(&self) -> Result<NotificationPermission, MediaIoError> {
        Ok(NotificationPermission::Granted)
    }

    async fn show(&self, request: NotificationRequest) -> Result<(), MediaIoError> {
        self.requests.lock().await.push(request);
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
        gift_icon_url: Some("https://i0.hdslb.com/bfs/live/gift.png".into()),
        quantity: 2,
        value_minor: 20_000,
    }
}

#[tokio::test(start_paused = true)]
async fn gift_system_notification_contains_gift_identity_and_image() {
    let notifications = Arc::new(RecordingNotification::default());
    let runtime = GiftMediaRuntime::start(
        default_gift_notify_config(),
        default_gift_sound_policy(),
        Vec::new(),
        Arc::new(RecordingAudio::default()),
        notifications.clone(),
    )
    .await;

    runtime
        .consume(&gift_event("gift-notification-content"))
        .await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }

    let requests = notifications.requests.lock().await;
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].title, "礼物提醒");
    assert_eq!(requests[0].body, "用户A 送出 礼物 ×2");
    assert_eq!(
        requests[0].image_url.as_deref(),
        Some("https://i0.hdslb.com/bfs/live/gift.png")
    );
    drop(requests);
    runtime.shutdown().await;
}

#[tokio::test(start_paused = true)]
async fn gift_sound_uses_the_exact_binding_before_the_default_alert() {
    let audio = Arc::new(RecordingAudio::default());
    let runtime = GiftMediaRuntime::start(
        default_gift_notify_config(),
        default_gift_sound_policy(),
        vec![GiftSoundBindingRecord {
            gift_id: "gift-1".into(),
            gift_name: "礼物".into(),
            sound_path: Some(PathBuf::from("gift-1.mp3")),
            volume: 0.5,
            is_enabled: true,
        }],
        audio.clone(),
        Arc::new(NoopNotification),
    )
    .await;

    runtime.consume(&gift_event("custom-gift-sound")).await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }

    assert_eq!(
        *audio.sources.lock().await,
        vec![PathBuf::from("gift-1.mp3")]
    );
    assert!(audio.alert_volumes.lock().await.is_empty());
    runtime.shutdown().await;
}

#[tokio::test(start_paused = true)]
async fn gift_sound_falls_back_to_the_default_alert_without_a_binding() {
    let audio = Arc::new(RecordingAudio::default());
    let runtime = GiftMediaRuntime::start(
        default_gift_notify_config(),
        default_gift_sound_policy(),
        Vec::new(),
        audio.clone(),
        Arc::new(NoopNotification),
    )
    .await;

    runtime
        .preview(default_gift_notify_config(), "preview-default-sound".into())
        .await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }

    assert!(audio.sources.lock().await.is_empty());
    assert_eq!(*audio.alert_volumes.lock().await, vec![0.8]);
    runtime.shutdown().await;
}

#[tokio::test(start_paused = true)]
async fn gift_sound_does_not_play_when_the_method_is_disabled() {
    let audio = Arc::new(RecordingAudio::default());
    let mut config = default_gift_notify_config();
    config.methods.sound = false;
    let runtime = GiftMediaRuntime::start(
        config,
        default_gift_sound_policy(),
        Vec::new(),
        audio.clone(),
        Arc::new(NoopNotification),
    )
    .await;

    runtime.consume(&gift_event("disabled-gift-sound")).await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }

    assert!(audio.sources.lock().await.is_empty());
    assert!(audio.alert_volumes.lock().await.is_empty());
    runtime.shutdown().await;
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

#[tokio::test(start_paused = true)]
async fn captain_effect_uses_the_configured_window_size_and_position() {
    let window = Arc::new(RecordingWindow::default());
    let mut config = default_captain_effect_config();
    config.window_width = 720;
    config.window_height = 360;
    config.position = crate::domain::media::MediaPosition::BottomRight;
    let runtime = EffectRuntime::start(
        config.clone(),
        Arc::new(RecordingAudio::default()),
        window.clone(),
    )
    .await;

    runtime.preview(config, "preview-window-spec".into()).await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }

    assert_eq!(
        window.specs.lock().await.as_slice(),
        [EffectWindowSpec {
            label: "effect-captain".into(),
            width: 720,
            height: 360,
            position: crate::domain::media::MediaPosition::BottomRight,
        }]
    );
    runtime.shutdown().await;
}

#[tokio::test(start_paused = true)]
async fn captain_effect_preview_exposes_the_active_payload_until_the_effect_finishes() {
    let mut config = default_captain_effect_config();
    config.preset_id = "golden-premium".into();
    config.duration_seconds = 3;
    let runtime = EffectRuntime::start(
        config.clone(),
        Arc::new(RecordingAudio::default()),
        Arc::new(NoopWindow),
    )
    .await;

    runtime
        .preview(config.clone(), "preview-active-effect".into())
        .await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }

    let active = runtime.active_effect().await.expect("preview is active");
    assert_eq!(active.item.event_id, "preview-active-effect");
    assert_eq!(active.item.username, "用户A");
    assert_eq!(active.config, config);
    assert!(active.is_preview);

    tokio::time::advance(std::time::Duration::from_secs(3)).await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    assert!(runtime.active_effect().await.is_none());
    runtime.shutdown().await;
}

#[tokio::test(start_paused = true)]
async fn captain_effect_active_payload_preserves_the_live_audience() {
    let runtime = EffectRuntime::start(
        default_captain_effect_config(),
        Arc::new(RecordingAudio::default()),
        Arc::new(NoopWindow),
    )
    .await;
    let event = captain_event("live-active-effect", CaptainAction::Renewal);

    runtime.enqueue(&event).await;
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }

    let active = runtime
        .active_effect()
        .await
        .expect("live effect is active");
    let audience = active.item.audience.expect("live audience is retained");
    assert_eq!(audience.user_uid.as_str(), "200");
    assert_eq!(audience.username, "用户A");
    assert_eq!(active.item.action, CaptainAction::Renewal);
    assert!(!active.is_preview);
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
async fn an_unavailable_cloud_engine_falls_back_to_the_system_voice() {
    // Mirrors the Qwen3-TTS deployment: the "cloud" slot stops answering while
    // the operator still expects every broadcast to be spoken.
    let system = Arc::new(RecordingSpeech::new(false));
    let cloud = Arc::new(RecordingSpeech::new(true));
    let local = Arc::new(RecordingSpeech::new(true));
    let mut config = default_tts_config();
    config.is_enabled = true;
    config.selected_engine = TtsSelectedEngine::Cloud;
    let runtime = TtsRuntime::start(config.clone(), system.clone(), cloud.clone(), local).await;

    runtime
        .enqueue_event(&danmu_event("danmu", 20_001, Vec::new()))
        .await;
    for _ in 0..100 {
        if !system.texts.lock().await.is_empty() {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(system.texts.lock().await.len(), 1);
    assert_eq!(cloud.texts.lock().await.len(), 1);
    let snapshot = runtime.snapshot().await;
    assert_eq!(snapshot.active_engine, ActiveSpeechEngine::System);
    assert_eq!(snapshot.fallback_reason_code, Some("tts_engine_fallback"));

    // Rolling back to the system engine must not touch the cloud engine again.
    config.selected_engine = TtsSelectedEngine::System;
    runtime.set_config(config).await;
    runtime
        .enqueue_event(&danmu_event("system", 20_002, Vec::new()))
        .await;
    for _ in 0..100 {
        if system.texts.lock().await.len() == 2 {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(system.texts.lock().await.len(), 2);
    assert_eq!(cloud.texts.lock().await.len(), 1);
    assert_eq!(
        runtime.snapshot().await.fallback_reason_code,
        None,
        "an explicit system selection is not a fallback"
    );
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

/// A port whose recorder process has already exited, used to exercise the
/// reconnect primitives without spawning FFmpeg.
struct ExitedRecorderPort;

struct ExitedRecorderProcess;

#[async_trait::async_trait]
impl RecorderProcess for ExitedRecorderProcess {
    async fn stop(&mut self) -> Result<(), MediaIoError> {
        Ok(())
    }

    async fn wait(&mut self) -> Result<RecorderExit, MediaIoError> {
        Ok(RecorderExit {
            success: true,
            code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
        })
    }

    fn has_exited(&mut self) -> bool {
        true
    }
}

#[async_trait::async_trait]
impl RecorderProcessPort for ExitedRecorderPort {
    async fn spawn(&self, _: RecorderLaunch) -> Result<Box<dyn RecorderProcess>, MediaIoError> {
        Ok(Box::new(ExitedRecorderProcess))
    }
}

fn launch() -> RecorderLaunch {
    RecorderLaunch {
        executable: PathBuf::from("ffmpeg.exe"),
        arguments: Vec::new(),
        working_directory: None,
    }
}

#[tokio::test]
async fn reconnect_primitives_detect_a_drop_and_keep_the_session_recordable() {
    let runtime = RecordingRuntime::new(default_recording_config(), Arc::new(ExitedRecorderPort));
    runtime.start("1", launch()).await.unwrap();

    // The watchdog relies on this probe to notice a dropped stream.
    assert!(runtime.has_exited().await);

    // A reconnect swaps the process but must not reset the session.
    runtime.replace_process(launch()).await.unwrap();
    assert_eq!(runtime.snapshot().await.state, RecordingState::Recording);

    // Exhausting the retry budget surfaces a stable code (PRD 9.12).
    runtime.fail("recording_stream_lost").await;
    let snapshot = runtime.snapshot().await;
    assert_eq!(snapshot.state, RecordingState::Error);
    assert_eq!(snapshot.failure_code, Some("recording_stream_lost"));
}

#[tokio::test]
async fn reconnect_is_refused_once_the_session_is_no_longer_recording() {
    let runtime = RecordingRuntime::new(default_recording_config(), Arc::new(ExitedRecorderPort));
    runtime.start("1", launch()).await.unwrap();
    runtime.fail("recording_stream_lost").await;

    assert!(runtime.replace_process(launch()).await.is_err());
}
