use crate::application::ports::StoragePortError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigScope {
    Global,
    Room,
}

impl ConfigScope {
    pub fn parse(value: &str) -> Result<Self, ConfigError> {
        match value {
            "global" => Ok(Self::Global),
            "room" => Ok(Self::Room),
            _ => Err(ConfigError::InvalidScope),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::Room => "room",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigKey(String);

impl ConfigKey {
    pub fn parse(value: &str) -> Result<Self, ConfigError> {
        let normalized = value.trim();
        let normalized_lower = normalized.to_ascii_lowercase();
        let is_sensitive = ["token", "secret", "password", "cookie", "credential"]
            .iter()
            .any(|part| normalized_lower.contains(part));
        let is_valid = !normalized.is_empty()
            && normalized.len() <= 100
            && normalized
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'));

        if !is_valid || is_sensitive {
            return Err(ConfigError::InvalidKey);
        }

        Ok(Self(normalized.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigValue {
    pub value: String,
    pub updated_at: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid configuration key")]
    InvalidKey,
    #[error("invalid configuration scope")]
    InvalidScope,
    #[error(transparent)]
    Storage(#[from] StoragePortError),
}

#[cfg(test)]
mod tests {
    use super::{ConfigKey, ConfigScope};

    #[test]
    fn accepts_the_remembered_room_session_keys() {
        // The shell persists the operator's identity code and auto-connect flag
        // under these keys; a rejected key would silently disable the feature.
        assert!(ConfigKey::parse("live.identity").is_ok());
        assert!(ConfigKey::parse("live.auto_connect").is_ok());
        assert!(ConfigScope::parse("global").is_ok());
    }

    #[test]
    fn still_rejects_sensitive_or_malformed_keys() {
        assert!(ConfigKey::parse("live.credential").is_err());
        assert!(ConfigKey::parse("live session").is_err());
        assert!(ConfigScope::parse("everything").is_err());
    }
}
