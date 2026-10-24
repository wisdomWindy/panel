use crate::{
    application::media::{EffectWindowPort, EffectWindowSpec, MediaIoError},
    domain::media::MediaPosition,
};
use tauri::{Manager, Runtime, WebviewUrl, WebviewWindowBuilder};

const WINDOW_MARGIN: f64 = 24.0;

#[derive(Clone, Copy, Debug)]
struct LogicalMonitorGeometry {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn logical_window_position(
    monitor: LogicalMonitorGeometry,
    window_width: f64,
    window_height: f64,
    position: MediaPosition,
) -> (f64, f64) {
    let horizontal_space = (monitor.width - window_width).max(0.0);
    let vertical_space = (monitor.height - window_height).max(0.0);
    let left = monitor.x + WINDOW_MARGIN.min(horizontal_space);
    let right = monitor.x + (horizontal_space - WINDOW_MARGIN).max(0.0);
    let top = monitor.y + WINDOW_MARGIN.min(vertical_space);
    let bottom = monitor.y + (vertical_space - WINDOW_MARGIN).max(0.0);
    let center_x = monitor.x + horizontal_space / 2.0;
    let center_y = monitor.y + vertical_space / 2.0;

    match position {
        MediaPosition::TopLeft => (left, top),
        MediaPosition::TopRight => (right, top),
        MediaPosition::BottomLeft => (left, bottom),
        MediaPosition::BottomRight => (right, bottom),
        MediaPosition::Center => (center_x, center_y),
    }
}

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
        let monitor = self
            .app
            .primary_monitor()
            .map_err(|_| MediaIoError::unavailable("primary monitor is unavailable"))?
            .ok_or_else(|| MediaIoError::unavailable("primary monitor is unavailable"))?;
        let scale_factor = monitor.scale_factor();
        let monitor_position = monitor.position().to_logical::<f64>(scale_factor);
        let monitor_size = monitor.size().to_logical::<f64>(scale_factor);
        let (x, y) = logical_window_position(
            LogicalMonitorGeometry {
                x: monitor_position.x,
                y: monitor_position.y,
                width: monitor_size.width,
                height: monitor_size.height,
            },
            f64::from(spec.width),
            f64::from(spec.height),
            spec.position,
        );
        WebviewWindowBuilder::new(
            &self.app,
            &spec.label,
            WebviewUrl::App("index.html?surface=effect".into()),
        )
        .inner_size(f64::from(spec.width), f64::from(spec.height))
        .position(x, y)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::media::MediaPosition;

    const MONITOR: LogicalMonitorGeometry = LogicalMonitorGeometry {
        x: 100.0,
        y: 50.0,
        width: 1920.0,
        height: 1080.0,
    };

    #[test]
    fn maps_each_effect_position_inside_the_primary_monitor() {
        assert_eq!(
            logical_window_position(MONITOR, 560.0, 280.0, MediaPosition::TopLeft),
            (124.0, 74.0)
        );
        assert_eq!(
            logical_window_position(MONITOR, 560.0, 280.0, MediaPosition::TopRight),
            (1436.0, 74.0)
        );
        assert_eq!(
            logical_window_position(MONITOR, 560.0, 280.0, MediaPosition::BottomLeft),
            (124.0, 826.0)
        );
        assert_eq!(
            logical_window_position(MONITOR, 560.0, 280.0, MediaPosition::BottomRight),
            (1436.0, 826.0)
        );
        assert_eq!(
            logical_window_position(MONITOR, 560.0, 280.0, MediaPosition::Center),
            (780.0, 450.0)
        );
    }

    #[test]
    fn keeps_an_oversized_effect_window_at_the_monitor_origin() {
        assert_eq!(
            logical_window_position(MONITOR, 2560.0, 1440.0, MediaPosition::BottomRight),
            (100.0, 50.0)
        );
    }
}
