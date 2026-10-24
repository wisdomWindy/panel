# Execution Changelog: Hide Update Channel In About

## Delivery Unit

- Request: `hide-update-channel-about`
- Task: `T1`
- Pattern decision: Level 0 `direct code`

## Changes

- Added contract assertions that the About & Licenses template does not reference `settings.aboutInfo.channel` or `release.about?.channel`.
- Removed the update-channel `<dt>` and `<dd>` from the About & Licenses template.
- Removed the now-unused `settings.aboutInfo.channel` entries from `zh-CN.json` and `en-US.json`.
- Preserved the release channel contract, adapters, store state, update check, version display, and license display.

## TDD Evidence

- RED: focused settings contract test failed because `settings.aboutInfo.channel` was still present.
- GREEN: the same focused test passed with 4/4 tests after the template and locale cleanup.

## Validation Executed

- Related release/runtime/i18n tests: 21/21 passed.
- Locale JSON parse: passed.
- Type check: passed.
- Lint: passed with 0 errors and 2 pre-existing `no-console` warnings in `src/renderer/modules/utils/logger.ts`.
- Free web build: passed; existing chunk-size warning remains non-blocking.
- `git diff --check`: passed.
- Build-generated removal of the pre-existing `ElAvatar` declaration was restored; no unrelated generated-file delta remains from this task.

## Scope Notes

- No API, IPC, Rust, state, type, async, style, or architecture behavior changed.
- No installer was built as part of this request.
