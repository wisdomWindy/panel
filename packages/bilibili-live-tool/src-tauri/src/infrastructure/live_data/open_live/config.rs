use crate::application::live_data::LiveRuntimeError;

/// Embedded 哔哩哔哩开放平台 credentials so a plain build works out of the box.
///
/// A build-time `BILI_OPEN_*` value (CI secret) wins over these, and a run-time
/// environment variable wins over both, so a rotated key can be used without
/// editing this file.
const EMBEDDED_APP_ID: &str = "1716751199246";
const EMBEDDED_ACCESS_KEY_ID: &str = "92yCCLAhXtGfRzWKMgSXruqN";
const EMBEDDED_ACCESS_KEY_SECRET: &str = "9lIaEErbeC62RJ1ApCNrx47ao3v0wd";

const BUILD_APP_ID: Option<&str> = option_env!("BILI_OPEN_APP_ID");
const BUILD_ACCESS_KEY_ID: Option<&str> = option_env!("BILI_OPEN_ACCESS_KEY_ID");
const BUILD_ACCESS_KEY_SECRET: Option<&str> = option_env!("BILI_OPEN_ACCESS_KEY_SECRET");

#[derive(Clone)]
pub struct OpenLiveConfig {
    pub app_id: u64,
    pub access_key_id: String,
    access_key_secret: String,
}

/// Run-time value, then build-time value, then the embedded constant.
///
/// A *blank* value counts as absent at both levels: CI injects
/// `BILI_OPEN_*: ${{ secrets.* }}`, which is an empty string when the secret is
/// not configured, and an empty string must not shadow the embedded fallback or
/// the packaged app can never connect.
fn resolve(name: &str, build_time: Option<&str>, embedded: &str) -> String {
    if let Ok(value) = std::env::var(name) {
        if !value.trim().is_empty() {
            return value;
        }
    }
    build_time
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(embedded)
        .to_owned()
}

impl OpenLiveConfig {
    /// Credentials for the 哔哩哔哩开放平台: run-time environment wins over the
    /// values compiled in at build time.
    pub fn embedded() -> Result<Self, LiveRuntimeError> {
        let app_id = resolve("BILI_OPEN_APP_ID", BUILD_APP_ID, EMBEDDED_APP_ID);
        let access_key_id = resolve("BILI_OPEN_ACCESS_KEY_ID", BUILD_ACCESS_KEY_ID, EMBEDDED_ACCESS_KEY_ID);
        let access_key_secret = resolve("BILI_OPEN_ACCESS_KEY_SECRET", BUILD_ACCESS_KEY_SECRET, EMBEDDED_ACCESS_KEY_SECRET);
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
    fn a_blank_build_time_value_falls_back_to_the_embedded_constant() {
        // CI injects `BILI_OPEN_*: ${{ secrets.* }}`, which is an empty string
        // when the secret is missing; that must not shadow the compiled values.
        assert_eq!(
            super::resolve("BILI_OPEN_TEST_BLANK", Some(""), "embedded"),
            "embedded"
        );
        assert_eq!(
            super::resolve("BILI_OPEN_TEST_BLANK", Some("   "), "embedded"),
            "embedded"
        );
        assert_eq!(
            super::resolve("BILI_OPEN_TEST_BLANK", None, "embedded"),
            "embedded"
        );
        assert_eq!(
            super::resolve("BILI_OPEN_TEST_BLANK", Some("injected"), "embedded"),
            "injected"
        );
    }

    #[test]
    fn an_empty_app_id_is_rejected() {
        let error = OpenLiveConfig::from_values("", "key", "secret").expect_err("empty app id");
        assert_eq!(error.code, "SYS_OPEN_LIVE_CREDENTIALS_MISSING");
    }

    #[test]
    fn the_embedded_credentials_are_usable() {
        let config = OpenLiveConfig::embedded().expect("embedded credentials");
        assert_eq!(config.app_id, 1_716_751_199_246);
        assert!(!config.access_key_id.is_empty());
        assert!(!config.access_key_secret().is_empty());
    }
}
