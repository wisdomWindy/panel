fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "app_bootstrap",
            "app_status",
            "config_get",
            "config_set",
            "path_select",
            "path_revoke",
        ]),
    ))
    .expect("failed to build Tauri application manifest")
}
