use crate::application::media::{
    MediaIoError, NotificationPermission, NotificationPort, NotificationRequest,
};
use tauri::Runtime;
use tauri_plugin_notification::NotificationExt;

pub struct TauriNotificationPort<R: Runtime> {
    app: tauri::AppHandle<R>,
}

impl<R: Runtime> TauriNotificationPort<R> {
    pub fn new(app: tauri::AppHandle<R>) -> Self {
        Self { app }
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
        self.app
            .notification()
            .builder()
            .title(request.title)
            .body(request.body)
            .show()
            .map_err(|_| MediaIoError::operation("notification delivery failed", true))
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
