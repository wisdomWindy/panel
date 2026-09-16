use crate::application::media::{HotkeyCallback, HotkeyPort, HotkeyRegistration, MediaIoError};
use tauri::Runtime;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

pub struct TauriHotkeyPort<R: Runtime> {
    app: tauri::AppHandle<R>,
}

impl<R: Runtime> TauriHotkeyPort<R> {
    pub fn new(app: tauri::AppHandle<R>) -> Self {
        Self { app }
    }
}

impl<R: Runtime> HotkeyPort for TauriHotkeyPort<R> {
    fn register(
        &self,
        shortcut: &str,
        callback: HotkeyCallback,
    ) -> Result<Box<dyn HotkeyRegistration>, MediaIoError> {
        if shortcut.trim().is_empty() {
            return Err(MediaIoError::invalid("shortcut is empty"));
        }
        if self.app.global_shortcut().is_registered(shortcut) {
            return Err(MediaIoError::conflict("shortcut is already registered"));
        }
        let registered = shortcut.to_owned();
        self.app
            .global_shortcut()
            .on_shortcut(shortcut, move |_, _, event| {
                if event.state == ShortcutState::Pressed {
                    callback();
                }
            })
            .map_err(|_| MediaIoError::conflict("shortcut could not be registered"))?;
        Ok(Box::new(TauriHotkeyRegistration {
            app: self.app.clone(),
            shortcut: registered,
            active: true,
        }))
    }
}

struct TauriHotkeyRegistration<R: Runtime> {
    app: tauri::AppHandle<R>,
    shortcut: String,
    active: bool,
}

impl<R: Runtime> HotkeyRegistration for TauriHotkeyRegistration<R> {
    fn unregister(&mut self) -> Result<(), MediaIoError> {
        if !self.active {
            return Ok(());
        }
        self.app
            .global_shortcut()
            .unregister(self.shortcut.as_str())
            .map_err(|_| MediaIoError::operation("shortcut could not be unregistered", true))?;
        self.active = false;
        Ok(())
    }
}

impl<R: Runtime> Drop for TauriHotkeyRegistration<R> {
    fn drop(&mut self) {
        let _ = self.unregister();
    }
}

pub struct UnavailableHotkeyPort;

impl HotkeyPort for UnavailableHotkeyPort {
    fn register(
        &self,
        _: &str,
        _: HotkeyCallback,
    ) -> Result<Box<dyn HotkeyRegistration>, MediaIoError> {
        Err(MediaIoError::unavailable(
            "global shortcut service is unavailable",
        ))
    }
}
