use serde_json::json;

use super::dto::{
    CaptainEffectConfigDto, MediaPositionDto, RecordingStateDto, RecordingStatusDto,
    SoundPreviewRequestDto, TtsConfigRequestDto, MEDIA_COMMAND_NAMES, MEDIA_EVENT_NAMES,
};

#[test]
fn exposes_the_approved_media_surface_only() {
    assert_eq!(MEDIA_COMMAND_NAMES.len(), 34);
    assert_eq!(MEDIA_COMMAND_NAMES[0], "captain_effect_get");
    assert_eq!(MEDIA_COMMAND_NAMES[33], "highlight_delete");
    assert_eq!(MEDIA_EVENT_NAMES.len(), 7);
    assert_eq!(MEDIA_EVENT_NAMES[0], "captain_effect_trigger");
    assert_eq!(MEDIA_EVENT_NAMES[6], "highlight_created");
}

#[test]
fn registers_the_same_media_commands_for_both_channels_and_main_only() {
    let library = include_str!("../lib.rs");
    let permission = include_str!("../../permissions/foundation.toml");
    let effect = include_str!("../../capabilities/effect.json");
    for command in MEDIA_COMMAND_NAMES {
        assert!(
            library.matches(command).count() >= 3,
            "{command} must be imported and registered for free/member"
        );
        assert!(permission.contains(command), "{command} must be main-only");
    }
    assert!(effect.contains("\"permissions\": []"));
}

#[test]
fn targets_effect_delivery_and_bootstraps_one_render_only_listener() {
    let media_ipc = include_str!("media.rs");
    let main = include_str!("../../../src/renderer/main.ts");
    let effect_root =
        include_str!("../../../src/renderer/views/captain-effect/EffectWindowRoot.vue");
    assert!(media_ipc.contains("EventTarget::webview_window(\"effect-captain\")"));
    assert!(media_ipc.contains("EventTarget::webview_window(\"main\")"));
    assert!(main.contains("surface') === 'effect'"));
    assert_eq!(effect_root.matches("listenDesktop(").count(), 1);
    assert!(!effect_root.contains("invokeDesktop("));
}

#[test]
fn malformed_media_payloads_fail_closed_during_deserialization() {
    assert!(serde_json::from_value::<TtsConfigRequestDto>(json!({
        "config": {
            "isEnabled": true,
            "selectedEngine": "unknown",
            "selectedVoiceId": null,
            "rate": 1,
            "volume": 0.8,
            "eventKinds": ["danmu"],
            "identityKinds": ["ordinary"],
            "templates": { "danmu": "{用户}说：{内容}", "gift": "{用户}", "captain": "{用户}" }
        }
    }))
    .is_err());
    assert!(serde_json::from_value::<SoundPreviewRequestDto>(json!({
        "target": { "kind": "grant", "grantId": null },
        "volume": 0.8
    }))
    .is_err());
}

#[test]
fn serializes_explicit_media_units_and_null_absence() {
    let config = CaptainEffectConfigDto {
        preset_id: "gold".into(),
        duration_seconds: 5,
        position: MediaPositionDto::TopRight,
        primary_color: "#FFD700".into(),
        secondary_color: "#FFA500".into(),
        first_sound: None,
        renewal_sound: None,
        combo_sound: None,
        volume: 0.7,
        combo_enabled: true,
        combo_window_seconds: 10,
    };
    let status = RecordingStatusDto {
        state: RecordingStateDto::Idle,
        session_id: None,
        room_id: None,
        elapsed_seconds: 0,
        file_size_bytes: 0,
        disk_remaining_bytes: 5 * 1024 * 1024 * 1024,
        highlight_count: 0,
        last_error_code: None,
    };

    assert_eq!(serde_json::to_value(config).unwrap()["durationSeconds"], 5);
    assert_eq!(
        serde_json::to_value(status).unwrap(),
        json!({
            "state": "idle",
            "sessionId": null,
            "roomId": null,
            "elapsedSeconds": 0,
            "fileSizeBytes": 0,
            "diskRemainingBytes": 5_u64 * 1024 * 1024 * 1024,
            "highlightCount": 0,
            "lastErrorCode": null
        })
    );
}
