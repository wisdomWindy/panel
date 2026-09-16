use crate::build_channel::member_command_names;

#[cfg(feature = "member")]
use super::member::{
    AuthQrStatusDto, MemberAccountDto, MemberCapabilitiesDto, MemberCapabilityStateDto,
    MemberShellStateDto, MemberStatusDto, MEMBER_EVENT_NAMES,
};

#[cfg(feature = "member")]
#[test]
fn member_build_exposes_the_approved_member_surface() {
    let commands = member_command_names();

    assert_eq!(commands.len(), 28);
    assert_eq!(commands.first(), Some(&"member_status"));
    assert_eq!(commands.last(), Some(&"moderate_mark_false_positive"));
    assert!(!commands.iter().any(|command| command.contains("send")));
    assert_eq!(MEMBER_EVENT_NAMES.len(), 11);
    assert!(!MEMBER_EVENT_NAMES
        .iter()
        .any(|event| event.contains("send")));
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
            ai: MemberCapabilityStateDto::Unavailable,
            moderation: MemberCapabilityStateDto::Unavailable,
        },
        service_error_code: Some("external_service_unavailable".into()),
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
