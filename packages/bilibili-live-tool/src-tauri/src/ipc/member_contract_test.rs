use super::member::AI_EVENT_NAMES;
use crate::build_channel::{ai_command_names, member_command_names};

#[cfg(feature = "member")]
use super::member::{
    AuthQrStatusDto, MemberAccountDto, MemberCapabilitiesDto, MemberCapabilityStateDto,
    MemberShellStateDto, MemberStatusDto, MEMBER_EVENT_NAMES,
};

#[cfg(feature = "member")]
#[test]
fn member_build_exposes_the_approved_member_surface() {
    let commands = member_command_names();

    assert_eq!(commands.len(), 19);
    assert_eq!(commands.first(), Some(&"member_status"));
    assert_eq!(commands.last(), Some(&"moderate_mark_false_positive"));
    assert!(!commands.iter().any(|command| command.contains("send")));
    assert_eq!(MEMBER_EVENT_NAMES.len(), 8);
    assert!(!MEMBER_EVENT_NAMES
        .iter()
        .any(|event| event.contains("send")));
}

#[test]
fn both_builds_expose_the_shared_ai_surface() {
    let commands = ai_command_names();

    assert_eq!(commands.len(), 19);
    assert_eq!(commands.first(), Some(&"ai_config_get"));
    assert_eq!(commands.last(), Some(&"ai_log_query"));
    assert!(commands.contains(&"ai_model_profile_save"));
    assert!(commands.contains(&"ai_voice_reply_set_enabled"));
    assert!(commands.contains(&"ai_tts_set_output_device"));
    assert!(!commands.iter().any(|command| command.contains("asr")));
    assert!(!commands.iter().any(|command| command.contains("send")));
    assert_eq!(AI_EVENT_NAMES.len(), 2);
    assert!(AI_EVENT_NAMES.contains(&"ai_generation_finished"));
    assert!(!AI_EVENT_NAMES.iter().any(|event| event.contains("asr")));

    let ipc = include_str!("member.rs");
    let provider = include_str!("../infrastructure/member/ai.rs");
    assert!(ipc.contains("AiReasoningEffortDto"));
    assert!(ipc.contains("pub voice_id: String"));
    assert!(ipc.contains("pub voices: Vec<AiVoiceDto>"));
    assert!(provider.contains("\"reasoning_effort\": profile.reasoning_effort.as_str()"));
    assert!(!provider.contains("\"temperature\": profile.temperature"));
}

#[cfg(feature = "member")]
#[test]
fn serializes_safe_member_status_and_tagged_qr_states() {
    let account = MemberAccountDto {
        user_uid: "123".into(),
        username: "viewer".into(),
        avatar_url: None,
        logged_in_at: 1_700_000_000_000,
    };
    let status = MemberStatusDto {
        shell_state: MemberShellStateDto::LoggedOut,
        account: None,
        entitlement: None,
        capabilities: MemberCapabilitiesDto {
            moderation: MemberCapabilityStateDto::Unavailable,
        },
        service_error_code: Some("NET_SERVICE_UNAVAILABLE".into()),
    };

    let status_json = serde_json::to_value(status).unwrap();
    assert_eq!(status_json["shellState"], "loggedOut");
    assert!(status_json["account"].is_null());
    assert!(status_json.get("token").is_none());
    assert!(status_json.get("cookie").is_none());

    let qr_json = serde_json::to_value(AuthQrStatusDto::Success {
        session_id: "session-1".into(),
        account,
    })
    .unwrap();
    assert_eq!(qr_json["state"], "success");
    assert_eq!(qr_json["sessionId"], "session-1");
}

#[cfg(all(feature = "free", not(feature = "member")))]
#[test]
fn free_build_exposes_no_member_surface() {
    assert!(member_command_names().is_empty());
}

/// PRD 12.2: enabling voice reply carries the data-disclosure acknowledgement;
/// an older caller that omits it deserialises as "not acknowledged".
#[test]
fn ai_voice_enable_request_requires_the_data_disclosure_field() {
    let without: super::member::AiVoiceEnabledRequestDto =
        serde_json::from_str(r#"{"isEnabled":true,"isAudioRouteConfirmed":true}"#).unwrap();
    assert!(!without.is_data_disclosure_acknowledged);

    let with: super::member::AiVoiceEnabledRequestDto = serde_json::from_str(
        r#"{"isEnabled":true,"isAudioRouteConfirmed":true,"isDataDisclosureAcknowledged":true}"#,
    )
    .unwrap();
    assert!(with.is_data_disclosure_acknowledged);
}
