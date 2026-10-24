# Normalized Defect Snapshot

## Verbatim Source Evidence Index

- `SRC-1`: “温柔桃子声色怎么与豆包的温柔桃子声音不一样，预期是与豆包的温柔桃子声色一致”

## Source Summary

The current audible `温柔桃子` result is not the Doubao voice it claims to represent. Exact voice identity, not merely a similar female system voice, is the required outcome.

## Key Business Goal

Voice-name fidelity: choosing `温柔桃子` must mean hearing Doubao's `温柔桃子`.

## Greenfield Scaffold Sensitivity

不适用：this is a repair inside an existing Tauri application.

## Explicit Behavior Constraints

- `温柔桃子` must not resolve to Microsoft HuiHui or an arbitrary system default.
- Test playback and queued playback must use the same exact voice route.
- No unrelated ASR, LLM, or general TTS behavior changes are authorized.

## Closed-World Product Scope Allowlist

- Correct the existing `温柔桃子` identity and playback behavior.
- Necessary technical configuration may be added only after its credential and speaker contract are confirmed.

## Forms, Tables, Displays, And Interactions

- Existing `声色` selector and test/confirm interactions remain the user-facing entrypoints.
- No new field or control is authorized yet because the credential source is unresolved.

## Workflow And State Rules

- Selection -> test/save -> application validation -> speech adapter -> audible playback.
- Missing credentials or unsupported speaker identity must fail explicitly; no silent system-voice substitution.

## Relevant Modules Or Pages

AI page voice-route dialog, typed desktop contracts/store, Rust member application state, and system speech adapter.

## Notable Open Questions

- Exact official speaker ID for the current Doubao-app `温柔桃子` voice.
- Approved source and storage/configuration UX for the required Doubao Voice API credential.

