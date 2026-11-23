use crate::application::live_data::LiveRuntimeError;

/// Build-time defaults, injected by the release workflow from repository secrets.
///
/// The literal values intentionally do not live in the repository: they are
/// supplied through `BILI_OPEN_*` environment variables when the binary is
/// built, and can be overridden at run time with the same variables (handy for
/// local development or a rotated key without rebuilding).
const BUILD_APP_ID: Option<&str> = option_env!("BILI_OPEN_APP_ID");
const BUILD_ACCESS_KEY_ID: Option<&str> = option_env!("BILI_OPEN_ACCESS_KEY_ID");
const BUILD_ACCESS_KEY_SECRET: Option<&str> = option_env!("BILI_OPEN_ACCESS_KEY_SECRET");

#[derive(Clone)]
pub struct OpenLiveConfig {
    pub app_id: u64,
    pub access_key_id: String,
    access_key_secret: String,
}

fn resolve(name: &str, build_time: Option<&str>) -> String {
    match std::env::var(name) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => build_time.unwrap_or_default().to_owned(),
    }
}

impl OpenLiveConfig {
    /// Credentials for the 哔哩哔哩开放平台: run-time environment wins over the
    /// values compiled in at build time.
    pub fn embedded() -> Result<Self, LiveRuntimeError> {
        let app_id = resolve("BILI_OPEN_APP_ID", BUILD_APP_ID);
        let access_key_id = resolve("BILI_OPEN_ACCESS_KEY_ID", BUILD_ACCESS_KEY_ID);
        let access_key_secret = resolve("BILI_OPEN_ACCESS_KEY_SECRET", BUILD_ACCESS_KEY_SECRET);
        Self::from_values(&app_id, &access_key_id, &access_key_secret)
    }

    pub fn from_values(
        app_id: &str,
        access_key_id: &str,
        access_key_secret: &str,
    ) -> Result<Self, LiveRuntimeError> {
        let app_id = app_id.trim();
        let access_key_id = access_key_id.trim();
        let access_key_secret = access_key_secret.trim();
        if app_id.is_empty() || access_key_id.is_empty() || access_key_secret.is_empty() {
            return Err(LiveRuntimeError::new(
                "SYS_OPEN_LIVE_CREDENTIALS_MISSING",
                "缺少哔哩哔哩开放平台运行时配置，请设置 BILI_OPEN_APP_ID / BILI_OPEN_ACCESS_KEY_ID / BILI_OPEN_ACCESS_KEY_SECRET",
                false,
            ));
        }
        let app_id = app_id.parse::<u64>().map_err(|_| {
            LiveRuntimeError::new(
                "invalid_open_live_app_id",
                "哔哩哔哩开放平台 appid 无效",
                false,
            )
        })?;
        if app_id == 0 {
            return Err(LiveRuntimeError::new(
                "invalid_open_live_app_id",
                "哔哩哔哩开放平台 appid 无效",
                false,
            ));
        }
        Ok(Self {
            app_id,
            access_key_id: access_key_id.to_owned(),
            access_key_secret: access_key_secret.to_owned(),
        })
    }

    pub fn access_key_secret(&self) -> &str {
        &self.access_key_secret
    }
}

impl std::fmt::Debug for OpenLiveConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenLiveConfig")
            .field("app_id", &self.app_id)
            .field("access_key_id", &"[REDACTED]")
            .field("access_key_secret", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::OpenLiveConfig;


    #[test]
    fn explicit_values_are_trimmed_and_redacted_in_debug() {
        let config = OpenLiveConfig::from_values("1716751199246", " key ", " s3cr3t-value ").expect("valid");
        assert_eq!(config.app_id, 1716751199246);
        assert_eq!(config.access_key_id, "key");
        let rendered = format!("{config:?}");
        assert!(!rendered.contains("s3cr3t-value"));
        assert!(rendered.contains("REDACTED"));
    }

    #[test]
    fn an_empty_app_id_is_rejected() {
        let error = OpenLiveConfig::from_values("", "key", "secret").expect_err("empty app id");
        assert_eq!(error.code, "SYS_OPEN_LIVE_CREDENTIALS_MISSING");
    }
}
