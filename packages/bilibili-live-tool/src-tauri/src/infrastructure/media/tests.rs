use std::{path::PathBuf, sync::Arc};

use crate::{
    application::{
        media::{
            AudioOutputPort, AudioRequest, ByteRange, EffectWindowPort, EffectWindowSpec,
            HotkeyPort, MediaFilePort, NotificationPort, PreviewPort, RecorderLaunch,
            RecorderProcessPort, SpeechEnginePort,
        },
        paths::PathPurpose,
    },
    domain::media::{
        CaptainEffectConfig, FfmpegSource, GiftNotifyConfig, GiftNotifyMethods, GiftSoundPolicy,
        MediaPosition, RecordingConfig, RecordingDirectoryGrant, RecordingFormat, RecordingQuality,
        TtsConfig, TtsEventKind, TtsIdentityKind, TtsSelectedEngine, TtsTemplates,
    },
    infrastructure::{database::open_database, path_grants::PathGrantStore},
};

use super::{
    compensated_delete, compensated_move, GiftSoundBindingRecord, GiftSoundDocument,
    GiftSoundFileStore, HighlightRecord, HighlightTriggerType, LocalFfmpegProcessPort,
    MediaRepositoryError, PreviewRegistry, RecordingItemState, RecordingRecord,
    SqliteMediaRepository, UnavailableAudioOutput, UnavailableEffectWindowPort,
    UnavailableHotkeyPort, UnavailableNotificationPort, UnavailableRecorderProcessPort,
    UnavailableSpeechEngine,
};

fn captain_config() -> CaptainEffectConfig {
    CaptainEffectConfig {
        preset_id: "golden-rain".into(),
        duration_seconds: 5,
        position: MediaPosition::TopRight,
        window_width: 560,
        window_height: 280,
        primary_color: "#FFD700".into(),
        secondary_color: "#FFA500".into(),
        first_sound: None,
        renewal_sound: None,
        combo_sound: None,
        volume: 0.7,
        combo_enabled: true,
        combo_window_seconds: 10,
    }
}

fn gift_notify_config() -> GiftNotifyConfig {
    GiftNotifyConfig {
        is_enabled: true,
        high_value_threshold_minor: 10_000,
        methods: GiftNotifyMethods {
            system_notification: true,
            local_popup: true,
            sound: true,
        },
        combo_enabled: true,
        combo_window_seconds: 10,
        popup_position: MediaPosition::TopRight,
        popup_duration_seconds: 5,
    }
}

fn tts_config() -> TtsConfig {
    TtsConfig {
        is_enabled: true,
        selected_engine: TtsSelectedEngine::System,
        selected_voice_id: Some("voice-1".into()),
        rate: 1.0,
        volume: 0.8,
        event_kinds: vec![TtsEventKind::Danmu, TtsEventKind::Gift],
        identity_kinds: vec![TtsIdentityKind::Captain, TtsIdentityKind::Admin],
        templates: TtsTemplates {
            danmu: "{用户}说：{内容}".into(),
            gift: "{用户}送出{礼物}×{数量}".into(),
            captain: "{用户}开通{等级}".into(),
        },
    }
}

fn recording_config() -> RecordingConfig {
    RecordingConfig {
        format: RecordingFormat::Mp4,
        quality: RecordingQuality::Original,
        directory: Some(RecordingDirectoryGrant {
            grant_id: "recording-grant".into(),
            display_path: "D:/recordings".into(),
        }),
        segmentation_enabled: true,
        segment_duration_seconds: 3_600,
        auto_record: false,
        danmaku_threshold: 30,
        highlight_before_seconds: 30,
        highlight_after_seconds: 30,
        quota_bytes: 10 * 1024 * 1024 * 1024,
        waterline_bytes: 5 * 1024 * 1024 * 1024,
        ffmpeg_source: FfmpegSource::Local,
        shortcut: "F9".into(),
    }
}

#[tokio::test]
async fn media_configs_round_trip_and_corrupt_json_is_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("app.db"))
        .await
        .unwrap();
    let repository = SqliteMediaRepository::new(pool.clone());
    let sound_policy = GiftSoundPolicy {
        global_volume: 0.65,
        queue_capacity_per_second: 10,
    };

    repository
        .save_captain_config(&captain_config())
        .await
        .unwrap();
    repository
        .save_gift_notify_config(&gift_notify_config())
        .await
        .unwrap();
    repository
        .save_gift_sound_policy(&sound_policy)
        .await
        .unwrap();
    repository
        .save_recording_config(&recording_config())
        .await
        .unwrap();
    repository.save_tts_config(&tts_config()).await.unwrap();

    assert_eq!(
        repository.load_captain_config().await.unwrap(),
        Some(captain_config())
    );
    assert_eq!(
        repository.load_gift_notify_config().await.unwrap(),
        Some(gift_notify_config())
    );
    assert_eq!(
        repository.load_gift_sound_policy().await.unwrap(),
        Some(sound_policy)
    );
    assert_eq!(
        repository.load_recording_config().await.unwrap(),
        Some(recording_config())
    );
    assert_eq!(
        repository.load_tts_config().await.unwrap(),
        Some(tts_config())
    );

    sqlx::query("UPDATE t_app_setting SET value = '{broken' WHERE key = 'media.captain_effect'")
        .execute(&pool)
        .await
        .unwrap();
    assert!(matches!(
        repository.load_captain_config().await,
        Err(MediaRepositoryError::InvalidData(_))
    ));
}

#[tokio::test]
async fn media_tables_support_round_trip_and_transactional_gift_import() {
    let directory = tempfile::tempdir().unwrap();
    let pool = open_database(&directory.path().join("app.db"))
        .await
        .unwrap();
    let repository = SqliteMediaRepository::new(pool.clone());
    let sound = GiftSoundBindingRecord {
        gift_id: "31036".into(),
        gift_name: "小花花".into(),
        sound_path: Some(PathBuf::from("D:/sounds/flower.mp3")),
        volume: 0.7,
        is_enabled: true,
    };
    repository.upsert_gift_sound(&sound).await.unwrap();
    assert_eq!(
        repository.list_gift_sounds().await.unwrap(),
        vec![sound.clone()]
    );

    let recording = RecordingRecord {
        id: 0,
        room_id: "123".into(),
        file_path: PathBuf::from("D:/recordings/live.mp4"),
        format: RecordingFormat::Mp4,
        quality: RecordingQuality::Original,
        started_at: 1_700_000_000_000,
        ended_at: Some(1_700_000_060_000),
        size_bytes: 1024,
        duration_seconds: 60,
        status: RecordingItemState::Completed,
    };
    let recording = repository.insert_recording(&recording).await.unwrap();
    let highlight = HighlightRecord {
        id: 0,
        recording_id: recording.id,
        room_id: "123".into(),
        position_seconds: 45,
        trigger_type: HighlightTriggerType::Manual,
        clip_duration_seconds: 60,
        file_path: Some(PathBuf::from("D:/recordings/highlight.mp4")),
        created_at: 1_700_000_045_000,
        is_clip_available: true,
    };
    let highlight = repository.insert_highlight(&highlight).await.unwrap();
    assert_eq!(
        repository.list_recordings().await.unwrap(),
        vec![recording.clone()]
    );
    assert_eq!(
        repository.list_highlights(recording.id).await.unwrap(),
        vec![highlight]
    );

    sqlx::query(
        "CREATE TRIGGER reject_bad_gift BEFORE INSERT ON t_gift_sound WHEN NEW.gift_id = 'bad' BEGIN SELECT RAISE(ABORT, 'bad gift'); END",
    )
    .execute(&pool)
    .await
    .unwrap();
    let replacement = [
        GiftSoundBindingRecord {
            gift_id: "new".into(),
            ..sound.clone()
        },
        GiftSoundBindingRecord {
            gift_id: "bad".into(),
            ..sound.clone()
        },
    ];
    assert!(repository.replace_gift_sounds(&replacement).await.is_err());
    assert_eq!(repository.list_gift_sounds().await.unwrap(), vec![sound]);
}

#[tokio::test]
async fn gift_sound_files_enforce_purpose_revocation_and_versioned_documents() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("gift-sounds.json");
    tokio::fs::write(
        &input,
        r#"{"version":1,"bindings":[{"giftId":"31036","giftName":"小花花","fileName":"flower.mp3","volume":0.7,"isEnabled":true}]}"#,
    )
    .await
    .unwrap();
    let grants = Arc::new(PathGrantStore::new());
    let store = GiftSoundFileStore::new(grants.clone());
    let wrong = grants
        .grant_selection(Some(input.clone()), PathPurpose::GiftSoundFile)
        .unwrap()
        .unwrap();
    assert!(store.read_import(wrong.id()).await.is_err());

    let grant = grants
        .grant_selection(Some(input), PathPurpose::GiftSoundImport)
        .unwrap()
        .unwrap();
    let document = store.read_import(grant.id()).await.unwrap();
    assert_eq!(document.version, GiftSoundDocument::CURRENT_VERSION);
    assert_eq!(document.bindings[0].gift_id, "31036");
    grants.revoke(grant.id()).unwrap();
    assert!(store.read_import(grant.id()).await.is_err());

    let output = directory.path().join("gift-sounds-export.json");
    tokio::fs::write(&output, b"old value").await.unwrap();
    let output_grant = grants
        .grant_selection(Some(output.clone()), PathPurpose::GiftSoundExport)
        .unwrap()
        .unwrap();
    store
        .write_export(output_grant.id(), &document)
        .await
        .unwrap();
    let exported: GiftSoundDocument =
        serde_json::from_slice(&tokio::fs::read(output).await.unwrap()).unwrap();
    assert_eq!(exported, document);
}

#[tokio::test]
async fn compensated_delete_restores_the_file_when_database_update_fails() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("recording.mp4");
    tokio::fs::write(&path, b"media").await.unwrap();

    let result = compensated_delete(&path, || async {
        Err::<(), _>(MediaRepositoryError::InvalidData("forced failure"))
    })
    .await;

    assert!(result.is_err());
    assert_eq!(tokio::fs::read(path).await.unwrap(), b"media");
}

#[tokio::test]
async fn compensated_move_restores_the_original_name_when_database_update_fails() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("recording.mp4");
    let target = directory.path().join("renamed.mp4");
    tokio::fs::write(&source, b"media").await.unwrap();

    let result = compensated_move(&source, &target, || async {
        Err::<(), _>(MediaRepositoryError::InvalidData("forced failure"))
    })
    .await;

    assert!(result.is_err());
    assert_eq!(tokio::fs::read(source).await.unwrap(), b"media");
    assert!(!target.exists());
}

#[tokio::test]
async fn media_file_port_stages_restores_moves_and_exports_owned_files() {
    let directory = tempfile::tempdir().unwrap();
    let grants = Arc::new(PathGrantStore::new());
    let store = GiftSoundFileStore::new(grants);
    let original = directory.path().join("recording.mp4");
    let renamed = directory.path().join("renamed.mp4");
    let exported = directory.path().join("exported.mp4");
    tokio::fs::write(&original, b"media").await.unwrap();

    store.move_file(&original, &renamed).await.unwrap();
    assert!(!original.exists());
    store.export_file(&renamed, &exported).await.unwrap();
    assert_eq!(tokio::fs::read(&exported).await.unwrap(), b"media");
    assert_eq!(store.file_size(&exported).await.unwrap(), 5);

    let staged = store
        .stage_delete(std::slice::from_ref(&renamed))
        .await
        .unwrap();
    assert!(!renamed.exists());
    store.restore_staged(&staged).await.unwrap();
    assert_eq!(tokio::fs::read(&renamed).await.unwrap(), b"media");

    let staged = store
        .stage_delete(std::slice::from_ref(&renamed))
        .await
        .unwrap();
    store.finalize_staged(&staged).await.unwrap();
    assert!(!renamed.exists());
}

#[tokio::test]
async fn unavailable_adapters_report_stable_non_success_states() {
    let audio = UnavailableAudioOutput;
    let audio_error = audio
        .play(AudioRequest {
            source: PathBuf::from("missing.mp3"),
            volume: 0.7,
        })
        .await
        .err()
        .unwrap();
    assert_eq!(audio_error.code, "RECORD_UNAVAILABLE");

    assert_eq!(
        UnavailableSpeechEngine.voices().await.unwrap_err().code,
        "RECORD_UNAVAILABLE"
    );
    assert_eq!(
        UnavailableRecorderProcessPort
            .spawn(RecorderLaunch {
                executable: PathBuf::from("ffmpeg.exe"),
                arguments: Vec::new(),
                working_directory: None,
            })
            .await
            .err()
            .unwrap()
            .code,
        "RECORD_UNAVAILABLE"
    );
    assert_eq!(
        UnavailableNotificationPort
            .permission()
            .await
            .unwrap_err()
            .code,
        "RECORD_UNAVAILABLE"
    );
    assert_eq!(
        UnavailableEffectWindowPort
            .ensure(&EffectWindowSpec {
                label: "effect".into(),
                width: 480,
                height: 240,
                position: MediaPosition::Center,
            })
            .await
            .unwrap_err()
            .code,
        "RECORD_UNAVAILABLE"
    );
    assert_eq!(
        UnavailableHotkeyPort
            .register("F9", Arc::new(|| {}))
            .err()
            .unwrap()
            .code,
        "RECORD_UNAVAILABLE"
    );
}

#[tokio::test]
async fn preview_registry_enforces_owner_expiry_range_and_revocation() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("preview.mp4");
    tokio::fs::write(&path, b"0123456789").await.unwrap();
    let registry = PreviewRegistry::new(2);
    let first = registry.issue(path.clone(), "main", 1_000, 5_000).unwrap();
    let second = registry.issue(path, "main", 1_001, 5_000).unwrap();
    assert_ne!(first.token, second.token);
    assert!(!first.url.contains("preview.mp4"));

    let range = registry
        .read(
            &first.token,
            "main",
            Some(ByteRange {
                start: 2,
                end_inclusive: 5,
            }),
            2_000,
        )
        .await
        .unwrap();
    assert_eq!(range.bytes, b"2345");
    assert_eq!(range.total_size, 10);
    assert_eq!(range.range.start, 2);
    assert_eq!(
        registry
            .read(&first.token, "effect", None, 2_000)
            .await
            .unwrap_err()
            .code,
        "SYS_PERMISSION_DENIED"
    );
    assert_eq!(
        registry
            .read(&first.token, "main", None, 6_001)
            .await
            .unwrap_err()
            .code,
        "RECORD_INVALID_PARAM"
    );
    assert!(registry.revoke(&second.token, "main").unwrap());
    assert!(registry
        .read(&second.token, "main", None, 2_000)
        .await
        .is_err());
}

#[tokio::test]
async fn local_ffmpeg_adapter_rejects_a_missing_executable() {
    let error = LocalFfmpegProcessPort
        .spawn(RecorderLaunch {
            executable: PathBuf::from("definitely-missing-ffmpeg.exe"),
            arguments: Vec::new(),
            working_directory: None,
        })
        .await
        .err()
        .unwrap();
    assert_eq!(error.code, "RECORD_UNAVAILABLE");
}

#[cfg(windows)]
#[tokio::test]
async fn local_process_adapter_waits_for_a_real_child_exit() {
    let mut process = LocalFfmpegProcessPort
        .spawn(RecorderLaunch {
            executable: PathBuf::from(r"C:\Windows\System32\cmd.exe"),
            arguments: vec!["/C".into(), "echo ffmpeg-test 1>&2".into()],
            working_directory: None,
        })
        .await
        .unwrap();
    let exit = process.wait().await.unwrap();
    assert!(exit.success);
    assert!(exit.stderr.contains("ffmpeg-test"));
}

#[cfg(windows)]
#[tokio::test]
async fn local_process_adapter_stops_and_reaps_a_long_running_child() {
    let mut process = LocalFfmpegProcessPort
        .spawn(RecorderLaunch {
            executable: PathBuf::from(r"C:\Windows\System32\cmd.exe"),
            arguments: vec!["/C".into(), "ping -n 30 127.0.0.1 >NUL".into()],
            working_directory: None,
        })
        .await
        .unwrap();

    process.stop().await.unwrap();
    let exit = process.wait().await.unwrap();
    assert!(!exit.success);
}

/// Live end-to-end proof: real platform API + the exact argument vector the
/// domain builds + the real child-process port.
///
/// Ignored by default because it needs network access and an FFmpeg binary:
/// ```text
/// RECORDING_ROOM_ID=21452505 RECORDING_FFMPEG=<path to ffmpeg> \
///   cargo test --lib --features free records_a_live_room_end_to_end -- --ignored --nocapture
/// ```
#[tokio::test]
#[ignore = "requires network + ffmpeg; set RECORDING_ROOM_ID and RECORDING_FFMPEG"]
async fn records_a_live_room_end_to_end() {
    use crate::{
        application::media::PlayUrlResolver,
        domain::media::{recording_arguments, RecordingPlanInput},
    };
    use super::BilibiliPlayUrlResolver;

    let room_id = std::env::var("RECORDING_ROOM_ID").expect("RECORDING_ROOM_ID");
    let ffmpeg = std::env::var("RECORDING_FFMPEG").expect("RECORDING_FFMPEG");
    let directory = tempfile::tempdir().unwrap();

    let stream = BilibiliPlayUrlResolver::new()
        .resolve(&room_id, RecordingQuality::Original)
        .await
        .expect("stream must resolve for a live room");
    println!("resolved {} bytes of URL, qualities={:?}", stream.url.len(), stream.available_qualities);

    let output = directory.path().join("e2e.flv");
    let arguments = recording_arguments(&RecordingPlanInput {
        stream_url: &stream.url,
        output: &output,
        format: RecordingFormat::Flv,
        quality: RecordingQuality::Original,
    })
    .unwrap();

    let mut process = LocalFfmpegProcessPort
        .spawn(RecorderLaunch {
            executable: PathBuf::from(ffmpeg),
            arguments,
            working_directory: Some(directory.path().to_path_buf()),
        })
        .await
        .expect("ffmpeg must start");

    tokio::time::sleep(std::time::Duration::from_secs(8)).await;
    process.stop().await.unwrap();
    let exit = process.wait().await.unwrap();
    let bytes = std::fs::read(&output).unwrap_or_default();
    println!(
        "recorded {} bytes, header={:?}, exit={:?}\nstderr={}",
        bytes.len(),
        bytes.get(..3),
        exit.code,
        exit.stderr
    );
    assert!(!bytes.is_empty(), "no data was recorded; stderr={}", exit.stderr);
    assert_eq!(&bytes[..3], b"FLV", "output is not a playable FLV container");
}

/// PRD 9.12 requires a clip to be cut from the recent window of a recording.
/// This drives the real FFmpeg through the production one-shot port so the
/// seek accuracy of `-ss` on an FLV file is actually measured, not assumed.
///
/// ```text
/// RECORDING_FFMPEG=<path to ffmpeg> \
///   cargo test --lib --features free extracts_a_highlight_clip -- --ignored --nocapture
/// ```
#[tokio::test]
#[ignore = "requires ffmpeg; set RECORDING_FFMPEG"]
async fn extracts_a_highlight_clip_from_a_real_recording() {
    use crate::{
        application::media::RecorderProcessPort,
        domain::media::{highlight_clip_arguments, HighlightClipInput},
    };
    use super::LocalFfmpegProcessPort;

    let ffmpeg = std::env::var("RECORDING_FFMPEG").expect("RECORDING_FFMPEG");
    let executable = PathBuf::from(&ffmpeg);
    let directory = tempfile::tempdir().unwrap();
    let recording = directory.path().join("synthetic.flv");
    let clip = directory.path().join("synthetic-clip.flv");

    // A 70 second synthetic live-like stream: FLV video + AAC audio.
    let status = std::process::Command::new(&ffmpeg)
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-f",
            "lavfi",
            "-i",
            "testsrc=size=320x240:rate=10:duration=70",
            "-f",
            "lavfi",
            "-i",
            "sine=frequency=440:duration=70",
            "-c:v",
            "flv",
            "-c:a",
            "aac",
            "-y",
        ])
        .arg(&recording)
        .status()
        .expect("synthetic recording must be produced");
    assert!(status.success(), "synthetic recording generation failed");
    let recording_bytes = std::fs::metadata(&recording).unwrap().len();
    println!("synthetic recording: {recording_bytes} bytes");

    // Marker at 65s with a 60s look-behind and a 5s tail: start 10s, 65s long.
    let arguments = highlight_clip_arguments(&HighlightClipInput {
        source: &recording,
        output: &clip,
        position_seconds: 60,
        before_seconds: 60,
        after_seconds: 5,
    })
    .unwrap();
    LocalFfmpegProcessPort
        .run(&executable, &arguments)
        .await
        .expect("clip extraction must succeed");

    let clip_bytes = std::fs::read(&clip).unwrap_or_default();
    assert!(!clip_bytes.is_empty(), "clip extraction produced no data");
    assert_eq!(&clip_bytes[..3], b"FLV", "clip is not a playable FLV");

    // Measure the produced duration: a wrong seek shows up here.
    let probe = std::process::Command::new(&ffmpeg)
        .args(["-hide_banner", "-i"])
        .arg(&clip)
        .output()
        .expect("ffmpeg must report clip metadata");
    let stderr = String::from_utf8_lossy(&probe.stderr);
    let duration = stderr
        .split("Duration: ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .expect("clip duration must be reported");
    let parts: Vec<f64> = duration
        .split(':')
        .map(|part| part.trim().parse::<f64>().unwrap_or(0.0))
        .collect();
    let seconds = parts[0] * 3600.0 + parts[1] * 60.0 + parts[2];
    println!("clip: {} bytes, duration {duration} ({seconds:.2}s)", clip_bytes.len());
    assert!(
        (62.0..=66.0).contains(&seconds),
        "clip duration {seconds:.2}s is outside the expected 65s window"
    );
}
/// The production path cuts a clip out of a file FFmpeg is still appending to.
/// This test keeps the writer running while the clip is extracted, because an
/// in-progress FLV has no completed index and may seek differently.
///
/// ```text
/// RECORDING_FFMPEG=<path to ffmpeg> \
///   cargo test --lib --features free extracts_a_clip_while_recording -- --ignored --nocapture
/// ```
#[tokio::test]
#[ignore = "requires ffmpeg; set RECORDING_FFMPEG"]
async fn extracts_a_clip_while_the_recording_is_still_writing() {
    use crate::{
        application::media::RecorderProcessPort,
        domain::media::{highlight_clip_arguments, HighlightClipInput},
    };
    use super::LocalFfmpegProcessPort;

    let ffmpeg = std::env::var("RECORDING_FFMPEG").expect("RECORDING_FFMPEG");
    let executable = PathBuf::from(&ffmpeg);
    let directory = tempfile::tempdir().unwrap();
    let recording = directory.path().join("live.flv");
    let clip = directory.path().join("live-clip.flv");

    let mut writer = std::process::Command::new(&ffmpeg)
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-re",
            "-f",
            "lavfi",
            "-i",
            "testsrc=size=320x240:rate=10:duration=130",
            "-f",
            "lavfi",
            "-i",
            "sine=frequency=440:duration=130",
            "-c:v",
            "flv",
            "-c:a",
            "aac",
            "-y",
        ])
        .arg(&recording)
        .spawn()
        .expect("writer must start");

    // Let it accumulate enough data for a 60s look-behind plus tail.
    tokio::time::sleep(std::time::Duration::from_secs(72)).await;
    let written = std::fs::metadata(&recording).map(|m| m.len()).unwrap_or(0);
    println!("in-progress recording: {written} bytes");

    let arguments = highlight_clip_arguments(&HighlightClipInput {
        source: &recording,
        output: &clip,
        position_seconds: 60,
        before_seconds: 60,
        after_seconds: 5,
    })
    .unwrap();
    let extraction = LocalFfmpegProcessPort.run(&executable, &arguments).await;
    let _ = writer.kill();
    let _ = writer.wait();
    extraction.expect("clip extraction must succeed on an in-progress file");

    let clip_bytes = std::fs::read(&clip).unwrap_or_default();
    assert!(!clip_bytes.is_empty(), "clip extraction produced no data");
    assert_eq!(&clip_bytes[..3], b"FLV", "clip is not a playable FLV");

    let probe = std::process::Command::new(&ffmpeg)
        .args(["-hide_banner", "-i"])
        .arg(&clip)
        .output()
        .expect("ffmpeg must report clip metadata");
    let stderr = String::from_utf8_lossy(&probe.stderr);
    let duration = stderr
        .split("Duration: ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .expect("clip duration must be reported");
    let parts: Vec<f64> = duration
        .split(':')
        .map(|part| part.trim().parse::<f64>().unwrap_or(0.0))
        .collect();
    let seconds = parts[0] * 3600.0 + parts[1] * 60.0 + parts[2];
    println!("in-progress clip: {} bytes, duration {seconds:.2}s", clip_bytes.len());
    assert!(
        seconds >= 55.0,
        "clip cut from a growing file is too short ({seconds:.2}s); the seek is unreliable"
    );
}