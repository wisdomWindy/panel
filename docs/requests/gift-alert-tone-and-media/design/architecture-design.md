# Architecture Design

## Ownership

- Upstream decoding owns `gift_icon` extraction.
- Domain owns optional gift image identity.
- IPC/renderer adapters own camelCase transport validation.
- `GiftAlertOverlay` owns visual load fallback state.
- `TauriNotificationPort` owns fetch/cache/platform notification delivery.
- `RodioAudioOutput` owns melody synthesis.

## Data Flow

`gift_icon` -> `LiveEvent::Gift.gift_icon_url` -> `GiftRecord.gift_icon_url` -> `GiftEventDto.gift_icon_url` -> `GiftEvent.giftIconUrl` -> popup image and notification request.

## Dependencies

- Domain remains independent of infrastructure.
- Application passes URL data through `NotificationRequest`; it does not fetch or resolve paths.
- Infrastructure may use `reqwest`, app cache paths, and platform-specific `notify-rust` behavior.

## Complexity Budget

One optional field across existing mappings, one bounded cache helper in the notification module, one local image-error flag in the existing popup, and a constant note sequence in audio infrastructure. No new services or shared utility modules.
