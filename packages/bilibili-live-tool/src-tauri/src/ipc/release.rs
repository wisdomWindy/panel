use serde::Deserialize;
use tauri::{Emitter, State};

use crate::{
    application::{
        release::{AboutSnapshot, ResilienceSnapshot, UpdateSnapshot},
        release_ports::ReleaseEventSink,
    },
    ipc::error::DesktopError,
    AppState,
};

pub struct TauriReleaseEventSink {
    app: tauri::AppHandle,
}

impl TauriReleaseEventSink {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self { app }
    }

    fn emit<T: serde::Serialize + Clone>(&self, event: &str, payload: &T) {
        if let Err(error) = self.app.emit(event, payload) {
            log::warn!("release event emission failed for {event}: {error}");
        }
    }
}

impl ReleaseEventSink for TauriReleaseEventSink {
    fn emit_update_status(&self, snapshot: &UpdateSnapshot) {
        self.emit("update://status", snapshot);
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSkipRequest {
    pub version: String,
}

#[tauri::command]
pub async fn update_status(state: State<'_, AppState>) -> Result<UpdateSnapshot, DesktopError> {
    Ok(state.release().update_status().await)
}

#[tauri::command]
pub async fn update_check(state: State<'_, AppState>) -> Result<UpdateSnapshot, DesktopError> {
    state.release().check_update().await.map_err(Into::into)
}

#[tauri::command]
pub async fn update_download(state: State<'_, AppState>) -> Result<UpdateSnapshot, DesktopError> {
    state.release().download_update().await.map_err(Into::into)
}

#[tauri::command]
pub async fn update_install(state: State<'_, AppState>) -> Result<UpdateSnapshot, DesktopError> {
    state.release().install_update().await.map_err(Into::into)
}

#[tauri::command]
pub async fn update_skip(
    state: State<'_, AppState>,
    request: UpdateSkipRequest,
) -> Result<UpdateSnapshot, DesktopError> {
    state
        .release()
        .skip_update(&request.version)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn about_info(state: State<'_, AppState>) -> Result<AboutSnapshot, DesktopError> {
    Ok(state.release().about())
}

#[tauri::command]
pub async fn resilience_status(
    state: State<'_, AppState>,
) -> Result<ResilienceSnapshot, DesktopError> {
    Ok(state.release().resilience())
}
