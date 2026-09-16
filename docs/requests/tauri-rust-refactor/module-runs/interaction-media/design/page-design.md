# Interaction Media Page Design

## Delivery Unit Identifier

- Request: `tauri-rust-refactor`
- Module: `interaction-media`
- Sequence: 3 of 6
- Stage: page-design
- Status: approved design input for spec

## Page Objective

Provide a compact local media-operations workspace for configuring captain effects, gift notifications and gift sounds, controlling TTS, and recording a live room with highlights. Every effect, notification, sound, spoken item and recording remains local. Vue presents configuration and status while Rust owns queues, file access, notifications, TTS, Sidecar processes, quotas, hotkeys and cleanup.

## Target Users Or Scenarios

- A streamer configures and previews a captain effect before going live.
- An operator sets a high-value gift threshold, chooses local alert channels and monitors current-session gift totals.
- A streamer binds MP3/WAV files to gifts and checks playback without exposing arbitrary file access to the WebView.
- An operator configures spoken event types and identities, then pauses or clears the TTS queue during a stream.
- A streamer starts/stops recording, marks highlights with F9 or the page action, and manages completed recordings and clips.

## Source And Upstream Constraints

- Product authority: PRD 8, 9.6, 9.7, 9.8, 9.12, 9.13, 10, 11, 14 and 15.
- Module boundary: `requirements/modules/interaction-media.md`; no AI/ASR, membership, moderation, plugin, update or global-shell redesign.
- Prior module contract: accepted live events and the Rust-backed room/feed are the only event inputs. Blacklisted or blocked content never reaches TTS, alerts or media statistics.
- Platform contract: the WebView uses the typed desktop gateway. System dialogs return purpose-bound grants; Rust alone reads/writes files, launches FFmpeg, owns global F9 and creates the effect window.
- Existing code facts are migration evidence only: `/gift-sound`, `/recording` and `/effect-preview` exist; TTS and recording settings are duplicated under settings; media services and pages still invoke Electron or browser file APIs.
- External resources remain unavailable until configured: the repository does not ship the required LGPL shared FFmpeg artifact or a cloud TTS service contract. The UI must expose the PRD fallback/degraded states and must not simulate success.
- Global navigation and the persistent recording status placement are finalized by `application-shell`; this module defines the required entries/status contract without redesigning the whole shell.

## Product-Content Allowlist Mapping

| Surface | Approved content | Exact source |
| --- | --- | --- |
| Reminder navigation | `提醒配置` with `上舰特效`, `礼物提醒`, `礼物音效`; separate `语音播报` and `直播录制` entries | PRD 9.6.3, 9.7.3, 9.8.3, 9.12.3, 9.13.5 |
| Captain effect form | template with 10+ presets, preview, duration 3/5/8/10s, position left/right top/bottom/center, primary/secondary colors, first/renew/combo sound files, volume 0-1, combo merge, 5/10/30s window | PRD 9.6.3-9.6.4 |
| Captain effect actions | preview, sound preview, save/immediate effect, reset with second confirmation | PRD 9.6.5 |
| Effect window | transparent, borderless, always-on-top local window; user, level, first/renew type; default 5s fade; queued non-overlap; DPI/multi-display positioning | PRD 9.6.1-9.6.3, 9.6.6 |
| Effect fallback | missing sound silently keeps visuals; window creation failure uses a main-window overlay; DPI/display changes reposition | PRD 9.6.6 |
| Gift reminder form | master switch, high-value threshold, system notification/local popup/sound choices, combo merge/window, popup position/duration, save and test | PRD 9.7.3-9.7.5 |
| Gift reminder data | current-session total value, donor count, gift count; latest 20 gifts with user/gift/count/value/time; row detail with the same fields | PRD 9.7.3-9.7.5 |
| Gift reminder fallback | notification permission falls back to local popup; sound failure is silent and logged | PRD 9.7.6 |
| Gift sound toolbar | gift search, bound-only filter, batch import/export | PRD 9.8.3-9.8.5 |
| Gift sound rows | gift, bound sound file, per-gift volume, enabled state; preview, replace and remove | PRD 9.8.3-9.8.5 |
| Gift sound footer/actions | global volume, queue capacity `10 个/秒`, save, reset default; system-dialog file selection and second-confirmed removal | PRD 9.8.3-9.8.5 |
| Gift sound validation/failure | MP3/WAV only; missing file shown in red with rebind guidance; overflow drops oldest low-priority sound | PRD 9.8.1, 9.8.6 |
| TTS form | master switch, system/cloud engine, test, voice, speed, volume, event types danmaku/gift/captain/entry, identities admin/captain/VIP/ordinary, three fixed-variable templates, save/reset | PRD 9.12.2-9.12.5 |
| TTS queue | current spoken item, queued count, pause, clear pending queue; captain > gift > admin > ordinary priority | PRD 9.12.2-9.12.5 |
| TTS fallback | system unavailable switches to cloud; cloud failure degrades local and prompts once; overflow drops low priority; invalid template prompts and uses default | PRD 9.12.6 |
| Recording header/status | start/stop, settings, status, duration, file size, disk remaining/quota, FFmpeg source, F9 and manual highlight | PRD 9.13.5-9.13.7 |
| Recording settings | MP4/FLV, original/high/standard quality, dialog-authorized directory, segmentation switch/duration, auto-record, danmaku heat threshold, fixed highlight before/after 30s, default 10GB disk quota, default 5GB waterline, built-in/local FFmpeg | PRD 9.13.3-9.13.5 |
| Recording list | filename, size, duration; preview, rename and second-confirmed delete | PRD 9.13.5-9.13.7 |
| Highlight list | timestamp, automatic danmaku-surge/manual reason; preview, grant-backed export and second-confirmed delete | PRD 9.13.2, 9.13.5-9.13.7 |
| Recording failures | FFmpeg path/permission with local-source alternative; low space/waterline stops and warns; stream retry 3 then failure; quota cleans oldest buffer; F9 conflict exposes custom shortcut setting | PRD 9.13.8 |
| Recording compliance | LGPL shared FFmpeg, license/source disclosure in About, optional local FFmpeg | PRD 9.13.3, 9.13.9 |
| Forbidden | sending/chat interaction, frontend process/file access, browser downloads, direct paths as authority, simulated FFmpeg/cloud success, media handling before safety filtering | PRD 0.1, 2.2, 9.6-9.8, 9.12-9.13, 10, 15 |

Existing unapproved UI is excluded: effect text/template editing, render-mode/animation/test-data controls, gift-sound duration/play mode/cooldown/priority and arbitrary add dialog, pasted JSON import, recording pause/resume, direct browser download/open-folder substitutions, cleanup-day controls, gift/captain highlight triggers and duplicate TTS/recording settings forms.

## User Journey And Task Success

1. Before a stream, the user opens `提醒配置 -> 上舰特效`, chooses only PRD-approved options, previews the effect or a selected sound, then saves. Success means the persisted Rust configuration is immediately active.
2. The user opens `礼物提醒`, enables local alert channels and saves a high-value threshold. `测试提醒` exercises the same local popup/sound path without creating a real gift event. During the stream, statistics and the latest 20 gifts update from accepted Rust events; selecting a row opens its approved detail.
3. The user opens `礼物音效`, searches or filters the gift list, replaces a row's file through a system dialog, previews it, adjusts row/global volume and saves. Removal and reset require confirmation; import/export use system dialogs and never browser blobs.
4. The user opens `语音播报`, chooses engine/voice/speed/volume, event and identity filters, edits approved template variables and saves. The live queue shows one current item and a count; pause and clear affect Rust queue ownership, not incoming event capture.
5. With a connected/live room and valid authorized recording directory, the user opens `直播录制` and starts recording. The header becomes a stable recording status band. F9 or `手动标记` records a highlight. Stopping completes files and refreshes both lists.
6. The user previews, renames or deletes a recording, and previews, exports or deletes a highlight. System-dialog cancellation is a no-op; destructive cancellation restores focus and changes nothing.

Primary actions: save each configuration and start/stop recording. Secondary actions: preview/test, search/filter, adjust controls, mark highlight, import/export and open settings. Destructive actions: reset defaults, remove sound binding, delete recording and delete highlight; each uses only the confirmation explicitly required by the PRD. Non-goals: pause/resume recording, send a thank-you message, expose arbitrary path editing or add new event types.

## Information Architecture And Content Strategy

- `上舰特效`: page title/actions -> template and visual placement -> sound bindings/volume -> combo behavior -> save/reset. Preview stays adjacent to template selection; the actual effect renders in the dedicated window or source-backed fallback overlay.
- `礼物提醒`: configuration band first, then current-session totals and latest gift list. The page supports threshold decisions before monitoring data, without mixing per-gift audio bindings.
- `礼物音效`: search/filter and batch commands -> dense binding table -> global volume/queue capacity -> save/reset. Per-gift edits remain row-scoped.
- `语音播报`: engine/voice controls -> content and identity filters -> templates -> save/reset -> live queue. Configuration and queue are separate full-width bands so live controls stay visible without interrupting form scanning.
- `直播录制`: control/status band -> recordings and highlights as two adjacent tabs/sections -> settings dialog. Current recording risk (disk/source/status) remains above historical file management.
- The effect renderer is a separate restricted window with no form/navigation. The main-window overlay is a rendering fallback, not a second configuration owner.
- TTS and recording configuration each have one page owner. The existing duplicate settings-tab forms are removed from the owned chain.

## Layout Structure

- Pages live in the existing shell content region and use unframed full-width bands with borders/dividers instead of nested cards.
- Each page starts with a compact toolbar: title left, preview/test or batch commands next, primary save/control right. At 1024px controls may wrap to a second row without changing content width.
- Form pages use a two-column grid only at wide width. Dependent fields stay in the same group and collapse to one column at minimum width.
- Gift sound, recording and highlight tables use stable headers, horizontal overflow and a bounded action column. Loading/empty/error states reuse the table region dimensions.
- Recording status is a single stable band under the page toolbar with status, duration, size and disk figures in tabular numerals; warning text never overlays controls.
- Dialogs are viewport-bounded with internal scrolling and fixed action footers. File paths are readonly/truncated display values next to a system-dialog action, never editable authority.
- The effect window is full-bleed transparent content. It has no card, toolbar or decorative background surface and accepts no input.

## Visual Hierarchy

- Page titles remain compact operational headings. Save/start/stop are the strongest commands; preview/test/import/export are secondary; reset/remove/delete are visibly destructive only at their action point.
- Active recording, disk waterline and unavailable Sidecar/TTS states use text plus icon and color. Status is never color-only.
- Current TTS item and recording state are stronger than queued counts and historical metadata.
- Missing sound files are red text/icon with a rebind action; disabled bindings keep readable labels and reduced emphasis.
- Current-session gift totals use tabular numbers but remain subordinate to the configuration task.
- Effect animation is expressive only inside the dedicated effect surface; configuration pages remain quiet and work-focused.

## Visual Design Direction

- Tone: restrained broadcast control surface with neutral white/gray bands, thin borders and compact 12-20px spacing rhythm.
- Brand pink is limited to navigation/primary accent. Success green, warning amber, destructive/error red and neutral blue-gray keep state meanings distinct.
- Cards use at most 8px radius only for genuinely framed dialogs or a single status tool; page sections are not floating cards and cards are never nested.
- Existing Element Plus controls provide form/table/dialog semantics. Existing icon library supplies play, stop, save, folder, upload/download, volume, alert and delete icons; unfamiliar icon-only actions require tooltips.
- Configuration transitions are short and functional. Only source-required effect fade/animation is prominent; reduced-motion preference replaces nonessential motion with a static effect frame plus timed visibility.
- Loading uses stable skeleton rows or control-level spinners without resizing toolbars or status bands.

## Section Breakdown

### Captain Effect Configuration

- Template selector consumes a Rust-owned preset catalog with at least ten entries; page design does not invent unspecified preset labels. The PRD example `金色尊享` is permitted when present.
- Duration, position, colors and combo fields use selects, color swatches/pickers and toggles appropriate to their semantics.
- First-open, renewal and combo sound rows each display the selected filename, choose action and preview action. Missing files do not block visual preview.
- Save is pessimistic. Reset opens the source-required confirmation and restores persisted defaults only after Rust acknowledgement.

### Effect Window

- Render one queued event at a time with username, captain level and first/renew meaning. A default event remains visible for five seconds then fades.
- Window geometry follows logical coordinates. Display/DPI changes reposition without replaying or overlapping events.
- Creation failure projects the same event into one main-window overlay owned by the media runtime; it does not create a second queue.

### Gift Reminder

- Configuration contains master enable, threshold, three alert-method checkboxes, combo merge/window, popup position/duration and save/test.
- Totals show total value, donor count and gift count. The latest list is capped at 20 and uses one compact row grid.
- A gift row opens a focused detail drawer/dialog containing only user, gift, quantity, value and time.

### Gift Sound

- Table columns are gift, sound file, volume, enabled and actions. The page does not preserve legacy ID, sound-name, duration, play-mode, cooldown or priority columns.
- Replace/import/export paths always come from system dialogs. Accepted files are MP3/WAV; missing files keep the row visible and actionable.
- Global volume and the informational `10 个/秒` capacity sit in a footer band. Queue overflow is reported as a degraded/logged condition without exposing an invented manual queue editor.

### TTS

- Engine and voice are grouped; speed/volume use sliders. Event/identity choices are checkbox groups.
- Three template fields correspond to danmaku, gift and captain. Rust validates the fixed variable whitelist.
- Queue band shows current item, pending count, pause/resume state and clear. `暂停` changes to the corresponding continue state as a safe consequence of the source-backed pause operation.
- System/cloud fallback is one status line and one-time message, not repeated toasts per item.

### Recording

- Start is available only when the room, path, FFmpeg and disk preconditions are satisfied; the control exposes a source-backed disabled/failure reason.
- While recording, start becomes stop and highlight remains available. No pause/resume control is included.
- Recordings and highlights use separate tabs sharing the available table region. Recording row actions are preview/rename/delete; highlight actions are preview/export/delete.
- Settings dialog groups encoding/output, segmentation/automatic behavior, highlight, disk protection, FFmpeg source and F9 shortcut. Defaults shown are only the PRD defaults.

## Styling Direction

- Use Tailwind CSS-style utilities for all authored layout, spacing, colors, typography, borders and responsive behavior.
- Prefer dividers and stable table/form bands over shadow-heavy containers.
- Use fixed grid tracks, min widths and tabular figures for controls/status so changing labels, queue counts and durations do not shift layout.

## Tailwind CSS-Style Styling Constraints

- No new scoped CSS, CSS modules, Sass/Less, inline style objects or non-utility semantic class names.
- Conditional class bindings express only small states such as recording/warning/error, selected, unavailable or missing-file.
- Canvas/WebGL/animation drawing configuration is a renderer contract, not permission to set authored DOM styles imperatively.
- Existing non-utility media styling touched by implementation must be migrated or deleted within the owned closure.

## Class Length And Structure Strategy

- Keep each utility class value within normal formatter width. Split only approved named business regions when their markup/lifecycle already justifies a component.
- Reduce ornamental utilities before introducing another wrapper.
- Never hide long utility strings in constants, maps, computed values, helpers or imports.
- Put stable size constraints on the semantic root of effect, queue, recording status and table regions so pending/error content cannot resize them.

## Interaction Skeleton

- Keyboard order follows toolbar, configuration groups, table/list, footer. Enter submits a valid focused dialog; Escape cancels and restores focus to its trigger.
- Preview/test actions disable only while their own playback/request is pending and reject duplicate starts. A new preview stops/replaces the prior preview only if the Rust queue contract approves that behavior in spec.
- Persisted changes are pessimistic: controls may hold a page-local draft, but persisted tables/status update only after Rust confirms success.
- Reset, remove and delete show the PRD-required confirmation. Cancel is side-effect free; success restores focus to the originating row/action where it still exists.
- Search/filter operate on current rows without stealing focus. Live gift/TTS/recording updates preserve selection and scroll position.
- File selection, directory selection, import and export always invoke system dialogs. Dialog cancel is not an error and creates no grant/write.
- F9 marks a highlight only during recording. Shortcut conflict routes focus to the source-approved custom shortcut setting.
- Recording start/stop rejects duplicate submission. Stop and waterline-triggered stop stay pending until the Sidecar cleanup/finalization result arrives.

## State And Feedback Model

| State | Presentation and behavior | Source |
| --- | --- | --- |
| First/section load | stable skeleton/table region; dependent controls disabled; no invented explanatory copy | source-derived presentation of PRD async load paths |
| Empty gift bindings/recent gifts/files/highlights/TTS queue | empty region retaining its toolbar; exact guidance uses the approved concise Chinese system-copy stance and adds no new action | source-backed collections; confirmed 2026-09-14 system-copy stance |
| Filtered empty gift bindings | distinguish from the unfiltered total and preserve search/filter controls | source-backed search/filter; source-derived presentation |
| Effect preview pending/active/failure | preview control pending; render once; missing sound keeps visuals; window failure uses main overlay | PRD 9.6.5-9.6.6 |
| Gift notification permission denied | system notification is unavailable and test/live alerts fall back to local popup | PRD 9.7.6 |
| Missing/unsupported sound | MP3/WAV validation; missing row red with rebind action; visual/other alert paths continue | PRD 9.6.6, 9.8.6 |
| Audio/TTS queue overflow | Rust drops approved low-priority/oldest low-priority item and records the degraded state; UI remains responsive | PRD 9.8.6, 9.12.6 |
| TTS unavailable/degraded | system -> cloud, then cloud -> local with one prompt; unavailable cloud contract cannot show success | PRD 9.12.6; external-interface pending |
| Invalid TTS template | field-associated validation; save reports invalid variable and effective template becomes the PRD default | PRD 9.12.5-9.12.6 |
| Configuration dirty/pending/success/failure | page-local draft; save/reset disabled while pending; authoritative values refresh on success; failure retains draft | source-backed save/reset outcomes; source-derived state ownership |
| Recording idle/starting/recording/stopping/finalizing/error | one fixed status band and one valid primary action; duration/size/disk update from Rust events | PRD 9.13.2, 9.13.5-9.13.8 |
| FFmpeg unavailable/start failure | show path/permission failure and local-FFmpeg alternative; no fake recording state | PRD 9.13.3, 9.13.8 |
| Low disk/waterline | automatically stop, warn and preserve finalized results when available | PRD 9.13.4, 9.13.8 |
| Stream failure | up to three Rust retries; only final failure becomes user-facing | PRD 9.13.8 |
| Path/dialog cancellation | no mutation, no error feedback and any temporary grant is revoked | PRD system-dialog flows; source-derived safe cancellation |

## UX Writing And Feedback Copy Stance

- Preserve source labels and exact numeric/default language: `上舰特效`, `礼物提醒`, `礼物音效`, `语音播报`, `直播录制`, `F9`, `10 个/秒`, MP3/WAV, 10GB and 5GB.
- Use the previously confirmed concise Chinese system-copy authority only for this request's necessary empty, validation, confirmation, success and actionable technical-error feedback. It must not add help, marketing, upgrade or new workflow content.
- File/permission/FFmpeg errors state the failed precondition and the source-backed next action. Queue overflow and sound playback failure avoid repetitive toast storms and remain logged.
- Destructive confirmation names the selected binding/file/highlight and states the PRD-required reset/remove/delete consequence; no unrelated warning text is added.
- Cloud TTS and FFmpeg artifacts are described as unavailable/configuration conditions, never as installed or connected when missing.

## Responsive Behavior

- Primary target is 1280x800; minimum is 1024x680.
- At minimum width, page toolbars wrap into two rows with the primary action visible. Form grids collapse from two columns to one without changing field order.
- Tables retain identity/file/status/action columns and use horizontal scrolling for secondary metadata. Fixed action columns are allowed only when they do not cover content.
- Dialog width is bounded by viewport and content scrolls internally; action footers remain visible.
- Long filenames/paths truncate visually with keyboard-accessible tooltip/full-value exposure; they never expand the page horizontally.
- Effect content scales within logical window bounds and remains correctly positioned under DPI/display changes.
- Recording status metrics wrap as complete label/value pairs, never separate labels from values or overlap global status UI.

## Accessibility And Keyboard Considerations

- Use Element Plus semantic buttons, switches, checkboxes, selects, sliders, dialogs, tabs and tables with visible labels/focus states.
- Icon-only preview, stop, replace, export and delete actions have accessible names and tooltips; target size is at least 36px on desktop.
- Status, missing-file, recording and warning meanings use text/icon plus color.
- Dialog focus moves to the first meaningful control and returns to the trigger after cancel/success. Confirmation and file dialogs do not create keyboard traps.
- Live list/queue/status updates do not steal focus or announce every event; only material state changes use a polite announcement boundary.
- Sliders are keyboard-adjustable and expose numeric values. Color pickers include textual hex values already authorized by the PRD form.
- Reduced motion disables decorative effect motion while preserving the five-second visible event meaning and text.

## Usability Heuristics And Risk Controls

- Configuration overload: group by visual, audio, event selection and storage meaning; hide only source-backed dependent controls, not critical status.
- Duplicate settings ownership: each media capability has one page/form owner; global settings cannot silently diverge.
- Accidental destructive action: use the explicit confirmation and keep destructive controls separated from preview/save.
- File trust: readonly display plus system-dialog grants prevents users mistaking typed text for authorized access.
- Audio contention: one Rust-owned priority/queue model avoids overlapping renderer players and keeps preview state explicit.
- High-frequency gifts: latest list is bounded at 20; statistics are Rust-owned; sound queue capacity/overflow policy prevents reactive fan-out.
- Recording loss: start preconditions, waterline stop, stop/finalize pending state and Sidecar cleanup prevent false completion.
- F9 conflict: explain conflict once and expose the PRD-approved custom shortcut control.
- Long filenames and narrow viewports: truncation, tooltip and table overflow preserve action access.

## Component Reuse And Split Strategy

| Candidate | Decision and trigger | Real callers | State and contract | Styling/test boundary |
| --- | --- | --- | --- | --- |
| `CaptainEffectWindow` | extract as restricted window entry; project-convention and interaction/render lifecycle boundary | 1 Tauri effect window plus source-backed main-overlay renderer reuse | normalized queued effect props/events; owns render/animation cleanup, no request or queue authority | DPI/reduced-motion/cleanup/render tests |
| `MediaFileField` | do not create shared SFC; captain/gift/recording paths have different purposes, validation and actions | 2+ different semantics | each page receives opaque grants through its store/gateway | reuse transport mechanism, keep UI policy local |
| `GiftDetailDrawer` | extract; single-caller cohesive detail/focus lifecycle | 1 (`GiftReminderPage`) | normalized user/gift/count/value/time props; close only | focus/content/long-text tests |
| `GiftSoundTable` | extract; single-caller cohesive row-action and focused missing-file/filter test surface | 1 (`GiftSoundPage`) | normalized rows; emits preview/replace/remove/toggle/volume intentions; page/store owns commands | table overflow, row pending and missing-file tests |
| `TtsQueuePanel` | extract; single-caller interaction lifecycle and focused live-state surface | 1 (`TtsPage`) | current item/count/paused/pending props; emits pause/resume/clear | status, keyboard and update-stability tests |
| `RecordingStatusBand` | reuse/adapt existing status component only after removing pause/resume; same status semantics across page and shell | 2+ page/shell | normalized Rust status; page variant may emit start/stop/mark, shell variant display/control contract finalized later | duration/size/waterline/status tests |
| `RecordingList` | retain and refactor; single-caller cohesive business table/focused action lifecycle | 1 (`RecordingPage`) | normalized recordings; emits preview/rename/delete, no store import | table/focus/pending tests |
| `HighlightList` | retain and refactor; single-caller cohesive business table/focused export lifecycle | 1 (`RecordingPage`) | normalized highlights; emits preview/export/delete, no store import | table/grant/focus tests |
| `RecordingSettingsDialog` | extract; single-caller form validation/dirty/focus lifecycle | 1 (`RecordingPage`) | local form snapshot; emits save/cancel/select-path/set-source; adapter/store outside | validation, dirty, pending and focus tests |
| Captain/gift reminder/TTS configuration forms | keep inline in owning pages | 1 each | page-local form draft; page/store orchestrates save/test | extracting would mirror form models and create pass-through emits |
| Volume sliders and page toolbars | reuse Element Plus directly; no wrapper | 3+ different business owners | primitive values and local labels | a wrapper would hide local semantics without lifecycle |

Parent pages own route orchestration, request calls, cross-section state and feedback. Stores own authoritative snapshots/request state. Adapters normalize Rust DTOs before components. Presentational SFCs do not import stores, desktop transport or raw DTOs. No slots are needed for the approved extracted components.

## Design Risks

- Existing media owners are Electron/browser-first and duplicate queue, file and process authority in Vue. Implementation must remove the full caller/listener/timer cleanup closure rather than layer a second path over it.
- The current effect preview exposes unapproved authoring/render/test controls. Preserving them would expand product scope and complicate the restricted effect window.
- A runtime preset catalog must provide at least ten templates, but the PRD specifies only one example label and no artwork catalog. The design intentionally does not invent additional marketing names; spec must define stable technical preset identifiers and treat final display assets as source/config data.
- Cloud TTS endpoints/credentials and the LGPL FFmpeg shared binary are absent. Their ports and unavailable/degraded states can be implemented, but production integration cannot be claimed.
- Long-running recording, global F9, sound/TTS queues and the effect window all require explicit teardown owners; leaked listeners/processes would be release blockers.
- The persistent recording status is shared with the later shell module; this module may stabilize its data/actions but must leave final shell placement to `application-shell`.

## Open UI Questions

- No blocking layout or interaction question remains. The PRD fully defines page grouping, controls, main states and recovery routes.
- `external-content-pending`: nine or more additional captain preset display names/art assets are not supplied. The UI consumes a Rust/config-provided catalog and does not hard-code invented product copy; release acceptance still requires at least ten installed presets.
- `external-interface-pending`: cloud TTS provider/credentials and the LGPL FFmpeg shared artifact/source bundle are absent. Corresponding choices remain unavailable/degraded until deployment materials exist.
- `technical-contract-pending`: gift-sound batch schema, preset identifiers and media command/event DTOs belong to spec/architecture design. They may normalize transport but may not add visible fields or behavior.
