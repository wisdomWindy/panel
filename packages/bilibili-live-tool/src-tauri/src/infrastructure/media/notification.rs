use crate::application::media::{
    MediaIoError, NotificationPermission, NotificationPort, NotificationRequest,
};
#[cfg(target_os = "windows")]
use futures_util::StreamExt;
use reqwest::Url;
#[cfg(target_os = "windows")]
use reqwest::{header::CONTENT_TYPE, redirect::Policy};
use sha2::{Digest, Sha256};
#[cfg(target_os = "windows")]
use std::{path::PathBuf, time::Duration};
#[cfg(target_os = "windows")]
use tauri::Manager;
use tauri::Runtime;
use tauri_plugin_notification::NotificationExt;

pub struct TauriNotificationPort<R: Runtime> {
    app: tauri::AppHandle<R>,
    #[cfg(target_os = "windows")]
    client: Option<reqwest::Client>,
}

impl<R: Runtime> TauriNotificationPort<R> {
    pub fn new(app: tauri::AppHandle<R>) -> Self {
        #[cfg(target_os = "windows")]
        let client = reqwest::Client::builder()
            .redirect(Policy::none())
            .timeout(Duration::from_secs(3))
            .build()
            .ok();
        Self {
            app,
            #[cfg(target_os = "windows")]
            client,
        }
    }

    #[cfg(target_os = "windows")]
    async fn cached_notification_image(&self, raw_url: &str) -> Result<PathBuf, MediaIoError> {
        const MAX_IMAGE_BYTES: usize = 1024 * 1024;
        const SUPPORTED_EXTENSIONS: [&str; 3] = ["png", "jpg", "gif"];

        let url = validated_gift_image_url(raw_url)
            .ok_or_else(|| MediaIoError::invalid("gift image URL is invalid"))?;
        let cache_dir = self
            .app
            .path()
            .app_cache_dir()
            .map_err(|_| MediaIoError::unavailable("gift image cache is unavailable"))?
            .join("gift-notification-images");
        tokio::fs::create_dir_all(&cache_dir)
            .await
            .map_err(|_| MediaIoError::operation("gift image cache could not be created", true))?;

        for extension in SUPPORTED_EXTENSIONS {
            let path = cache_dir.join(notification_image_cache_name(raw_url, extension));
            if let Ok(metadata) = tokio::fs::metadata(&path).await {
                if metadata.len() > 0 && metadata.len() <= MAX_IMAGE_BYTES as u64 {
                    return Ok(path);
                }
            }
        }

        let client = self
            .client
            .as_ref()
            .ok_or_else(|| MediaIoError::unavailable("gift image download is unavailable"))?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|_| MediaIoError::operation("gift image download failed", true))?;
        if !response.status().is_success() {
            return Err(MediaIoError::operation(
                "gift image download returned an invalid status",
                true,
            ));
        }
        if response
            .content_length()
            .is_some_and(|length| length > MAX_IMAGE_BYTES as u64)
        {
            return Err(MediaIoError::invalid("gift image is too large"));
        }
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(notification_image_extension)
            .ok_or_else(|| MediaIoError::invalid("gift image format is unsupported"))?;
        let mut bytes = Vec::new();
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk =
                chunk.map_err(|_| MediaIoError::operation("gift image download failed", true))?;
            if bytes.len().saturating_add(chunk.len()) > MAX_IMAGE_BYTES {
                return Err(MediaIoError::invalid("gift image is too large"));
            }
            bytes.extend_from_slice(&chunk);
        }
        if bytes.is_empty() {
            return Err(MediaIoError::invalid("gift image is empty"));
        }

        let path = cache_dir.join(notification_image_cache_name(raw_url, content_type));
        let temporary_path = path.with_extension(format!("{content_type}.tmp"));
        tokio::fs::write(&temporary_path, bytes)
            .await
            .map_err(|_| MediaIoError::operation("gift image cache write failed", true))?;
        if let Err(error) = tokio::fs::rename(&temporary_path, &path).await {
            let _ = tokio::fs::remove_file(&temporary_path).await;
            if !path.exists() {
                return Err(MediaIoError::operation(
                    format!("gift image cache commit failed: {error}"),
                    true,
                ));
            }
        }
        Ok(path)
    }
}

#[async_trait::async_trait]
impl<R: Runtime> NotificationPort for TauriNotificationPort<R> {
    async fn permission(&self) -> Result<NotificationPermission, MediaIoError> {
        self.app
            .notification()
            .permission_state()
            .map(map_permission)
            .map_err(|_| MediaIoError::unavailable("notification permission is unavailable"))
    }

    async fn request_permission(&self) -> Result<NotificationPermission, MediaIoError> {
        self.app
            .notification()
            .request_permission()
            .map(map_permission)
            .map_err(|_| MediaIoError::unavailable("notification permission request failed"))
    }

    async fn show(&self, request: NotificationRequest) -> Result<(), MediaIoError> {
        if request.title.trim().is_empty()
            || request.body.trim().is_empty()
            || request.title.chars().any(char::is_control)
            || request.body.chars().any(char::is_control)
        {
            return Err(MediaIoError::invalid("notification content is invalid"));
        }
        if self.permission().await? != NotificationPermission::Granted {
            return Err(MediaIoError::permission(
                "notification permission has not been granted",
            ));
        }
        #[cfg(target_os = "windows")]
        {
            let image_path = match request.image_url.as_deref() {
                Some(url) => match self.cached_notification_image(url).await {
                    Ok(path) => Some(path),
                    Err(error) => {
                        log::warn!("gift notification image is degraded: {}", error.code);
                        None
                    }
                },
                None => None,
            };
            return show_windows_notification(
                &self.app,
                &request.title,
                &request.body,
                image_path.as_deref(),
            );
        }
        #[cfg(not(target_os = "windows"))]
        self.app
            .notification()
            .builder()
            .title(request.title)
            .body(request.body)
            .show()
            .map_err(|_| MediaIoError::operation("notification delivery failed", true))
    }
}

#[cfg_attr(not(windows), allow(dead_code))]
fn validated_gift_image_url(value: &str) -> Option<Url> {
    let url = Url::parse(value.trim()).ok()?;
    let host = url.host_str()?;
    let allowed_host = host == "bilibili.com"
        || host.ends_with(".bilibili.com")
        || host == "hdslb.com"
        || host.ends_with(".hdslb.com");
    (url.scheme() == "https"
        && allowed_host
        && url.username().is_empty()
        && url.password().is_none()
        && url.port().is_none())
    .then_some(url)
}

#[cfg_attr(not(windows), allow(dead_code))]
fn notification_image_cache_name(value: &str, extension: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    format!("{digest:x}.{extension}")
}

#[cfg(target_os = "windows")]
fn notification_image_extension(content_type: &str) -> Option<&'static str> {
    match content_type
        .split(';')
        .next()?
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpg"),
        "image/gif" => Some("gif"),
        _ => None,
    }
}

#[cfg(target_os = "windows")]
fn show_windows_notification<R: Runtime>(
    app: &tauri::AppHandle<R>,
    title: &str,
    body: &str,
    image_path: Option<&std::path::Path>,
) -> Result<(), MediaIoError> {
    fn build(title: &str, body: &str, app_id: Option<&str>) -> notify_rust::Notification {
        let mut notification = notify_rust::Notification::new();
        notification.summary(title).body(body);
        if let Some(app_id) = app_id {
            notification.app_id(app_id);
        }
        notification
    }

    let app_id = tauri::utils::platform::current_exe()
        .ok()
        .and_then(|path| path.parent().map(std::path::Path::to_path_buf))
        .filter(|directory| {
            let value = directory.to_string_lossy();
            !value.ends_with("\\target\\debug") && !value.ends_with("\\target\\release")
        })
        .map(|_| app.config().identifier.as_str());
    let mut notification = build(title, body, app_id);
    if let Some(path) = image_path {
        notification.image_path(&path.to_string_lossy());
    }
    match notification.show() {
        Ok(_) => Ok(()),
        Err(_) if image_path.is_some() => build(title, body, app_id)
            .show()
            .map(|_| ())
            .map_err(|_| MediaIoError::operation("notification delivery failed", true)),
        Err(_) => Err(MediaIoError::operation(
            "notification delivery failed",
            true,
        )),
    }
}

pub struct UnavailableNotificationPort;

#[async_trait::async_trait]
impl NotificationPort for UnavailableNotificationPort {
    async fn permission(&self) -> Result<NotificationPermission, MediaIoError> {
        Err(MediaIoError::unavailable(
            "notification service is unavailable",
        ))
    }

    async fn request_permission(&self) -> Result<NotificationPermission, MediaIoError> {
        Err(MediaIoError::unavailable(
            "notification service is unavailable",
        ))
    }

    async fn show(&self, _: NotificationRequest) -> Result<(), MediaIoError> {
        Err(MediaIoError::unavailable(
            "notification service is unavailable",
        ))
    }
}

const fn map_permission(
    value: tauri_plugin_notification::PermissionState,
) -> NotificationPermission {
    match value {
        tauri_plugin_notification::PermissionState::Granted => NotificationPermission::Granted,
        tauri_plugin_notification::PermissionState::Denied => NotificationPermission::Denied,
        tauri_plugin_notification::PermissionState::Prompt
        | tauri_plugin_notification::PermissionState::PromptWithRationale => {
            NotificationPermission::Prompt
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{notification_image_cache_name, validated_gift_image_url};

    #[test]
    fn accepts_only_https_bilibili_gift_image_urls() {
        assert!(validated_gift_image_url("https://i0.hdslb.com/bfs/live/gift.png").is_some());
        assert!(validated_gift_image_url("https://static.bilibili.com/gift.png").is_some());
        assert!(validated_gift_image_url("http://i0.hdslb.com/gift.png").is_none());
        assert!(validated_gift_image_url("https://hdslb.com.evil.test/gift.png").is_none());
        assert!(validated_gift_image_url("https://user@i0.hdslb.com/gift.png").is_none());
    }

    #[test]
    fn derives_a_stable_path_safe_cache_name() {
        let first = notification_image_cache_name("https://i0.hdslb.com/bfs/live/gift.png", "png");
        let second = notification_image_cache_name("https://i0.hdslb.com/bfs/live/gift.png", "png");
        assert_eq!(first, second);
        assert!(first.ends_with(".png"));
        let digest = first.strip_suffix(".png").unwrap();
        assert_eq!(digest.len(), 64);
        assert!(digest.chars().all(|value| value.is_ascii_hexdigit()));
    }
}
