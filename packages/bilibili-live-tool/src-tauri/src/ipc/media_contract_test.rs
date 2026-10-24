use serde_json::json;

use super::dto::{
    CaptainEffectConfigDto, MediaPositionDto, RecordingStateDto, RecordingStatusDto,
    SoundPreviewRequestDto, TtsConfigRequestDto, MEDIA_COMMAND_NAMES, MEDIA_EVENT_NAMES,
};
use super::media::{captain_effect_payload, captain_effect_presets};
use crate::{
    application::media::{default_captain_effect_config, ActiveCaptainEffect, EffectItem},
    domain::live_data::{CaptainAction, CaptainLevel},
};

#[test]
fn exposes_the_approved_media_surface_only() {
    assert_eq!(MEDIA_COMMAND_NAMES.len(), 35);
    assert_eq!(MEDIA_COMMAND_NAMES[0], "captain_effect_get");
    assert_eq!(MEDIA_COMMAND_NAMES[1], "captain_effect_current");
    assert_eq!(MEDIA_COMMAND_NAMES[34], "highlight_delete");
    assert_eq!(MEDIA_EVENT_NAMES.len(), 7);
    assert_eq!(MEDIA_EVENT_NAMES[0], "captain_effect_trigger");
    assert_eq!(MEDIA_EVENT_NAMES[6], "highlight_created");
}

#[test]
fn maps_the_active_preview_to_the_existing_captain_effect_payload_contract() {
    let payload = captain_effect_payload(ActiveCaptainEffect {
        item: EffectItem {
            event_id: "preview-active".into(),
            username: "用户A".into(),
            audience: None,
            action: CaptainAction::Purchase,
            level: CaptainLevel::Captain,
        },
        config: default_captain_effect_config(),
        is_preview: true,
    });

    assert_eq!(payload.event_id, "preview-active");
    assert_eq!(payload.audience.user_uid, "preview-user");
    assert_eq!(payload.audience.username, "用户A");
    assert!(payload.is_preview);
}

#[test]
fn exposes_twelve_built_in_captain_effect_presets_including_golden_premium() {
    let presets = captain_effect_presets();
    let expected = [
        ("captain-default", "经典应援", "#00AEEC", "#FB7299"),
        ("golden-premium", "金色尊享", "#FFD700", "#FFA500"),
        ("starlight-blue", "星海蓝耀", "#38BDF8", "#6366F1"),
        ("cherry-blossom", "樱花绽放", "#FB7185", "#F9A8D4"),
        ("violet-galaxy", "紫曜星河", "#A78BFA", "#7C3AED"),
        ("emerald-stream", "翡翠流光", "#34D399", "#0F766E"),
        ("crimson-celebration", "赤焰庆典", "#F43F5E", "#F97316"),
        ("silver-frost", "冰晶银辉", "#E2E8F0", "#94A3B8"),
        ("neon-party", "霓虹派对", "#22D3EE", "#F472B6"),
        ("aurora-dream", "极光幻彩", "#2DD4BF", "#C084FC"),
        ("sunset-flare", "落日鎏光", "#FB923C", "#EAB308"),
        ("midnight-crown", "午夜王冠", "#818CF8", "#312E81"),
    ];
    let actual = presets
        .iter()
        .map(|preset| {
            (
                preset.preset_id.as_str(),
                preset.display_name.as_str(),
                preset.primary_color.as_str(),
                preset.secondary_color.as_str(),
            )
        })
        .collect::<Vec<_>>();
    let preset_ids = presets
        .iter()
        .map(|preset| preset.preset_id.as_str())
        .collect::<std::collections::HashSet<_>>();

    assert_eq!(actual, expected);
    assert_eq!(preset_ids.len(), presets.len());
    assert!(presets.iter().all(|preset| {
        [&preset.primary_color, &preset.secondary_color]
            .iter()
            .all(|color| {
                color.len() == 7
                    && color.starts_with('#')
                    && color[1..]
                        .chars()
                        .all(|character| character.is_ascii_hexdigit())
            })
    }));
}

#[test]
fn registers_the_same_media_commands_for_both_channels_and_main_only() {
    let library = include_str!("../lib.rs");
    let permission = include_str!("../../permissions/foundation.toml");
    let effect = include_str!("../../capabilities/effect.json");
    for command in MEDIA_COMMAND_NAMES
        .iter()
        .copied()
        .filter(|command| *command != "captain_effect_current")
    {
        assert!(
            library.matches(command).count() >= 3,
            "{command} must be imported and registered for free/member"
        );
        assert!(permission.contains(command), "{command} must be main-only");
    }
    assert!(permission.contains("captain-effect-current"));
    assert!(effect.contains("captain-effect-current"));
    assert!(!effect.contains("foundation"));
}

#[test]
fn targets_effect_delivery_and_recovers_active_state_after_registering_one_listener() {
    let media_ipc = include_str!("media.rs");
    let main = include_str!("../../../src/renderer/main.ts");
    let effect_root =
        include_str!("../../../src/renderer/views/captain-effect/EffectWindowRoot.vue");
    assert!(media_ipc.contains("EventTarget::webview_window(\"effect-captain\")"));
    assert!(media_ipc.contains("EventTarget::webview_window(\"main\")"));
    assert!(main.contains("surface') === 'effect'"));
    assert_eq!(effect_root.matches("listenDesktop(").count(), 1);
    assert_eq!(effect_root.matches("invokeDesktop(").count(), 1);
    assert!(effect_root.contains("captain_effect_current"));
    assert!(effect_root.contains("eventRevision"));
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
