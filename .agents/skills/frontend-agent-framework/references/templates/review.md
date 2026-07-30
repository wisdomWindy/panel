# Review Templates

## Required Artifact Path

- `docs/requests/<request-id>/review/review.md` for non-split work
- `docs/requests/<request-id>/module-runs/<module-id>/review/review.md` for split PRD-driven work

## `review/review.md` or `module-runs/<module-id>/review/review.md`

Required sections:

- delivery unit identifier
- workflow efficiency assessment
- self-healing loop assessment
- blocking issues
- non-blocking issues
- accepted risks
- follow-up items
- user intent assessment
- change-chain integrity assessment
- removal cleanup assessment
- clean-code assessment
- source grounding assessment
- expert frontend engineering assessment
- field display semantics assessment
- frontend architecture quality assessment
- architecture reuse assessment
- production code quality assessment
- code review checklist assessment
- human review readiness assessment
- functional-programming assessment
- frontend styling assessment
- API contract assessment
- TypeScript context assessment
- design-pattern assessment
- code-context structural assessment
- merge readiness summary

`workflow efficiency assessment` must include:

- `speed_profile.level`
- whether review used compact changed-hunk review or full review
- whether context breadth was sufficient for the impact found
- whether any broader impact should have upgraded the profile earlier
- confirmation that speed did not skip applicable quality verdicts

`self-healing loop assessment` must include:

- result (`pass` or `fail`)
- `state.json.loop.iteration`
- active delivery unit self-healing iteration count and whether it is within the limit `<= 7`
- defect inventory grouped by: requirement-mismatch, plan-gap, architecture-gap, logic-error, syntax-type-error, runtime-error, code-organization-error, maintainability-error, technical-debt-blocker
- blocker identity and attempt count for each remaining blocker
- independent review findings, not only verification summaries
- automatic re-entry target for every blocker: `execute`, `plan`, `spec`, `architecture-design`, front-loaded confirmation gate, or `none`
- issues fixed in the current pass and evidence rechecked after the fix
- progress check: new evidence, new repair, narrower diagnosis, or escalation; if none, result must be `fail`
- loop-bound check: same blocker same-stage attempts `<= 2`, same blocker total attempts `<= 3`
- remaining blockers, or `none`
- completion allowed: `yes` only when no blocker remains and every applicable verification / review verdict passes

`user intent assessment` is required when the spec contains a user intent contract. It must include:

- result (`pass` or `fail`)
- literal request compliance
- practical goal compliance
- forbidden interpretations found or ruled out
- required follow-up if failed

`change-chain integrity assessment` is required when implementation modifies or removes existing code. It must include:

- result (`pass` or `fail`)
- changed feature chain reviewed
- pre-change flow and reference analysis sufficiency
- post-change chain cleanliness
- missing links, stale extra links, duplicate paths, conflict paths, or orphan symbols found
- test references to changed or removed production code adapted rather than treated as production owners
- neighboring feature impact found or ruled out
- required follow-up if failed

`removal cleanup assessment` is required when implementation removes a call, request, branch, field, control, or side effect. It must include:

- result (`pass` or `fail`)
- orphan imports, helpers, constants, types, request wrappers, state, tests, mocks, or comments found
- retained helpers and their remaining real production callers
- test-only references found and adapted, not used as retention evidence
- required follow-up if failed

`clean-code assessment` must include:

- result (`pass` or `fail`)
- key findings
- findings for constants introduced or moved to a broader scope, including the domain meaning, constraint, snapshot, simplification, or real reuse that justifies each non-obvious declaration
- unnecessary one-off aliases, speculative module or exported constants, synonym constants, and over-broad constant scope found or ruled out
- findings for helper / hook / mapper / utility files introduced or moved out of their owner, including real production caller count and approved boundary reason
- findings for Vue SFC / page-local component extraction, including extraction trigger, non-extraction blocker, real production caller count (`1`, `2+ same semantics`, or `2+ different semantics`), single-caller exception if any, props / emits / slots contract, state owner, and whether the boundary improved or harmed readability
- single-page single-caller one-function files, local catch-all `utils.ts` / `helpers.ts`, or single-caller mapper files found or ruled out
- one-caller components without an approved exception, one-caller pass-through Vue components, generic wrappers, broad optional-prop components, DTO / form-model passthrough components, prop / emit chains longer than the original local markup, and components extracted only to shorten a file or hide long classes found or ruled out
- required follow-up if failed

`source grounding assessment` must include:

- result (`pass` or `fail`)
- independent reverse diff inventory for every added or changed field, copy item, control, interaction, branch, state, default, validation, permission, navigation target, API / payload semantic, status mapping, analytics event, and acceptance behavior
- whether each product-content item maps through the plan allowlist to exact original requirement wording or an explicit confirmation record
- whether technical-only hunks are truly `product-content delta: none` and code facts were used only as implementation constraints
- whether upstream artifacts were edited after implementation to rationalize unsupported code
- ungrounded behavior, neighboring-module expansion, sample-content expansion, convention-based expansion, or preference-based expansion found or ruled out
- whether any missing-source item was incorrectly implemented as confirmed behavior
- required follow-up if failed

Missing reverse-inventory items, broken original-source chains, code-fact-only product semantics, or post-hoc grounding are blocking findings and require `fail`.

`architecture reuse assessment` is required when the scoped work touches business rules, validation, filtering, sorting, option building, permission checks, payload construction, status mapping, adapter / mapper normalization, view-model construction, helper logic, or state derivation. It must include:

- result (`pass` or `fail`)
- missed extraction or reuse opportunities found or ruled out
- whether kept-separate duplicates have real domain, layer, lifecycle, permission, API, dependency, use-site-count, instability, variation-axis, or evolution reasons
- whether shared owner, dependency direction, public API, type ownership, and side-effect boundary are appropriate
- whether Anti-DRY reasoning prevented wrong abstraction instead of blindly enforcing DRY
- whether commonality classification and target owner are correct
- whether single-use locality decisions are correct, and whether page-private logic stayed colocated unless a real boundary reason justified extraction
- whether shared code uses dependency injection, strategy, getters, adapters, or thin wrappers instead of hard-coded business fields
- whether all in-scope production callers were migrated
- duplicate legacy paths, orphan helpers, dumping-ground utility, premature abstraction, feature-entity import, environment side effect, or merged-interface findings
- JSDoc `@see` / `@example` traceability findings for promoted shared APIs
- behavior-equivalence evidence reviewed
- required follow-up if failed

`expert frontend engineering assessment` is required when the scoped work adds or changes user-facing frontend behavior, state flow, data flow, component composition, frontend architecture, or production integration. It must include:

- result (`pass` or `fail`)
- end-to-end user journey findings
- page-design UI / UX contract findings when a page-design artifact exists: information architecture, visual hierarchy, feedback states, responsive behavior, accessibility inputs, and UX risk controls
- field-level business display semantics findings or reference to `field display semantics assessment`
- state ownership and data lifecycle findings
- async correctness findings for loading, dedupe, cancellation, stale response, race semantics, idempotency, and retry when applicable
- interaction resilience findings for keyboard, focus, semantic controls, disabled states, destructive confirmations, permissions, and feedback when applicable
- performance and rendering boundary findings
- evolution safety findings for migration, compatibility, rollback, cleanup trigger, or dual-path behavior when applicable
- testability and diagnostic evidence findings
- required follow-up if failed

`field display semantics assessment` is required when scoped work touches displayed fields, table columns, detail fields, cards, links, tags, status labels, jump targets, date/time fields, or highlighted fields. It must include:

- result (`pass` or `fail`)
- field / column list reviewed
- approved spec / plan display contract reference
- whether each field's business meaning was correctly inferred from source wording, label, surrounding workflow, API / DTO meaning, code facts, and acceptance intent
- whether technically similar fields were reviewed by business meaning rather than raw type, field name, neighboring code, or component defaults
- whether fields whose business meaning requires precision use the approved precision and formatter
- whether amount, status, tag, text, and domain-specific fields use the approved format and empty-value behavior
- whether clickable / navigational fields have the approved click behavior and visual affordance
- whether default, hover, focus, active, visited, disabled, and permission-denied states match the approved contract
- whether source-required business emphasis was incorrectly implemented as interaction-only affordance
- generic formatter misuse, component-default interaction substitution, or convention-based styling found or ruled out
- required follow-up if failed

`frontend architecture quality assessment` is required when an architecture-design artifact exists, or when scoped work changes module boundaries, ownership, state/data topology, dependency direction, extension seams, migration paths, public APIs, or frontend architecture. It must include:

- result (`pass` or `fail`)
- whether architecture decision ids were implemented and verified
- whether business capability owner and module boundary are clear
- whether ownership map is coherent for page orchestration, business rules, form model, view model, request transport, adapter normalization, server/client/route state, permissions, and side effects
- whether state topology has one writable owner per business state and derived state is not duplicated without approval
- whether data lifecycle keeps DTO semantics out of components, templates, watchers, and display fallbacks
- whether dependency rule matrix was respected, including allowed imports, forbidden imports, public APIs, private symbols, and cycle risk
- whether extension seams are backed by real variation axes and rejected future-proofing stayed rejected
- whether complexity budget was respected or intentionally rolled back
- whether migration, compatibility, fallback, feature flag, or dual-path code has an owner, cleanup trigger, and rollback surface
- architecture blockers found or ruled out: unclear owner, reversed dependency, DTO leakage, duplicated writable state, unjustified seam, unowned temporary path, or unverifiable boundary
- required follow-up if failed

`functional-programming assessment` is required when the scoped work includes business rules, validation, data transformations, payload construction, state derivation, adapter / mapper normalization, or side-effect orchestration. It must include:

- result (`pass` or `fail`)
- pure transformation and rule-boundary findings
- immutability findings
- side-effect boundary findings
- adapter / mapper / `fromDetail` semantic-normalization findings
- hidden mutation or hidden side effects found
- readability findings for composition, reducer chains, currying, or point-free style
- required follow-up if failed

`production code quality assessment` is required when production code was added or changed. It must include:

- result (`pass` or `fail`)
- type-first contract findings, including broad types, unsafe assertions, missing interfaces, or missing branded / nominal distinctions for confusable identifiers
- fail-fast findings for silent failures, swallowed errors, ignored promises, hidden invalid data, or unclear impossible-state handling
- strict null findings for blurred `null` / `undefined` semantics or fallback display logic masking data contract problems
- naming findings for booleans, callback props, internal event handlers, and function verbs
- no magic variable findings, including implicit helper dependencies and misplaced or meaningless configuration constants
- maintainability-first performance findings, including unjustified memoization, caching, debounce, throttle, virtualization, `useMemo` / `useCallback`, computed caching, or watcher optimization
- pure function / immutable data findings, including unnecessary classes or mutation-based data shaping
- boundary UI state findings for touched lists, async operations, and form inputs
- required follow-up if failed

`code review checklist assessment` is required when production code was added or changed. It must include:

- result (`pass` or `fail`)
- plan contract findings: whether `plan/plan.md` contains the code-review checklist contract and whether execute followed it without ad hoc guessing
- robustness findings: external data optional chaining / null checks, API exception handling, explicit abnormal-state handling, and missing guards found or ruled out
- maintainability findings: functions over 150 lines found or ruled out, giant mixed-responsibility functions found or ruled out, and magic numbers without meaningful constants found or ruled out
- performance and memory findings: high-frequency events found or ruled out, debounce / throttle decision-table conformance when applicable, and side effects without cleanup functions found or ruled out
- project convention consistency findings: component library, styling scheme, directory layout, import alias / relative path style, and technology-stack anchoring conformance

`project scaffold assessment` is required when the scoped work creates, initializes, or starts a project, app, package, frontend surface, micro frontend, component library, SDK package, or buildable frontend entry. It must include:

- result (`pass` or `fail`)
- whether scaffold-sensitive detection happened in upstream artifacts
- whether candidate discovery scope and evidence were sufficient
- whether the selected scaffold / starter matched the project type and repository constraints
- whether rejected candidates had concrete reasons
- whether generated / copied structure stayed within approved deviations
- whether hand-written bootstrap bypassed a suitable scaffold
- required follow-up if failed
- unauthorized technology-stack changes found or ruled out; if found, result must be `fail`, the issue must be marked as a blocker, and the implementation must be rewritten instead of accepted
- required follow-up if failed

`human review readiness assessment` is required when production code, tests, mocks, contracts, or generated-facing files were changed. It must include:

- result (`pass` or `fail`)
- whether the diff is scoped, necessary, and reviewer-readable
- unrelated changes, broad formatting churn, or drive-by refactors found or ruled out
- local convention mismatches found or ruled out
- changed hunks without approved task / cleanup / test-adaptation mapping found or ruled out
- debug code, dead code, commented-out code, stale TODOs, stale comments, temporary names, unused imports / exports, stale tests, and stale mocks found or ruled out
- whether tests, type checks, lint checks, manual checks, and unrun-command reasons are sufficient for reviewer trust
- required follow-up if failed

`frontend styling assessment` is required when the scoped work adds or changes authored styling. It must include:

- result (`pass` or `fail`)
- Tailwind CSS-style utility class conformance
- class length and inline reviewability findings
- findings for any constants, maps, computed properties, helpers, or imported variables that hide overlong class values
- findings for business emphasis, link affordance, and default / hover / focus / active / disabled state fidelity
- required follow-up if failed

`API contract assessment` is required when the scoped work touches backend integration. It must include:

- result (`pass` or `fail`)
- contract-source fidelity findings
- backend-owned type reuse findings
- non-TypeScript contract translation and backend field-name preservation findings
- adapter / mapper boundary findings
- request-layer ownership findings
- required follow-up if failed

`TypeScript context assessment` is required when the scoped work is TypeScript-affecting. It must include:

- result (`pass` or `fail`)
- governing `tsconfig` recovery findings
- relevant declaration or generated type source findings
- alias, ambient global, module resolution, JSX runtime, strictness, or generated-type assumption findings
- any guessed TypeScript context found or ruled out
- required follow-up if failed

`design-pattern assessment` must include:

- result (`pass` or `fail`)
- key findings
- whether pattern-fit evaluation exists for the delivery unit, including small/local/bugfix work
- decision depth reviewed: Level 0, Level 1, Level 2, or Level 3
- selected decision: `direct code`, `reuse existing pattern`, `adapt lightweight pattern`, or `introduce pattern`
- triggered candidate signals found or ruled out at the appropriate depth
- direct-code no-signal rationale when no pattern was selected
- triggered rejected alternatives and whether the rejection is reasonable
- frontend syntax shape findings for React, Vue, TypeScript, component, hook / composable, store, request, adapter, and event boundaries
- mechanical enumeration of unrelated pattern families found or ruled out
- overbuilt pattern ceremony, fake extensibility, or missed lightweight pattern opportunities found or ruled out
- required follow-up if failed

`merge readiness summary` must include:

- final result (`merge-ready` or `not merge-ready`)
- confirmation that `self-healing loop compliance` and `self-healing loop assessment` are present and `pass`
- confirmation that every applicable verification and review verdict registered in `constraint-model.md` is present and `pass`
- confirmation that `code review checklist assessment` is present and `pass` when production code changed
- confirmation that `human review readiness assessment` is present and `pass` when production code, tests, mocks, contracts, or generated-facing files changed
- confirmation that `frontend architecture quality compliance` and `frontend architecture quality assessment` are present and `pass` when architecture quality is applicable
- blocking issues that prevent merge readiness, or `none`
- reviewer-facing evidence summary: tests, type checks, lint checks, manual checks, commands not run with reason, and accepted risks
- out-of-scope changes found or ruled out
