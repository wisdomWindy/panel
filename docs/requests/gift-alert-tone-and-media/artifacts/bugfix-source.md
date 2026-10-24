# Bugfix Source

## Verbatim Evidence

1. `礼物提醒中的礼物提示音就不能欢快一点、愉悦一点`
2. `礼物提醒的本地弹框、系统通知中显示的文本添加礼物名称、礼物图片`
3. Confirmed design: propagate `gift_icon`; local popup uses the remote image with fallback; Windows system notification downloads a controlled cached image; text is `用户送出 礼物名 ×数量`; built-in alert becomes a bright ascending four-note cue.

## Observed

- The current built-in fallback is a descending two-tone sine cue.
- Local popup shows a generic present icon instead of the event gift image.
- The system notification body contains gift name and quantity, but its title/body do not form a complete “sender sent gift” message and no gift image is supplied.
- Bilibili Open Live payload already contains `gift_icon`, but the event mapper drops it.

## Expected

- A more cheerful built-in cue.
- Gift name and gift image visible in the local popup.
- Sender, gift name, and quantity visible in the system notification, with gift image when platform delivery supports it.

## Closed-world Allowlist

- Add the optional gift image URL across the existing gift event contract.
- Change the existing gift reminder display copy and image slot only.
- Add bounded HTTPS image caching for the notification adapter only.
- Replace only the built-in fallback melody.
