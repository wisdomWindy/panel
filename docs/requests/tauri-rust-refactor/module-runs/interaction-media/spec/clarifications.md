# Interaction Media Clarifications

## Captain Preset Content

- question: What are the display names and visual assets for the required 10+ captain presets?
- answer: The PRD supplies the count and one wireframe example (`金色尊享`) but no complete catalog; no additional source was supplied.
- final decision: Implement a trusted Rust/config preset-catalog contract and count validation without inventing marketing names in the page/spec. The final asset/catalog package remains `external-content-pending`; release acceptance for 10+ finished presets requires that package.
- affected spec area: IM-01, effect configuration/render assets, IM-A01/IM-A02.
- grounding status and required source: `missing-source` only for final names/art; PRD 9.6 authorizes catalog mechanics and at least ten entries.

## Cloud TTS Contract

- question: Which cloud TTS provider, endpoint, credentials and voice catalog are production-authorized?
- answer: None is present in the PRD or repository.
- final decision: Keep a typed provider port and real unavailable/degraded state. Do not call an invented service or report cloud success. System/local paths may operate when actually available.
- affected spec area: IM-04, API ports, TTS fallback, IM-A05/IM-A14.
- grounding status and required source: `external-interface-pending`; provider API/credential/voice catalog is required for production cloud activation.

## Bundled FFmpeg Artifact

- question: Which LGPL shared FFmpeg binary, checksum, license text and source URL are approved for packaging?
- answer: The PRD recommends BtbN or gyan.dev but no artifact/checksum is in the repository.
- final decision: Implement Sidecar ports, resource discovery, compliance metadata contract, unavailable state and user-selected local FFmpeg. Do not download or package an unapproved binary in this module.
- affected spec area: IM-05, build resources, About handoff, IM-A06/IM-A14.
- grounding status and required source: `external-artifact-pending`; deployment owner must provide the approved bundle metadata/artifact.

## Technical Contract Normalization

- question: What transport formats should represent batch gift-sound configuration, duration inputs and secure media preview?
- answer: PRD defines the visible fields/outcomes and security boundary but not wire shapes.
- final decision: Architecture design may choose a versioned structured import document limited to approved fields, canonical integer duration units with `HH:mm:ss` UI, and an opaque Rust-owned preview session/protocol. These choices must add no fields or user behavior.
- affected spec area: IM-03/IM-05 API and adapter contracts.
- grounding status and required source: `code-fact-backed` technical normalization under PRD 9.8/9.13/10; no product-content delta.

## Danmaku Heat Threshold Window

- question: Over what interval is the recording page's `弹幕热度阈值` evaluated?
- answer: PRD 9.13 requires a danmaku-surge threshold but omits the interval; the existing product config/UI/tests consistently define the threshold as accepted danmaku count per rolling 10 seconds.
- final decision: Preserve the existing rolling 10-second product baseline and move the rule to Rust. Only accepted danmaku events count; changing the interval requires a future product decision.
- affected spec area: IM-05 automatic highlight detection and IM-A07.
- grounding status and required source: `source-backed` high-level behavior from PRD 9.13 plus `code-fact-backed` behavior-preserving interval from the existing product requested for refactor; no new control is added.

## Gift Identity Contract

- question: How can per-gift sound bindings remain stable when the completed live event DTO currently exposes only `giftName`?
- answer: PRD 9.7.2 requires parsing gift identity and PRD 9.8 repeatedly uses `gift_id`; SQLite already persists `gift_id`.
- final decision: Add required `giftId` to the shared accepted `GiftEvent`/Rust gift variant and key bindings by it. `giftName` stays a display label and is not an identity key.
- affected spec area: shared event DTO, IM-02/IM-03 and their adapters/tests.
- grounding status and required source: `source-backed`, PRD 9.7.2 and 9.8.2-9.8.5.

## Approval Status

- question: Are any product-visible or business-semantic decisions still being treated as assumptions?
- answer: No. Missing external content/interfaces remain unavailable conditions and are not written as approved behavior.
- final decision: Spec may pass automatic approval and route to architecture design; production release claims remain bounded by the pending external inputs above.
- affected spec area: source grounding, acceptance and handoff.
- grounding status and required source: `source-backed` behavior plus explicit `missing-source` external conditions.
