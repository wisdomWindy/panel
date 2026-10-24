# Review: AI Voice Tone Selection

## Delivery Unit Identifier

`ai-voice-tone-selection`

## Review Scope

Reviewed the complete AI voice-reply chain, migration, platform voice selection, frontend state/interaction, tests, specification, plan, and verification evidence at `S2 scoped` depth.

## Findings

### Blocking Issues

None remaining.

### Fixed During Review

- `logic-error`: stale or empty voice catalogs left a non-null draft ID, so test/save controls could appear enabled. Fixed by deriving `isDraftRouteAvailable` from both current catalogs and using it in handlers and disabled states. Attempt count: same-stage 1, total 1. Re-entry target after repair: none.

### Non-Blocking Issues

- Local Node is 22.21.0 while the package declares Node 24; all invoked checks and builds pass with an engine warning.
- Vite reports the repository's existing large-chunk warning; this change does not materially alter the bundle topology.

### Accepted Risks

- Physical Windows audio capture and subjective voice quality were not automated. SAPI token enumeration/selection compiles, while injected-port tests prove exact ID propagation; `gentle-peach` has a controlled current/default fallback.
- A concrete host voice may later be uninstalled. Catalog availability disables UI actions and backend validation returns `ai_tts_voice_invalid`; no silent substitution occurs.

### Follow-Up Items

None required for this request.

## Source And Intent Audit

- Added product content is limited to `声色`, `温柔桃子`, host-provided voice labels, and the interactions necessary for selection to affect playback.
- Fields, defaults, validation, payloads, state, and acceptance behavior trace to the direct user instruction and the approved source snapshot/allowlist.
- SQL, DTOs, mapping, port arguments, OS APIs, tests, and route availability checks are technical implementation with `product-content delta: none`.
- No ASR behavior, cloud provider, general TTS page behavior, permission, navigation, analytics, or invented branded voice was added.

## Chain And Correctness Review

- Verified Vue draft -> emits -> page -> Pinia -> desktop contract -> IPC DTO -> application validation -> repository/platform -> queue playback.
- Confirmed catalog/snapshot serialization, branded frontend ID, persisted non-null ID, exact system-token selection, default fallback, and invalid/stale selection rejection.
- Reviewed command registration/reference sweep and all test doubles. No stale three-argument speech call, duplicate path, orphan helper, or missing consumer remains.
- No behavior was removed, so removal dependency cleanup is not applicable.

## Code Quality Review

- Constants `DEFAULT_AI_VOICE_ID` and `DEFAULT_AI_VOICE_DISPLAY_NAME` carry stable cross-boundary domain meaning at the narrow application-contract owner.
- The existing `AiSpeechOutput` strategy is the only added variation seam; no factory, manager, registry, wrapper, utility file, or speculative abstraction exists.
- DTO normalization is deterministic and immutable; writable UI state is dialog-local; I/O side effects remain in store, IPC, repository, and platform owners.
- Guards cover empty, unknown, unavailable, invalid-volume, missing-token, and catalog-empty states. Control flow and names remain explicit.
- No high-frequency event, subscription, listener, timer, cache, memoization, or cleanup obligation was added.

## Architecture Review

- ADR-1 stable preset, ADR-2 speech-port extension, and ADR-3 snapshot-owned catalog match implementation.
- Dependency direction remains Vue -> contract/store -> IPC -> application port/state -> infrastructure; OS token types do not escape infrastructure.
- State topology has one persisted owner and one dialog draft owner; no duplicated server state or template-level DTO normalization exists.
- General TTS reuse was correctly rejected by the Anti-DRY matrix because domain, persistence, runtime, and use-site semantics differ.
- Migration is additive, defaulted, validated, covered by fresh/idempotent DB tests, and has a bounded rollback surface.

## Evidence Rechecked

- `cargo test --features free --lib`: 131 passed.
- `cargo check --features member`: passed.
- Focused Vitest: 6 passed, including the review regression assertion.
- `pnpm ... type-check`: passed.
- Scoped ESLint: passed.
- `cargo fmt --check`: passed.
- `git diff --check`: passed, with pre-existing line-ending warnings on the broad run.
- `build:web:free` and `build:web:member`: passed; member build rerun after the review fix.

## Assessments

- self-healing loop assessment: pass
- clean-code assessment: pass
- source grounding assessment: pass
- design-pattern assessment: pass
- user intent assessment: pass
- change-chain integrity assessment: pass
- removal cleanup assessment: pass (not applicable)
- expert frontend engineering assessment: pass
- field display semantics assessment: pass
- frontend architecture quality assessment: pass
- architecture reuse assessment: pass
- production code quality assessment: pass
- code review checklist assessment: pass
- human review readiness assessment: pass
- functional-programming assessment: pass
- frontend styling assessment: pass
- API contract assessment: pass
- TypeScript context assessment: pass

## Self-Healing Assessment

- Iteration: 7, within the maximum of 7.
- Defect classes remaining: requirement-mismatch none; plan-gap none; architecture-gap none; logic-error none; syntax-type-error none; runtime-error none; code-organization-error none; maintainability-error none; technical-debt-blocker none.
- Progress: the review produced a new boundary-state diagnosis, a focused repair, and passing regression evidence.
- Same-blocker limits: 1 same-stage attempt, 1 total attempt; both within bounds.
- Remaining blockers: none. Completion allowed: yes.

## Summary

The implementation satisfies the approved specification and practical user goal, stays within the planned architecture and content allowlist, and is ready to complete.
