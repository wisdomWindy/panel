use crate::application::live_data::LiveRuntimeError;

/// Open-platform credentials are injected at build time and never committed.
///
/// A committed secret is a public secret: this repository ships releases, so any
/// value in source is readable by everyone and extractable from the binary.
/// Supply them from the environment when building (`OPEN_LIVE_*`), or fail with
/// `missing_open_live_credentials` so the UI can explain what is missing.
const APP_ID: &str = match option_env!("OPEN_LIVE_APP_ID") {
    Some(value) => value,
    None => "",
};
const ACCESS_KEY_ID: &str = match option_env!("OPEN_LIVE_ACCESS_KEY_ID") {
    Some(value) => value,
    None => "",
};
const ACCESS_KEY_SECRET: &str = match option_env!("OPEN_LIVE_ACCESS_KEY_SECRET") {
    Some(value) => value,
    None => "",
};

#[derive(Clone)]
pub struct OpenLiveConfig {
    pub app_id: u64,
    pub access_key_id: String,
    access_key_secret: String,
}

impl OpenLiveConfig {
    pub fn embedded() -> Result<Self, LiveRuntimeError> {
        Self::from_values(APP_ID, ACCESS_KEY_ID, ACCESS_KEY_SECRET)
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
                "missing_open_live_credentials",
                "缺少哔哩哔哩开放平台运行时配置",
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
