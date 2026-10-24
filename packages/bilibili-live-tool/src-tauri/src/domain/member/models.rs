use crate::domain::live_data::UserUid;

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: &str) -> Result<Self, &'static str> {
                let value = value.trim();
                if value.is_empty() {
                    return Err("identifier cannot be empty");
                }
                Ok(Self(value.to_owned()))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

string_id!(AuthSessionId);
string_id!(OrderId);
string_id!(ModuleId);
string_id!(MemberOperationId);
string_id!(GenerationId);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberAccount {
    pub user_uid: UserUid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub logged_in_at: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntitlementStatus {
    Active,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entitlement {
    pub status: EntitlementStatus,
    pub plan_id: String,
    pub expires_at: i64,
    pub issued_at: i64,
    pub last_verified_at: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemberModuleStatus {
    Missing,
    Downloading,
    Verifying,
    Installed,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberModule {
    pub module_id: ModuleId,
    pub name: String,
    pub version: String,
    pub status: MemberModuleStatus,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub error_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiCapabilityState {
    Unavailable,
    MissingModule,
    Ready,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberCapabilities {
    pub moderation: AiCapabilityState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntitlementValidity {
    ActiveOnline,
    ActiveOffline,
    Expired,
    VerificationRequired,
    InvalidClock,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModerationDefaults {
    pub auto_mute_enabled: bool,
    pub frequency_window_seconds: u16,
    pub frequency_limit: u16,
    pub duplicate_window_seconds: u16,
    pub duplicate_limit: u16,
    pub similarity_window_seconds: u16,
    pub similarity_limit: u16,
    pub similarity_threshold_percent: u8,
    pub mute_duration_seconds: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemberShellState {
    Unavailable,
    LoggedOut,
    Unpaid,
    Active,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberSnapshot {
    pub shell_state: MemberShellState,
    pub account: Option<MemberAccount>,
    pub entitlement: Option<Entitlement>,
    pub modules: Vec<MemberModule>,
    pub capabilities: MemberCapabilities,
    pub service_error_code: Option<String>,
}
