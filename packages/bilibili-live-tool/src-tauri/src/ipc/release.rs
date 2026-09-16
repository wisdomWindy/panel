use serde::Deserialize;
use tauri::{Emitter, State};

use crate::{
    application::{
        release::{AboutSnapshot, PluginSnapshot, ResilienceSnapshot, UpdateSnapshot},
        release_ports::{ReleaseEventSink, ReleaseServiceError},
    },
    domain::release::is_valid_plugin_id,
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

    fn emit_plugin_status(&self, snapshot: &PluginSnapshot) {
        self.emit("plugin://status", snapshot);
    }

    fn emit_plugin_crashed(&self, snapshot: &PluginSnapshot) {
        self.emit("plugin://crashed", snapshot);
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSkipRequest {
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginIdRequest {
    pub plugin_id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginToggleRequest {
    pub plugin_id: String,
    pub is_enabled: bool,
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
pub async fn plugin_list(state: State<'_, AppState>) -> Result<Vec<PluginSnapshot>, DesktopError> {
    Ok(state.release().plugins().await)
}

#[tauri::command]
pub async fn plugin_select_install(
    state: State<'_, AppState>,
) -> Result<PluginSnapshot, DesktopError> {
    state
        .release()
        .select_plugin_install()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn plugin_install(
    state: State<'_, AppState>,
    request: PluginIdRequest,
) -> Result<PluginSnapshot, DesktopError> {
    state
        .release()
        .install_plugin(&request.plugin_id)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn plugin_toggle(
    state: State<'_, AppState>,
    request: PluginToggleRequest,
) -> Result<PluginSnapshot, DesktopError> {
    state
        .release()
        .toggle_plugin(&request.plugin_id, request.is_enabled)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn plugin_uninstall(
    state: State<'_, AppState>,
    request: PluginIdRequest,
) -> Result<bool, DesktopError> {
    state
        .release()
        .uninstall_plugin(&request.plugin_id)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn plugin_config_get(request: PluginIdRequest) -> Result<bool, DesktopError> {
    validate_plugin_id(&request.plugin_id)?;
    Err(ReleaseServiceError::unavailable("plugin_config_unavailable").into())
}

#[tauri::command]
pub async fn plugin_config_save(request: PluginIdRequest) -> Result<bool, DesktopError> {
    validate_plugin_id(&request.plugin_id)?;
    Err(ReleaseServiceError::unavailable("plugin_config_unavailable").into())
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

fn validate_plugin_id(value: &str) -> Result<(), DesktopError> {
    if !is_valid_plugin_id(value) {
        return Err(ReleaseServiceError::invalid("invalid_plugin_id").into());
    }
    Ok(())
}
