use crate::application::media::{EffectWindowPort, EffectWindowSpec, MediaIoError};
use tauri::{Manager, Runtime, WebviewUrl, WebviewWindowBuilder};

pub struct TauriEffectWindowPort<R: Runtime> {
    app: tauri::AppHandle<R>,
}

impl<R: Runtime> TauriEffectWindowPort<R> {
    pub fn new(app: tauri::AppHandle<R>) -> Self {
        Self { app }
    }

    fn window(&self, label: &str) -> Result<tauri::WebviewWindow<R>, MediaIoError> {
        self.app
            .get_webview_window(label)
            .ok_or_else(|| MediaIoError::unavailable("effect window is unavailable"))
    }
}

#[async_trait::async_trait]
impl<R: Runtime> EffectWindowPort for TauriEffectWindowPort<R> {
    async fn ensure(&self, spec: &EffectWindowSpec) -> Result<(), MediaIoError> {
        if spec.label.trim().is_empty() || spec.width == 0 || spec.height == 0 {
            return Err(MediaIoError::invalid(
                "effect window specification is invalid",
            ));
        }
        if self.app.get_webview_window(&spec.label).is_some() {
            return Ok(());
        }
        WebviewWindowBuilder::new(
            &self.app,
            &spec.label,
            WebviewUrl::App("index.html?surface=effect".into()),
        )
        .inner_size(f64::from(spec.width), f64::from(spec.height))
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .focused(false)
        .visible(false)
        .build()
        .map(|_| ())
        .map_err(|_| MediaIoError::unavailable("effect window could not be created"))
    }

    async fn show(&self, label: &str) -> Result<(), MediaIoError> {
        self.window(label)?
            .show()
            .map_err(|_| MediaIoError::operation("effect window could not be shown", true))
    }

    async fn close(&self, label: &str) -> Result<(), MediaIoError> {
        match self.app.get_webview_window(label) {
            Some(window) => window
                .close()
                .map_err(|_| MediaIoError::operation("effect window could not be closed", true)),
            None => Ok(()),
        }
    }
}

pub struct UnavailableEffectWindowPort;

#[async_trait::async_trait]
impl EffectWindowPort for UnavailableEffectWindowPort {
    async fn ensure(&self, _: &EffectWindowSpec) -> Result<(), MediaIoError> {
        Err(MediaIoError::unavailable("effect window is unavailable"))
    }

    async fn show(&self, _: &str) -> Result<(), MediaIoError> {
        Err(MediaIoError::unavailable("effect window is unavailable"))
    }

    async fn close(&self, _: &str) -> Result<(), MediaIoError> {
        Ok(())
    }
}
