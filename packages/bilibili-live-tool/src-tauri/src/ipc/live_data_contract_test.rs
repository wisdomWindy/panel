use serde_json::json;

use super::dto::{
    AudienceIdentityDto, ConnectionStatusDto, DataRangeDto, GiftEventDto, RoomSnapshotDto,
    LIVE_DATA_COMMAND_NAMES, LIVE_DATA_EVENT_NAMES,
};
use crate::build_channel::foundation_command_names;

#[test]
fn exposes_the_approved_live_data_surface_only() {
    assert_eq!(LIVE_DATA_COMMAND_NAMES.len(), 20);
    assert_eq!(LIVE_DATA_EVENT_NAMES.len(), 9);
    assert_eq!(LIVE_DATA_COMMAND_NAMES[0], "room_connect");
    assert_eq!(LIVE_DATA_COMMAND_NAMES[19], "rank_export");
    assert!(LIVE_DATA_EVENT_NAMES.contains(&"room_status_changed"));
    assert!(LIVE_DATA_EVENT_NAMES.contains(&"export_done"));
    let common_commands = foundation_command_names();
    let permissions = include_str!("../../permissions/foundation.toml");
    for command in LIVE_DATA_COMMAND_NAMES {
        assert!(common_commands.contains(command));
        assert!(permissions.contains(&format!("\"{command}\"")));
    }
}

#[test]
fn serializes_room_and_range_contracts_in_camel_case() {
    let room = RoomSnapshotDto {
        status: ConnectionStatusDto::Reconnecting,
        room_id: Some("123".into()),
        room_name: None,
        online_count: Some(42),
        reconnect_attempt: 2,
        last_error_code: Some("transport_unavailable".into()),
    };

    assert_eq!(
        serde_json::to_value(room).unwrap(),
        json!({
            "status": "reconnecting",
            "roomId": "123",
            "roomName": null,
            "onlineCount": 42,
            "reconnectAttempt": 2,
            "lastErrorCode": "transport_unavailable"
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
        quantity: 2,
        value_minor: 200,
    };

    let value = serde_json::to_value(event).unwrap();
    assert_eq!(value["giftId"], "gift-31036");
    assert_eq!(value["giftName"], "辣条");
}
