use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::{
    application::shell::{ConfigMutation, ShellError, ShellEventSink, SystemMetrics},
    domain::shell::{
        is_config_scope_available, LocalePreferences, ShellConfigScope, UiPreferences,
    },
    ipc::error::DesktopError,
    AppState,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMetricsDto {
    pub memory_bytes: Option<u64>,
    pub cpu_percent: Option<f64>,
    pub observed_at: i64,
}

impl From<SystemMetrics> for SystemMetricsDto {
    fn from(value: SystemMetrics) -> Self {
        Self {
            memory_bytes: value.memory_bytes,
            cpu_percent: value.cpu_percent,
            observed_at: value.observed_at,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigScopeOptionDto {
    pub id: ShellConfigScope,
    pub is_available: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOverviewDto {
    pub display_path: String,
    pub last_backup_at: Option<i64>,
    pub scopes: Vec<ConfigScopeOptionDto>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigExportRequestDto {
    pub grant_id: String,
    pub scopes: Vec<ShellConfigScope>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigImportRequestDto {
    pub grant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigExportResultDto {
    pub revision: String,
    pub display_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigImportResultDto {
    pub revision: String,
    pub imported_scopes: Vec<ShellConfigScope>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResetResultDto {
    pub revision: String,
    pub reset_scopes: Vec<ShellConfigScope>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocaleAutoRequestDto {
    pub is_enabled: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocaleSaveRequestDto {
    pub preferences: LocalePreferences,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiPreferencesRequestDto {
    pub preferences: UiPreferences,
}

#[tauri::command]
pub async fn shell_status(state: State<'_, AppState>) -> Result<SystemMetricsDto, DesktopError> {
    state
        .shell()
        .status()
        .await
        .map(Into::into)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn config_overview(
    state: State<'_, AppState>,
) -> Result<ConfigOverviewDto, DesktopError> {
    state
        .shell()
        .config_overview()
        .await
        .map(|value| ConfigOverviewDto {
            display_path: value.display_path,
            last_backup_at: value.last_backup_at,
            scopes: value
                .scopes
                .into_iter()
                .map(|id| ConfigScopeOptionDto {
                    id,
                    is_available: is_config_scope_available(id),
                })
                .collect(),
        })
        .map_err(Into::into)
}

#[tauri::command]
pub async fn config_open_directory(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, DesktopError> {
    state
        .shell()
        .open_config_directory()
        .map_err(DesktopError::from)?;
    Ok(serde_json::json!({ "opened": true }))
}

#[tauri::command]
pub async fn config_export(
    state: State<'_, AppState>,
    request: ConfigExportRequestDto,
) -> Result<ConfigExportResultDto, DesktopError> {
    let path = state.paths().resolve(
        &request.grant_id,
        crate::application::paths::PathPurpose::ConfigExport,
    )?;
    let result = state.shell().export_config(&path, &request.scopes).await?;
    config_export_result(result)
}

#[tauri::command]
pub async fn config_import(
    state: State<'_, AppState>,
    request: ConfigImportRequestDto,
) -> Result<ConfigImportResultDto, DesktopError> {
    let path = state.paths().resolve(
        &request.grant_id,
        crate::application::paths::PathPurpose::ConfigImport,
    )?;
    let result = state.shell().import_config(&path).await?;
    Ok(ConfigImportResultDto {
        revision: result.revision,
        imported_scopes: result.scopes,
    })
}

#[tauri::command]
pub async fn config_reset(
    state: State<'_, AppState>,
) -> Result<ConfigResetResultDto, DesktopError> {
    let result = state.shell().reset_config().await?;
    Ok(ConfigResetResultDto {
        revision: result.revision,
        reset_scopes: result.scopes,
    })
}

#[tauri::command]
pub async fn locale_get(state: State<'_, AppState>) -> Result<LocalePreferences, DesktopError> {
    Ok(state.shell().locale().await)
}

#[tauri::command]
pub async fn locale_set_auto(
    state: State<'_, AppState>,
    request: LocaleAutoRequestDto,
) -> Result<LocalePreferences, DesktopError> {
    state
        .shell()
        .set_locale_auto(request.is_enabled)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn locale_save(
    state: State<'_, AppState>,
    request: LocaleSaveRequestDto,
) -> Result<LocalePreferences, DesktopError> {
    state
        .shell()
        .save_locale(request.preferences)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ui_get(state: State<'_, AppState>) -> Result<UiPreferences, DesktopError> {
    Ok(state.shell().ui().await)
}

#[tauri::command]
pub async fn ui_preview(
    state: State<'_, AppState>,
    request: UiPreferencesRequestDto,
) -> Result<UiPreferences, DesktopError> {
    state
        .shell()
        .preview_ui(request.preferences)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ui_save(
    state: State<'_, AppState>,
    request: UiPreferencesRequestDto,
) -> Result<UiPreferences, DesktopError> {
    state
        .shell()
        .save_ui(request.preferences)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn ui_reset(state: State<'_, AppState>) -> Result<UiPreferences, DesktopError> {
    state.shell().reset_ui().await.map_err(Into::into)
}

fn config_export_result(result: ConfigMutation) -> Result<ConfigExportResultDto, DesktopError> {
    let display_path = result.display_path.ok_or_else(|| {
        DesktopError::new(
            "config_export_failed",
            "Configuration export returned no destination",
            false,
        )
    })?;
    Ok(ConfigExportResultDto {
        revision: result.revision,
        display_path,
    })
}

pub struct TauriShellEventSink {
    app: AppHandle,
}

impl TauriShellEventSink {
    pub const fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl ShellEventSink for TauriShellEventSink {
    fn publish_locale(&self, value: LocalePreferences) -> Result<(), ShellError> {
        self.app
            .emit_to("main", "locale_changed", value)
            .map_err(|error| ShellError::new("shell_event_failed", error.to_string(), true))
    }

    fn publish_ui(&self, value: &UiPreferences) -> Result<(), ShellError> {
        self.app
            .emit_to("main", "ui_preferences_changed", value)
            .map_err(|error| ShellError::new("shell_event_failed", error.to_string(), true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::shell::{SHELL_COMMAND_NAMES, SHELL_EVENT_NAMES};

    #[test]
    fn shell_surface_is_exact_and_dtos_use_camel_case() {
        assert_eq!(SHELL_COMMAND_NAMES.len(), 13);
        assert_eq!(
            SHELL_EVENT_NAMES,
            [
                "shell_status_changed",
                "locale_changed",
                "ui_preferences_changed"
            ]
        );
        let value = serde_json::to_value(SystemMetricsDto {
            memory_bytes: Some(1),
            cpu_percent: None,
            observed_at: 2,
        })
        .unwrap();
        assert_eq!(
            value,
            serde_json::json!({ "memoryBytes": 1, "cpuPercent": null, "observedAt": 2 })
        );
    }
}
