use crate::domain::validation::{RoomId, UserUid, Username, ValidationError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityTag {
    Admin,
    Governor,
    Admiral,
    Captain,
    Vip,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaptainLevel {
    Governor,
    Admiral,
    Captain,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaptainAction {
    Purchase,
    Renewal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FanMedal {
    pub name: String,
    pub level: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AudienceIdentity {
    pub user_uid: UserUid,
    pub username: String,
    pub user_level: Option<u32>,
    pub medal: Option<FanMedal>,
    pub tags: Vec<IdentityTag>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlacklistEntry {
    pub uid: UserUid,
    pub username: String,
    pub reason: Option<String>,
    pub created_at: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AudienceIdentityInput {
    pub user_uid: String,
    pub username: String,
    pub user_level: Option<i64>,
    pub medal_name: Option<String>,
    pub medal_level: Option<i64>,
    pub is_admin: bool,
    pub captain_level: Option<CaptainLevel>,
    pub is_vip: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum LiveDataDomainError {
    #[error("invalid live data field")]
    InvalidField,
    #[error("money value must be non-negative")]
    InvalidMoney,
    #[error("event timestamp must be positive")]
    InvalidTimestamp,
}

impl From<ValidationError> for LiveDataDomainError {
    fn from(_: ValidationError) -> Self {
        Self::InvalidField
    }
}

pub fn normalize_optional_reason(value: Option<&str>) -> Option<String> {
    value.and_then(|reason| {
        let normalized: String = reason
            .chars()
            .filter(|character| !character.is_control())
            .collect::<String>()
            .trim()
            .to_owned();
        (!normalized.is_empty()).then_some(normalized)
    })
}

pub fn normalize_identity(
    input: AudienceIdentityInput,
) -> Result<AudienceIdentity, LiveDataDomainError> {
    let user_uid = UserUid::parse(&input.user_uid)?;
    let username = Username::parse(&input.username)?.as_str().to_owned();
    let medal = input
        .medal_name
        .as_deref()
        .map(Username::parse)
        .transpose()?
        .map(|name| FanMedal {
            name: name.as_str().to_owned(),
            level: input
                .medal_level
                .and_then(|level| u32::try_from(level).ok()),
        });
    let mut tags = Vec::with_capacity(3);
    if input.is_admin {
        tags.push(IdentityTag::Admin);
    }
    if let Some(level) = input.captain_level {
        tags.push(match level {
            CaptainLevel::Governor => IdentityTag::Governor,
            CaptainLevel::Admiral => IdentityTag::Admiral,
            CaptainLevel::Captain => IdentityTag::Captain,
        });
    }
    if input.is_vip {
        tags.push(IdentityTag::Vip);
    }

    Ok(AudienceIdentity {
        user_uid,
        username,
        user_level: input.user_level.and_then(|level| u32::try_from(level).ok()),
        medal,
        tags,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveEventBase {
    pub event_id: String,
    pub room_id: RoomId,
    pub occurred_at: i64,
    pub audience: AudienceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveEvent {
    Danmu {
        base: LiveEventBase,
        content: String,
    },
    Gift {
        base: LiveEventBase,
        gift_id: String,
        gift_name: String,
        gift_icon_url: Option<String>,
        quantity: u32,
        value_minor: i64,
    },
    Captain {
        base: LiveEventBase,
        action: CaptainAction,
        level: CaptainLevel,
        value_minor: i64,
    },
    Enter {
        base: LiveEventBase,
    },
    /// PRD 9.1.1: the room went live. Resets per-session statistics and is the
    /// signal auto-recording waits for.
    SessionStarted {
        base: LiveEventBase,
    },
    /// PRD 9.1.1: the room ended the session. Seals the statistics for the
    /// session and stops auto-recording.
    SessionEnded {
        base: LiveEventBase,
    },
    /// PRD 9.1.1: a paid message. Counted as revenue and shown highlighted.
    SuperChat {
        base: LiveEventBase,
        message_id: i64,
        content: String,
        value_minor: i64,
        duration_seconds: u64,
    },
    /// PRD 9.1.1: the platform withdrew one or more paid messages. Only the
    /// display is removed; revenue that was already counted stays counted.
    SuperChatDeleted {
        base: LiveEventBase,
        message_ids: Vec<i64>,
    },
}

impl LiveEvent {
    pub const fn base(&self) -> &LiveEventBase {
        match self {
            Self::Danmu { base, .. }
            | Self::Gift { base, .. }
            | Self::Captain { base, .. }
            | Self::Enter { base }
            | Self::SessionStarted { base }
            | Self::SessionEnded { base }
            | Self::SuperChat { base, .. }
            | Self::SuperChatDeleted { base, .. } => base,
        }
    }

    pub fn validate(&self) -> Result<(), LiveDataDomainError> {
        if self.base().event_id.trim().is_empty() {
            return Err(LiveDataDomainError::InvalidField);
        }
        if self.base().occurred_at <= 0 {
            return Err(LiveDataDomainError::InvalidTimestamp);
        }
        match self {
            Self::Danmu { content, .. } if content.trim().is_empty() => {
                Err(LiveDataDomainError::InvalidField)
            }
            Self::Gift {
                gift_id,
                gift_name,
                quantity,
                value_minor,
                ..
            } if gift_id.trim().is_empty()
                || gift_name.trim().is_empty()
                || *quantity == 0
                || *value_minor < 0 =>
            {
                Err(LiveDataDomainError::InvalidField)
            }
            Self::Captain { value_minor, .. } if *value_minor < 0 => {
                Err(LiveDataDomainError::InvalidMoney)
            }
            _ => Ok(()),
        }
    }
}
