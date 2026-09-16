# Extensions Release And Resilience Page Design

## Delivery Unit

- request: `tauri-rust-refactor`
- module: `extensions-release-resilience`
- source: PRD 9.17-9.19, 10, structured snapshot 11/13-15, requirement module
- scope: configuration management, update log/prompt, plugin center/configuration, About/licenses

## Objectives And Journeys

- Configuration: inspect the native data location and last backup, select source-listed scopes, export/import through system dialogs, or reset only after confirmation.
- Update: inspect current/latest versions and release notes, then download, defer/skip when optional, or restart/install after verified download; forced updates omit defer.
- Plugin: filter installed/updateable/disabled WASM plugins, inspect declared permissions, install a signed package, configure, toggle, refresh, or confirm uninstall.
- About: inspect actual application/channel version and distributed third-party license/source information.

Task success is a native-confirmed state change. Missing manifests, keys, binaries, or desktop bridge end in an explicit unavailable/error state and never a simulated success.

## Source And Product-Content Allowlist

| Surface | Allowed content | Source |
| --- | --- | --- |
| Configuration | storage path, open directory, last backup, listed config scopes, export/import/reset, reset confirmation, corrupt/incompatible/rollback errors | PRD 9.17 |
| Update | current/latest version, update content, progress, immediate update, later, skip version, restart/install, retry/network/signature/rollback states | PRD 9.18 |
| Plugin | installed/updateable/disabled tabs; name/version/status/permission; install/refresh/configure/toggle/uninstall; sandbox safety notice and signature/permission/crash errors | PRD 9.19 |
| About | FFmpeg LGPL license text/source address; Whisper license when actually distributed | PRD 9.13.3, 9.14 |
| Shared | offline update checks are silent; destructive reset/uninstall uses confirmation; system dialog owns all file paths | PRD 9.17-9.19, 10 |

No sample changelog, plugin catalog, provider URL, public key, license body, success result, help text, crash-upload consent, or arbitrary path input may be invented. These remain `missing-source` or `external-interface-pending`.

## Information Architecture And Layout

- Keep the existing four-region application shell and existing navigation positions.
- Configuration remains an unframed settings section: path/backup definition list, scope checkbox grid, then action bar with destructive reset separated at the end.
- Update log is an unframed operational page: compact version/status header, release-note stream, and a modal only when a native update candidate exists.
- Plugin center is a dense management workspace: title/actions, segmented tabs, horizontally scrollable table, persistent security notice. Plugin details/configuration and install permission review use dialogs.
- About/licenses is a settings section, not a marketing page. Application metadata precedes a license list; missing artifacts are shown as unavailable rather than substituted text.

## Visual Hierarchy And Direction

- Quiet operational UI using existing Element Plus controls and neutral full-width surfaces.
- Page titles use compact `text-xl`; section headings use `text-sm font-semibold`; metadata uses subdued gray; destructive actions use danger styling.
- Status always uses text plus icon/tag, never color alone. Warnings use bordered alerts, not decorative cards or gradients.
- Tables prioritize name/version/status/permissions/actions and preserve predictable column widths.
- Loading uses control/table loading states without layout shifts. No decorative animation.

## Interaction And Feedback Model

- All mutations are pessimistic: disable the owning action while pending and update the list/snapshot only from validated native results.
- Config import preserves existing data on corrupt/incompatible/failing input; reset confirmation restores focus to its trigger.
- Update download exposes native progress. Signature failure cannot expose install. Forced update hides Later; offline detection has no visible interruption.
- Plugin install begins with the native file dialog, then shows verified manifest permissions before confirmation. Signature/permission rejection leaves the list unchanged.
- Toggle rolls back visually on failure. Uninstall is confirmed and removes a row only after native success. Plugin crash retains the host page and marks only that plugin failed/disabled.
- Empty and filtered-empty plugin states use only source terminology; no catalog recommendation or installation tutorial.

## Responsive And Accessibility

- Target 1280x800 and minimum 1024x680; narrow content keeps action toolbars wrapping and tables horizontally scrollable.
- Dialog width is bounded by `min(90vw, 40rem)`; long permissions/release notes wrap and paths break safely.
- Icon buttons use existing Lucide/Element icons and accessible names/tooltips where the symbol is not self-evident.
- Native buttons, checkboxes, tabs, switches, progress and dialogs preserve keyboard semantics. Focus enters dialogs, returns to the trigger, and errors associate with the owning operation.
- Live progress and terminal failures use polite announcements; confirmations do not create keyboard traps.

## Styling Contract

- New/changed authored styling uses inline Tailwind CSS-style utility classes only.
- No scoped CSS, CSS modules, preprocessors, inline style objects, or semantic class aliases.
- Conditional class bindings are limited to short state toggles. Long utility values are reduced by simpler markup or a named component boundary, never hidden in constants/computed/helpers.
- Cards are not nested; page sections remain unframed.

## Component Decisions

| Candidate | Decision | Owner / reason | Contract |
| --- | --- | --- | --- |
| `UpdateAvailableDialog` | extract | one caller; interaction lifecycle, progress, forced-action and focus boundary | normalized update snapshot + pending; emits download/defer/skip/install |
| `PluginInstallDialog` | extract | one caller; system selection/permission review/confirmation lifecycle and focused security tests | verified manifest + pending; emits select/confirm/cancel |
| `PluginConfigDialog` | extract only if native schema exists | source allows configuration but schema is external pending | normalized schema only; never raw DTO |
| plugin table/filter | keep inline | single page orchestration and direct row actions; extraction would pass row DTO/emits | page/store owns |
| configuration panel | reuse existing | already matches the source workflow and native shell store | no new boundary |
| About/license panel | keep inline | small read-only one-caller section | settings owns |

All normalization, permission decisions, transport, and route orchestration remain outside presentational SFCs. No slots are needed.

## UX Risks And Controls

- Accidental destructive reset/uninstall: isolated danger placement, explicit source-backed confirmation, pending lock.
- False security confidence: show verified/native status only; absent keys/manifests remain unavailable.
- Permission overload: concise permission tags in table, full declared capability review before install.
- Update interruption: optional updates can defer/skip; forced update follows manifest policy; offline checks remain silent.
- Dense tables: stable widths, wrapping metadata and horizontal overflow.
- Stale mutation races: operation IDs/native snapshots and per-action pending ownership.

## Open UI Questions

- Exact update endpoint, public key, release-note payload, plugin signature key/package schema, configuration schema, FFmpeg source URL/license file and Whisper artifacts are external pending.
- No production UI may claim these resources are available until the corresponding signed/native contract exists.

## Exit Check

The design covers the source-authorized journeys, hierarchy, states, responsive/accessibility behavior, Tailwind constraints and Vue extraction decisions without adding product content. Advance to `spec`.
