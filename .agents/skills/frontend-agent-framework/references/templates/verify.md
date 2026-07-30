# Verification Templates

## Required Artifact Paths

- `docs/requests/<request-id>/verification/verification.md` and `verification/evidence/` for non-split work
- `docs/requests/<request-id>/module-runs/<module-id>/verification/verification.md` and `verification/evidence/` for split PRD-driven work

## `verification/verification.md` or `module-runs/<module-id>/verification/verification.md`

Required top-level sections:

- delivery unit identifier
- workflow efficiency evidence
- self-healing loop compliance
- acceptance coverage
- user intent compliance
- change-chain integrity
- removal cleanup compliance
- spec constraint compliance
- source grounding compliance
- design-pattern compliance
- expert frontend engineering compliance
- field display semantics compliance
- frontend architecture quality compliance
- architecture reuse compliance
- production code quality compliance
- human review readiness compliance
- functional-programming compliance
- frontend styling compliance
- API contract conformance
- TypeScript context compliance
- summary

Required fields for each acceptance item:

- acceptance item id
- verification method
- result
- evidence reference
- follow-up if failed
- handoff status

`workflow efficiency evidence` must include:

- `speed_profile.level`
- verification scope used
- scoped commands run
- broader commands skipped with reason and substitute evidence
- any profile upgrade found necessary during verification

`self-healing loop compliance` must include:

- result (`pass` or `fail`)
- `state.json.loop.iteration`
- active delivery unit self-healing iteration count and whether it is within the limit `<= 7`
- defect inventory grouped by: requirement-mismatch, plan-gap, architecture-gap, logic-error, syntax-type-error, runtime-error, code-organization-error, maintainability-error, technical-debt-blocker
- blocker identity and attempt count for each remaining blocker
- automatic re-entry target for every failed item: `execute`, `plan`, `spec`, `architecture-design`, front-loaded confirmation gate, or `none`
- issues fixed in the current pass
- evidence rechecked after the fix
- progress check: new evidence, new repair, narrower diagnosis, or escalation; if none, result must be `fail`
- loop-bound check: same blocker same-stage attempts `<= 2`, same blocker total attempts `<= 3`
- remaining blockers, or `none`
- completion allowed: `yes` only when no blocker remains and all applicable verification verdicts pass

`user intent compliance` is required when the spec contains a user intent contract. It must include:

- result (`pass` or `fail`)
- literal compliance result
- practical-goal compliance result
- forbidden-interpretation checks
- evidence that complexity, risk, ambiguity, or responsibility was not merely relocated
- follow-up if failed

`change-chain integrity` is required when implementation modifies or removes existing code. It must include:

- result (`pass` or `fail`)
- changed feature chain
- pre-change chain review evidence
- post-change chain validation evidence
- checked entrypoints, files, imports, exports, callers, consumers, side effects, state, tests, and mocks
- test references to changed or removed production code treated as adaptation targets, not production owners
- missing required links found or ruled out
- stale extra links found or ruled out
- neighboring feature impact found or ruled out
- follow-up if failed

`removal cleanup compliance` is required when implementation removes a call, request, branch, field, control, or side effect. It must include:

- result (`pass` or `fail`)
- removed behavior
- checked imports, helpers, constants, types, request wrappers, state, tests, mocks, and comments
- retained helpers and their remaining real production callers
- test-only references found and adapted, not used as retention evidence
- evidence reference
- follow-up if failed

`spec constraint compliance` must include:

- result (`pass` or `fail`)
- checked constraints
- locality and file-extraction checks for new helper / hook / mapper / utility files, including real production caller count and approved boundary reason for any single-caller separate file
- Vue component extraction checks when applicable: approved candidate decision, extraction trigger or non-extraction blocker, real production caller count (`1`, `2+ same semantics`, or `2+ different semantics`), single-caller exception if any, props / emits / slots contract, state owner, data normalization boundary, Tailwind class impact, and test surface
- evidence reference
- follow-up if failed

`source grounding compliance` must include:

- result (`pass` or `fail`)
- checked source-grounding labels from spec and plan
- reverse diff inventory of every added or changed field, copy item, control, interaction, branch, state, default, validation, permission, navigation target, API / payload semantic, status mapping, analytics event, and acceptance behavior
- per-item mapping to the plan product-content allowlist and exact original requirement or explicit confirmation record
- technical-only hunks marked `product-content delta: none`, with evidence that code facts only constrained behavior-preserving implementation
- confirmation that downstream artifacts did not retroactively manufacture authority for code already written
- ungrounded behavior, neighboring-module expansion, sample-content expansion, or convention-based expansion found or ruled out
- evidence reference
- follow-up if failed

The result must be `fail` if any diff item is missing from the reverse inventory, lacks original-source or confirmation authority, relies only on code facts / tests / conventions for product semantics, or was justified through post-hoc artifact edits.

`design-pattern compliance` is required for every delivery unit. It must include:

- result (`pass` or `fail`)
- pattern-fit decision from spec and plan: `direct code`, `reuse existing pattern`, `adapt lightweight pattern`, or `introduce pattern`
- decision depth verified: Level 0, Level 1, Level 2, or Level 3
- triggered candidate signals verified; if none were triggered, confirm the artifact did not mechanically enumerate unrelated patterns
- direct-code no-signal rationale checked when no pattern was selected
- triggered rejected alternatives checked
- implementation conformance to the approved decision
- frontend syntax shape conformance checked for the current stack; backend-style class hierarchy, abstract base, manager, factory, handler, or one-method wrapper ceremony found or ruled out
- overbuilt pattern ceremony, fake extensibility, unapproved pattern introduction, missed lightweight pattern, or downgraded approved pattern found or ruled out
- evidence reference
- follow-up if failed

`architecture reuse compliance` is required when the scoped work touches business rules, validation, filtering, sorting, option building, permission checks, payload construction, status mapping, adapter / mapper normalization, view-model construction, helper logic, or state derivation. It must include:

- result (`pass` or `fail`)
- reuse candidates checked
- Anti-DRY matrix verified: semantic owner, layer, production use-site count, change stability, and variation axes
- extraction / reuse / keep-separate / defer decisions verified
- shared owner, dependency direction, public API, and side-effect boundary checks
- single-use locality decisions checked; one-caller helper files found or ruled out
- commonality classification and target owner checks
- dependency injection / strategy / adapter boundary checks for business-specific variation
- migrated production callers, tests, mocks, imports, and exports
- JSDoc `@see` / `@example` traceability for promoted shared APIs
- duplicate legacy paths found or ruled out
- premature abstraction, God utils, business-entity imports in shared utilities, environment side effects, and merged mega-interfaces found or ruled out
- behavior-equivalence evidence
- follow-up if failed

`expert frontend engineering compliance` is required when the scoped work adds or changes user-facing frontend behavior, state flow, data flow, component composition, frontend architecture, or production integration. It must include:

- result (`pass` or `fail`)
- user journey integrity checks
- page-design UI / UX contract checks when a page-design artifact exists: information architecture, visual hierarchy, feedback states, responsive behavior, accessibility inputs, and UX risk controls
- field-level business display semantics checked or delegated to `field display semantics compliance`
- state ownership and data lifecycle checks
- async correctness checks for loading, dedupe, cancellation, stale responses, race semantics, idempotency, and retry when applicable
- interaction resilience checks for keyboard, focus, semantic controls, disabled states, destructive confirmations, permissions, and feedback when applicable
- performance and rendering boundary checks
- evolution safety checks for migration, compatibility, rollback, cleanup trigger, or dual-path behavior when applicable
- testability and diagnostic evidence
- follow-up if failed

`field display semantics compliance` is required when scoped work touches displayed fields, table columns, detail fields, cards, links, tags, status labels, jump targets, date/time fields, or highlighted fields. It must include:

- result (`pass` or `fail`)
- checked field / column list
- approved spec / plan field-display contract reference
- business-meaning evidence checked for each field: source wording, label, surrounding workflow, API / DTO meaning, code facts, and acceptance intent
- semantic display role check when useful: project/domain meaning first, with optional diagnostic tags only after the business meaning is proven
- exact format and precision evidence for every field whose business meaning requires precision
- empty / invalid / unknown value display evidence
- formatter / helper / component conformance; generic formatter or component-default substitution found or ruled out
- clickable / navigational behavior evidence when applicable
- default, hover, focus, active, visited, disabled, and permission-denied visual-state evidence when applicable
- business emphasis vs interaction-only affordance check when source requires highlight / emphasis / callout
- convention-substitution check for technically similar fields, such as temporal values, amounts, statuses, links, tags, ids, or booleans that may have different business meanings
- follow-up if failed

`frontend architecture quality compliance` is required when an architecture-design artifact exists, or when scoped work changes module boundaries, ownership, state/data topology, dependency direction, extension seams, migration paths, public APIs, or frontend architecture. It must include:

- result (`pass` or `fail`)
- architecture decision ids checked
- business capability owner and module boundary checks
- ownership map checks for page orchestration, business rules, form model, view model, request transport, adapter normalization, server/client/route state, permissions, and side effects
- state topology checks for writable state, derived state, async state, cleanup, cancellation, stale response handling, and synchronization
- data lifecycle checks from DTO -> adapter / mapper -> view model / domain model -> form model / props -> payload
- dependency rule matrix checks for allowed imports, forbidden imports, public APIs, private symbols, and cycles
- extension seams and rejected future-proofing checked against real variation axes
- complexity budget checks for file count, public APIs, indirection depth, reactive fan-out, and migration scope
- migration, compatibility, fallback, feature flag, or dual-path owner and cleanup trigger checks
- architecture boundary verification evidence beyond a manual happy path
- follow-up if failed

`functional-programming compliance` is required when the scoped work includes business rules, validation, data transformations, payload construction, state derivation, adapter / mapper normalization, or side-effect orchestration. It must include:

- result (`pass` or `fail`)
- pure rules or transformations checked
- immutable input handling checked
- side-effect boundary checks
- adapter / mapper / `fromDetail` semantic-normalization checks
- hidden mutation or hidden side effects found or ruled out
- readability of functional composition
- evidence reference
- follow-up if failed

`production code quality compliance` is required when production code was added or changed. It must include:

- result (`pass` or `fail`)
- type-first contract checks, including precise type / interface ownership and branded type or equivalent nominal distinctions when needed
- fail-fast checks for abnormal, empty, nullable, invalid, timeout, rejected, partial-success, and impossible states
- strict null checks for `null` / `undefined` distinction and proper use of `??` / `?.`
- naming checks for booleans, callback props, internal event handlers, and function verbs
- no magic variable checks, including helper dependency inputs and configuration constant ownership
- maintainability-first performance checks, including justification for memoization, caching, debounce, throttle, virtualization, `useMemo` / `useCallback`, computed caching, or watcher optimization
- pure function / immutable data checks, including any approved exception for classes or mutable owners
- boundary UI state checks for list empty states, async loading states, and form input error states
- evidence reference
- follow-up if failed

`human review readiness compliance` is required when production code, tests, mocks, contracts, or generated-facing files were changed. It must include:

- result (`pass` or `fail`)
- changed files checked against approved scope
- every changed hunk mapped to an approved task, cleanup obligation, or test adaptation
- local convention conformance checked
- unrelated changes, formatting churn, debug code, dead code, commented-out code, stale comments, temporary names, unused imports / exports, stale tests, and stale mocks found or ruled out
- reviewer-ready evidence checked: tests, type checks, lint checks, manual checks, commands not run with reason, and accepted risks
- follow-up if failed

`frontend styling compliance` is required when the scoped work adds or changes authored styling. It must include:

- result (`pass` or `fail`)
- Tailwind CSS-style utility class conformance
- class length and inline reviewability check
- check that overlong class values were not hidden in constants, maps, computed properties, helpers, or imported variables
- check that source-approved business emphasis, link affordance, and interaction visual states were not replaced by component-default hover classes
- evidence reference
- follow-up if failed

`project scaffold compliance` is required when the scoped work creates, initializes, or starts a project, app, package, frontend surface, micro frontend, component library, SDK package, or buildable frontend entry. It must include:

- result (`pass` or `fail`)
- scaffold-sensitive source signal and project type checked
- scaffold candidate discovery evidence checked
- selected scaffold / starter, generator, template, sibling app baseline, or approved no-scaffold decision checked
- initialization command / copy strategy checked
- approved deviations checked against generated / initialized files
- rejected-candidate reasons checked
- hand-written bootstrap found or ruled out
- follow-up if failed

`API contract conformance` is required when the scoped work touches backend integration. It must include:

- result (`pass` or `fail`)
- authoritative contract source checked
- request and response shape conformance
- backend-owned TypeScript type reuse, when available
- non-TypeScript contract translation with backend field names preserved, when applicable
- adapter / mapper semantic-normalization boundary checks
- request-layer ownership for transport, errors, loading, and side effects
- evidence reference
- follow-up if failed

`TypeScript context compliance` is required when the scoped work is TypeScript-affecting. It must include:

- result (`pass` or `fail`)
- governing `tsconfig` checked
- materially relevant `extends` chain checked
- relevant compiler options checked
- relevant declarations or generated type sources checked
- confirmation that implementation did not rely on guessed aliases, globals, generated types, or module resolution
- evidence reference
- follow-up if failed
