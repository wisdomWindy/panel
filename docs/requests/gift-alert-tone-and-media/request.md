# Gift Alert Tone And Media

## Goal

Make the built-in gift alert sound cheerful and show the gift name and gift image in both local and system notifications.

## Done Signals

- The default alert uses a short bright ascending melody while custom sounds remain first priority.
- `gift_icon` reaches the renderer and local popup, with an icon fallback when unavailable.
- System notification text includes sender, gift name, and quantity; supported Windows notifications receive a cached local gift image.
- Image failures degrade to the existing icon/no-image path and never suppress notification delivery.
- Focused contract, mapping, audio, notification, full regression, and package checks pass.

## Non-goals

- No user-selectable melody catalog.
- No persistent gift catalog or manual image upload.
- No unrelated reminder layout redesign.
