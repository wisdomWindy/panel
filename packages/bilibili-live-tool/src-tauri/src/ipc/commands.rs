use std::path::PathBuf;

use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use crate::{
    application::paths::PathPurpose,
    ipc::{
        dto::{
            AppStatusDto, ConfigGetRequestDto, ConfigRevisionDto, ConfigSetRequestDto,
            ConfigValueDto, PathGrantDto, PathRevokeRequestDto, PathRevokeResultDto,
            PathSelectRequestDto,
        },
        error::DesktopError,
    },
    AppState,
};

#[tauri::command]
pub async fn app_bootstrap(state: State<'_, AppState>) -> Result<AppStatusDto, DesktopError> {
    state
        .application()
        .bootstrap()
        .await
        .map(AppStatusDto::from)
        .map_err(DesktopError::from)
}

#[tauri::command]
pub async fn app_status(state: State<'_, AppState>) -> Result<AppStatusDto, DesktopError> {
    Ok(AppStatusDto::from(state.application().status().await))
}

#[tauri::command]
pub async fn config_get(
    state: State<'_, AppState>,
    request: ConfigGetRequestDto,
) -> Result<ConfigValueDto, DesktopError> {
    state
        .application()
        .get_config(&request.key, &request.scope)
        .await
        .map(ConfigValueDto::from)
        .map_err(DesktopError::from)
}

#[tauri::command]
pub async fn config_set(
    state: State<'_, AppState>,
    request: ConfigSetRequestDto,
) -> Result<ConfigRevisionDto, DesktopError> {
    state
        .application()
        .set_config(&request.key, &request.scope, &request.value)
        .await
        .map(|revision| ConfigRevisionDto { revision })
        .map_err(DesktopError::from)
}

#[tauri::command]
pub async fn path_select(
    app: AppHandle,
    state: State<'_, AppState>,
    request: PathSelectRequestDto,
) -> Result<PathGrantDto, DesktopError> {
    let selection = pick_path(&app, request.purpose)?;
    state
        .paths()
        .select(selection, request.purpose)
        .map_err(DesktopError::from)?
        .map(PathGrantDto::from)
        .ok_or_else(|| DesktopError::new("SYS_DIALOG_CANCELLED", "Path selection was cancelled", false))
}

#[tauri::command]
pub async fn path_revoke(
    state: State<'_, AppState>,
    request: PathRevokeRequestDto,
) -> Result<PathRevokeResultDto, DesktopError> {
    state
        .paths()
        .revoke(&request.grant_id)
        .map(|revoked| PathRevokeResultDto { revoked })
        .map_err(DesktopError::from)
}

fn pick_path(app: &AppHandle, purpose: PathPurpose) -> Result<Option<PathBuf>, DesktopError> {
    let file_dialog = app.dialog().file();
    let selection = if purpose.is_save_file() {
        file_dialog.blocking_save_file()
    } else if purpose.expects_directory() {
        file_dialog.blocking_pick_folder()
    } else {
        file_dialog.blocking_pick_file()
    };

    selection
        .map(|path| {
            path.into_path().map_err(|_| {
                DesktopError::new(
                    "invalid_selected_path",
                    "The selected path is not a local filesystem path",
                    false,
                )
            })
        })
        .transpose()
}
