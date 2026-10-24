# Code Context

Text search fallback was used; no repository code-graph tool was available in the active tool surface.

## Current Flow

`open_live/event_mapper.rs` -> `LiveEvent::Gift` -> media `GiftRecord` -> `GiftEventDto` -> renderer adapter -> `GiftAlertOverlay.vue`.

System notification: `GiftWorker` builds `NotificationRequest` -> `TauriNotificationPort` -> `tauri-plugin-notification` -> `notify-rust`.

Audio: `GiftWorker` selects custom binding first -> `AudioOutputPort::play_alert` fallback -> Rodio synthesized tones.

## Facts

- Upstream Open Live gift contract contains `gift_icon`.
- Current domain event, record, DTO, and TypeScript contract omit it.
- Renderer CSP currently omits remote HTTPS images.
- Tauri notification desktop builder exposes `icon`, but on Windows `notify-rust` renders notification media via a local `image_path`; a direct `notify-rust` call is required for reliable gift images.
- `reqwest`, `sha2`, and Tauri app path resolution are already available.

## Affected Owners

- Domain/event contract: `domain/live_data/models.rs`, `domain/media/models.rs`.
- Boundary mapping: Open Live mapper, IPC DTO/mappers, renderer media adapter.
- Side effects: notification infrastructure and Rodio audio infrastructure.
- UI: existing `GiftAlertOverlay.vue` only; no new component boundary.
