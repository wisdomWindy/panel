# Specification

## Behavior

1. `GiftEvent` and `GiftRecord` own `gift_icon_url: Option<String>`; DTO/TypeScript expose `giftIconUrl: string | null`.
2. Open Live maps blank `gift_icon` to `None`, nonblank values to `Some`; preview uses `None`.
3. Local popup shows the image when present. Image load failure hides the broken image and reveals the existing present icon fallback. Text reads sender, gift name, quantity, and value.
4. System notification title is `礼物提醒`; body is `<username> 送出 <gift_name> ×<quantity>`.
5. Notification image URLs must be HTTPS and hosted by `bilibili.com` or `hdslb.com` subdomains. Valid images are downloaded with timeout and size bounds into app cache. Any validation/download/write/image-delivery failure retries notification without the image.
6. On Windows, cached image delivery uses `notify-rust::Notification::image_path`; other platforms keep the existing Tauri plugin path.
7. Built-in alert is a short ascending four-note major cue. Bound custom files remain first priority; `sound=false` remains silent.

## Field Semantics

- `giftIconUrl` is the Bilibili-provided visual identity of the gift, not the sender avatar.
- Null or failed images render a neutral gift icon, never an empty/broken media box.
- Gift name is business-critical and remains visible independently of image success.

## Failure And Security

- Image fetching is best effort and cannot block local popup, sound, or text-only system notification.
- Reject non-HTTPS, credential-bearing, non-Bilibili hosts, non-image content, and oversized responses.
- Cache keys are SHA-256 of URL; user-controlled URL text is never used as a path.

## Pattern Fit

Level 1 local lightweight shape. Reuse existing adapter/port boundaries; add no manager/factory. Image-cache logic stays inside the notification infrastructure owner because it has one production caller and exists solely to satisfy that side effect.

## Acceptance Criteria

- AC1 mapper and DTO preserve gift image nullability.
- AC2 local popup renders gift image and fallback plus gift name.
- AC3 system notification text includes sender, gift name, and quantity.
- AC4 allowed image produces an image path; invalid/failing image still produces a text notification.
- AC5 built-in alert sequence is ascending and custom/silent priorities are unchanged.
