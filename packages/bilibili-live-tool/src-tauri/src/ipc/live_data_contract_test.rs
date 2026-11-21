use serde_json::json;

use super::dto::{
    AudienceIdentityDto, ConnectionStatusDto, DataRangeDto, GiftEventDto, RoomConnectRequestDto,
    RoomSnapshotDto, LIVE_DATA_COMMAND_NAMES, LIVE_DATA_EVENT_NAMES,
};
use crate::build_channel::foundation_command_names;

#[test]
fn exposes_the_approved_live_data_surface_only() {
    assert_eq!(LIVE_DATA_COMMAND_NAMES.len(), 20);
    assert_eq!(LIVE_DATA_EVENT_NAMES.len(), 13);
    assert_eq!(LIVE_DATA_COMMAND_NAMES[0], "room_connect");
    assert_eq!(LIVE_DATA_COMMAND_NAMES[19], "rank_export");
    assert!(LIVE_DATA_EVENT_NAMES.contains(&"room_status_changed"));
    assert!(LIVE_DATA_EVENT_NAMES.contains(&"export_done"));
    // PRD 9.1.1: session boundaries and paid messages have their own events.
    for event in [
        "session_started",
        "session_ended",
        "super_chat_received",
        "super_chat_deleted",
    ] {
        assert!(LIVE_DATA_EVENT_NAMES.contains(&event), "{event}");
    }
    let common_commands = foundation_command_names();
    let permissions = include_str!("../../permissions/foundation.toml");
    for command in LIVE_DATA_COMMAND_NAMES {
        assert!(common_commands.contains(command));
        assert!(permissions.contains(&format!("\"{command}\"")));
    }
}

#[test]
fn deserializes_room_connect_from_an_identity_code() {
    let request: RoomConnectRequestDto = serde_json::from_value(json!({
        "identityCode": "ABC123"
    }))
    .unwrap();
    assert_eq!(request.identity_code, "ABC123");
}

#[test]
fn serializes_room_and_range_contracts_in_camel_case() {
    let room = RoomSnapshotDto {
        status: ConnectionStatusDto::Reconnecting,
        room_id: Some("123".into()),
        room_name: None,
        online_count: Some(42),
        reconnect_attempt: 2,
        last_error_code: Some("NET_TRANSPORT_UNAVAILABLE".into()),
    };

    assert_eq!(
        serde_json::to_value(room).unwrap(),
        json!({
            "status": "reconnecting",
            "roomId": "123",
            "roomName": null,
            "onlineCount": 42,
            "reconnectAttempt": 2,
            "lastErrorCode": "NET_TRANSPORT_UNAVAILABLE"
        })
    );
    assert_eq!(
        serde_json::to_value(DataRangeDto::Custom {
            start_at: 1_700_000_000_000,
            end_at: 1_700_003_600_000,
        })
        .unwrap(),
        json!({
            "kind": "custom",
            "startAt": 1_700_000_000_000_i64,
            "endAt": 1_700_003_600_000_i64
        })
    );
}

#[test]
fn serializes_gift_identity_separately_from_its_display_name() {
    let event = GiftEventDto {
        kind: "gift".into(),
        event_id: "event-1".into(),
        room_id: "100".into(),
        occurred_at: 1_700_000_000_000,
        audience: AudienceIdentityDto {
            user_uid: "200".into(),
            username: "viewer".into(),
            user_level: None,
            medal: None,
            tags: Vec::new(),
        },
        gift_id: "gift-31036".into(),
        gift_name: "辣条".into(),
        gift_icon_url: Some("https://i0.hdslb.com/bfs/live/gift.png".into()),
        quantity: 2,
        value_minor: 200,
    };

    let value = serde_json::to_value(event).unwrap();
    assert_eq!(value["giftId"], "gift-31036");
    assert_eq!(value["giftName"], "辣条");
    assert_eq!(
        value["giftIconUrl"],
        "https://i0.hdslb.com/bfs/live/gift.png"
    );
}
