# Page Design: AI Voice Tone Selection

## Delivery Unit Identifier

`ai-voice-tone-selection`

## Page Objective

Extend the existing AI audio-route configuration so users can choose the audible AI reply voice before testing and confirming the route.

## Target Users or Scenarios

Livestream operators configuring local AI reply playback for capture by OBS or Bilibili Live Studio.

## Source and Upstream Constraints

- Source `SRC-1`: add voice-tone selection and default it to `温柔桃子`.
- The existing `AI 音频路由` dialog remains the owner of output device, volume, test playback, and route confirmation.
- ASR remains recognition-only; the audible choice is placed in the adjacent TTS route configuration.

## Product-Content Allowlist Mapping

| Proposed content | Source |
| --- | --- |
| `声色` select | `SRC-1` voice-tone selection |
| `温柔桃子` initial selection | `SRC-1` named default |
| Installed system voice options | Required implementation of `SRC-1` selection without inventing branded tone names |
| Test/save using the selected tone | Source-derived functional consequence of selection |
| Existing device, volume, confirmation, cancel, and enable behavior | Preserved code fact; no new product behavior |

## User Journey and Task Success

The user opens `音频路由`, chooses an output device and voice tone, adjusts volume, runs test playback, confirms external capture, and saves. Success is the existing confirmed route snapshot returning with the selected tone retained. Cancellation remains non-destructive. There are no destructive actions.

## Information Architecture and Content Strategy

Keep all playback choices in one dialog. Reading and action order is output device, voice tone, volume, test playback, capture confirmation, then save. The voice choice is visible before testing so the test represents what queued replies will use.

## Layout Structure

Add one full-width `el-form-item` between output device and volume. Reuse the current modal width and footer. No nested cards, new page region, or standalone explanatory copy is added.

## Visual Hierarchy and Direction

Preserve the compact operational form. The existing primary save button remains primary, test playback remains secondary, and the new selector uses the same full-width Element Plus form treatment as the device selector.

## Section Breakdown

- Route destination: existing output-device select.
- Voice identity: new `声色` select.
- Playback intensity: existing volume slider.
- Validation: existing test and confirmation controls.

## Styling Direction

Reuse existing Element Plus controls and current spacing. Any authored styling remains Tailwind utility classes; no CSS block or semantic style class is added.

## Tailwind CSS-Style Styling Constraints

The selector uses the existing short `w-full` utility. No computed/class constant is introduced. Existing class strings remain unchanged.

## Interaction Skeleton

- Opening the dialog hydrates device, voice, volume, and confirmation from the snapshot.
- Selecting a voice updates only dialog-local draft state.
- Test playback emits the selected device, voice, and volume.
- Confirm emits the same values and preserves the existing enable-after-save behavior.
- Save stays disabled until device, voice, and capture confirmation are present.

## State and Feedback Model

- Loading: existing pending flags remain owners of test and save state.
- Empty voice catalog: the voice selector and save action are disabled; no new copy is introduced because none was source-authorized.
- Error: existing store/page error region remains owner.
- Success: existing `音频路由已确认` feedback remains unchanged.
- Dirty state: not separately exposed; the dialog keeps its current save semantics.

## UX Writing and Feedback Copy Stance

Only the source-authorized label `声色` and default display name `温柔桃子` are added. Existing feedback copy remains unchanged.

## Responsive Behavior

The full-width select fits the current `min(560px, 94vw)` modal and stacks with existing fields on narrow viewports.

## Accessibility and Keyboard Considerations

Use native Element Plus form labeling and select keyboard behavior. The new field follows DOM/focus order between device and volume. Disabled state is conveyed by the control, not color alone.

## Usability Heuristics and Risk Controls

- Prevent testing or saving without a selected voice.
- Keep voice choice adjacent to other playback settings to avoid hidden dependency.
- Do not overload the AI page header with another top-level control.

## Component Reuse and Split Strategy

- Reuse the existing `el-form-item`, `el-select`, and `el-option` pattern.
- Keep the new field inline in `AiVoiceReplyControl.vue`; it has one caller, introduces no independent lifecycle, and extraction would be pass-through plumbing.
- `AiVoiceReplyControl` remains the existing cohesive single-caller interaction boundary; no new SFC is introduced.

## Design Risks

- System voice availability differs by host. The default preset must stay selectable and use a deterministic fallback when its preferred system voice is absent.
- Legacy rows have no voice field and require migration/default handling.

## Open UI Questions

None blocking. Additional branded voice names are intentionally not invented; host-provided voices retain their system display names.

