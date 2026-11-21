use crate::{
    application::live_data::LiveRuntimeError,
    domain::live_data::{
        normalize_identity, AudienceIdentity, AudienceIdentityInput, CaptainAction, CaptainLevel,
        LiveEvent, LiveEventBase, RoomId, UserUid,
    },
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct CommandEnvelope {
    cmd: String,
    data: Value,
}

#[derive(Deserialize)]
struct CommonData {
    room_id: u64,
    #[serde(default)]
    uid: u64,
    #[serde(default)]
    open_id: String,
    uname: String,
    timestamp: i64,
    #[serde(default)]
    msg_id: String,
    #[serde(default)]
    fans_medal_level: i64,
    #[serde(default)]
    fans_medal_name: String,
    #[serde(default)]
    fans_medal_wearing_status: bool,
    #[serde(default)]
    guard_level: i64,
    #[serde(default)]
    is_admin: i64,
}

#[derive(Deserialize)]
struct DanmuData {
    #[serde(flatten)]
    common: CommonData,
    msg: String,
}

#[derive(Deserialize)]
struct GiftData {
    #[serde(flatten)]
    common: CommonData,
    gift_id: Value,
    gift_name: String,
    #[serde(default)]
    gift_icon: String,
    gift_num: u32,
    price: i64,
}

#[derive(Deserialize)]
struct GuardUser {
    #[serde(default)]
    uid: u64,
    #[serde(default)]
    open_id: String,
    uname: String,
}

#[derive(Deserialize)]
struct GuardData {
    room_id: u64,
    user_info: GuardUser,
    guard_level: i64,
    price: i64,
    timestamp: i64,
    msg_id: String,
    #[serde(default)]
    fans_medal_level: i64,
    #[serde(default)]
    fans_medal_name: String,
    #[serde(default)]
    fans_medal_wearing_status: bool,
}

pub fn map_command(body: &[u8]) -> Result<Option<LiveEvent>, LiveRuntimeError> {
    let envelope: CommandEnvelope = serde_json::from_slice(body).map_err(invalid_event)?;
    let event = match envelope.cmd.as_str() {
        "LIVE_OPEN_PLATFORM_DM" => map_danmu(envelope.data)?,
        "LIVE_OPEN_PLATFORM_SEND_GIFT" => map_gift(envelope.data)?,
        "LIVE_OPEN_PLATFORM_GUARD" => map_guard(envelope.data)?,
        "LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER" => map_enter(envelope.data)?,
        "LIVE_OPEN_PLATFORM_LIVE_START" => map_session(envelope.data, true)?,
        "LIVE_OPEN_PLATFORM_LIVE_END" => map_session(envelope.data, false)?,
        // PRD 9.1.1: DM_MIRROR repeats DM without sender fields; collecting it
        // would double-count danmaku and lose the audience identity.
        "LIVE_OPEN_PLATFORM_DM_MIRROR" => return Ok(None),
        _ => return Ok(None),
    };
    event.validate().map_err(invalid_event)?;
    Ok(Some(event))
}

fn map_danmu(value: Value) -> Result<LiveEvent, LiveRuntimeError> {
    let data: DanmuData = serde_json::from_value(value).map_err(invalid_event)?;
    Ok(LiveEvent::Danmu {
        base: common_base(&data.common, &data.common.msg_id)?,
        content: data.msg,
    })
}

fn map_gift(value: Value) -> Result<LiveEvent, LiveRuntimeError> {
    let data: GiftData = serde_json::from_value(value).map_err(invalid_event)?;
    let gift_id = match data.gift_id {
        Value::String(value) => value,
        Value::Number(value) => value.to_string(),
        _ => return Err(invalid_event("gift id")),
    };
    let value_minor = data
        .price
        .checked_mul(i64::from(data.gift_num))
        .and_then(|value| value.checked_div(10))
        .ok_or_else(|| invalid_event("gift value"))?;
    Ok(LiveEvent::Gift {
        base: common_base(&data.common, &data.common.msg_id)?,
        gift_id,
        gift_name: data.gift_name,
        gift_icon_url: (!data.gift_icon.trim().is_empty())
            .then(|| data.gift_icon.trim().to_owned()),
        quantity: data.gift_num,
        value_minor,
    })
}

fn map_guard(value: Value) -> Result<LiveEvent, LiveRuntimeError> {
    let data: GuardData = serde_json::from_value(value).map_err(invalid_event)?;
    let level = captain_level(data.guard_level).ok_or_else(|| invalid_event("guard level"))?;
    let common = CommonData {
        room_id: data.room_id,
        uid: data.user_info.uid,
        open_id: data.user_info.open_id,
        uname: data.user_info.uname,
        timestamp: data.timestamp,
        msg_id: data.msg_id.clone(),
        fans_medal_level: data.fans_medal_level,
        fans_medal_name: data.fans_medal_name,
        fans_medal_wearing_status: data.fans_medal_wearing_status,
        guard_level: data.guard_level,
        is_admin: 0,
    };
    Ok(LiveEvent::Captain {
        base: common_base(&common, &data.msg_id)?,
        action: CaptainAction::Purchase,
        level,
        value_minor: data
            .price
            .checked_div(10)
            .ok_or_else(|| invalid_event("guard value"))?,
    })
}

fn map_enter(value: Value) -> Result<LiveEvent, LiveRuntimeError> {
    let data: CommonData = serde_json::from_value(value).map_err(invalid_event)?;
    let uid = user_uid(&data)?;
    let event_id = format!("enter:{}:{uid}:{}", data.room_id, data.timestamp);
    Ok(LiveEvent::Enter {
        base: common_base(&data, &event_id)?,
    })
}

#[derive(Deserialize)]
struct SessionData {
    room_id: u64,
    timestamp: i64,
    #[serde(default)]
    uname: String,
    #[serde(default)]
    uid: u64,
    #[serde(default)]
    open_id: String,
}

fn map_session(value: Value, started: bool) -> Result<LiveEvent, LiveRuntimeError> {
    let data: SessionData = serde_json::from_value(value).map_err(invalid_event)?;
    let marker = if started { "session-start" } else { "session-end" };
    let event_id = format!("{marker}:{}:{}", data.room_id, data.timestamp);
    // A session boundary has no viewer identity: the "audience" is the room
    // itself, so the shared audience normaliser must not be used here.
    let audience = AudienceIdentity {
        user_uid: UserUid::parse(&format!("room-{}", data.room_id)).map_err(invalid_event)?,
        username: if data.uname.trim().is_empty() {
            format!("#{}", data.room_id)
        } else {
            data.uname.trim().to_owned()
        },
        user_level: None,
        medal: None,
        tags: Vec::new(),
    };
    let base = LiveEventBase {
        event_id,
        room_id: RoomId::parse(&data.room_id.to_string()).map_err(invalid_event)?,
        occurred_at: data
            .timestamp
            .checked_mul(1_000)
            .filter(|value| *value > 0)
            .ok_or_else(|| invalid_event("timestamp"))?,
        audience,
    };
    Ok(if started {
        LiveEvent::SessionStarted { base }
    } else {
        LiveEvent::SessionEnded { base }
    })
}

fn common_base(data: &CommonData, event_id: &str) -> Result<LiveEventBase, LiveRuntimeError> {
    let room_id = RoomId::parse(&data.room_id.to_string()).map_err(invalid_event)?;
    let occurred_at = data
        .timestamp
        .checked_mul(1_000)
        .filter(|value| *value > 0)
        .ok_or_else(|| invalid_event("timestamp"))?;
    Ok(LiveEventBase {
        event_id: event_id.to_owned(),
        room_id,
        occurred_at,
        audience: audience(data)?,
    })
}

fn audience(data: &CommonData) -> Result<AudienceIdentity, LiveRuntimeError> {
    normalize_identity(AudienceIdentityInput {
        user_uid: user_uid(data)?,
        username: data.uname.clone(),
        user_level: None,
        medal_name: (data.fans_medal_wearing_status && !data.fans_medal_name.trim().is_empty())
            .then(|| data.fans_medal_name.clone()),
        medal_level: data
            .fans_medal_wearing_status
            .then_some(data.fans_medal_level),
        is_admin: data.is_admin == 1,
        captain_level: captain_level(data.guard_level),
        is_vip: false,
    })
    .map_err(invalid_event)
}

fn user_uid(data: &CommonData) -> Result<String, LiveRuntimeError> {
    if !data.open_id.trim().is_empty() {
        Ok(data.open_id.trim().to_owned())
    } else if data.uid > 0 {
        Ok(data.uid.to_string())
    } else {
        Err(invalid_event("user id"))
    }
}

fn captain_level(value: i64) -> Option<CaptainLevel> {
    match value {
        1 => Some(CaptainLevel::Governor),
        2 => Some(CaptainLevel::Admiral),
        3 => Some(CaptainLevel::Captain),
        _ => None,
    }
}

fn invalid_event(_: impl std::fmt::Display) -> LiveRuntimeError {
    LiveRuntimeError::new(
        "invalid_open_live_event",
        "哔哩哔哩开放平台消息格式无效",
        false,
    )
}
