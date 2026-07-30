# Source Grounding Policy

Use this policy whenever request artifacts are created, approved, planned, implemented, verified, or reviewed.

The goal is to prevent requirement expansion beyond the user's source material and confirmed decisions, and to prevent repository facts from being misused as product authority.

## Authoritative Sources

Allowed authority sources are:

- direct user instruction for the active request
- original PRD, defect report, local requirement file, or retrieved upstream document
- durable source snapshots under `docs/requests/<request-id>/artifacts/`
- requirement-analysis, requirement-splitting, page-design, architecture-design, spec, and plan artifacts that passed the framework gates, but only as carriers of their own traceable upstream authority
- repository code facts discovered from scoped code reading, code graph, tests, generated contracts, or documented project conventions
- explicit human confirmations recorded in request artifacts

Do not treat the following as authority for new scope:

- general product or frontend convention
- behavior from a neighboring module unless it is explicitly in the source or required by a confirmed shared contract
- sample request business content
- preferred architecture, preferred UX, or personal implementation taste
- assumptions made to make a document look complete

Downstream artifacts do not become independent authority merely because the framework approved them. Every product-facing item they contain must retain an unbroken trace to the original requirement source or an explicit human confirmation.

## Closed-World Scope Rule

Treat the approved requirement scope as closed, not open-ended:

- absence from the authoritative requirement source is not permission to add a plausible behavior
- product-facing or business-semantic additions require `source-backed` or `confirmed-decision` authority
- repository code, tests, neighboring modules, design systems, and quality policies may constrain how approved behavior is implemented, but cannot authorize new product behavior
- completeness, consistency, UX quality, defensive programming, or similarity to another page is not authority to add fields, copy, controls, interactions, states, defaults, validation, permissions, navigation, API semantics, analytics, or acceptance criteria
- when approved behavior cannot be implemented without choosing an unspecified product semantic, classify it as `missing-source` and return to the front-loaded confirmation gate

## Grounding Labels

Every material scope item, behavior, field, interaction, state rule, API contract, dependency, task, and acceptance criterion must be classifiable as one of:

- `source-backed`: directly stated in an authoritative requirement source
- `code-fact-backed`: a technical constraint, compatibility obligation, dependency-closure cleanup, or behavior-preserving implementation fact required by scoped repository code; it cannot independently authorize new product-facing or business-semantic behavior
- `confirmed-decision`: explicitly confirmed by user, product, backend, or another named owner and recorded in artifacts
- `source-derived`: a narrow consequence of a source-backed item where every plausible interpretation leads to the same downstream spec, plan, implementation, and verification result
- `missing-source`: not supported enough to define behavior; must become a clarification, blocker, external-interface pending item, or non-scope note
- `out-of-scope`: adjacent or tempting work that must not be silently absorbed

Only `source-backed`, `code-fact-backed`, `confirmed-decision`, and safe `source-derived` items may drive downstream spec, plan, execution, or acceptance criteria.

`source-derived` is allowed only to expose implementation-relevant dependency, risk, or normalization consequences. It must not create new product behavior, new fields, new states, new interactions, new validation rules, new API requirements, new modules, or new acceptance criteria.

For any new user-visible or business-semantic item, `code-fact-backed` and `source-derived` are supporting evidence only. The authorizing label must be `source-backed` or `confirmed-decision`.

## Artifact Rules

- Each stage artifact must preserve source references for material decisions.
- If an item has no source reference, label it `missing-source` instead of rewriting it as a requirement.
- If multiple interpretations are possible, route it to the front-loaded confirmation gate instead of choosing the most convenient interpretation.
- If a source only says a business result and omits implementation-relevant semantics, record the omitted semantics as missing or pending rather than filling them from convention.
- If repository code shows a technical constraint, use it to bound implementation, not to expand product scope.
- If adjacent modules suggest a possible behavior, record it as a candidate comparison or clarification, not as current-scope behavior.
- Maintain an explicit allowlist of product-facing and business-semantic content approved for implementation. At minimum this covers fields, copy, controls, interactions, states, defaults, validation, permissions, navigation, API or payload semantics, status mappings, analytics, and acceptance behavior.
- Do not add an item to source artifacts after implementation merely to justify code that was already written. New information discovered during execution must trigger rollback to the owning upstream stage before that content remains in code.
- Technical necessities may be recorded as `code-fact-backed` only when they preserve approved behavior and introduce no new product choice. Record the exact code fact, the constrained implementation choice, and why no user-visible or business-semantic alternative is being selected.

## Approval Rules

Framework automatic approval for `spec` and `plan` must fail when:

- a requirement, behavior, task, or acceptance criterion has no grounding label or source reference
- a `missing-source` item is used as if it were confirmed behavior
- a `source-derived` item creates new product semantics
- a user-visible or business-semantic item relies only on `code-fact-backed` or `source-derived` authority
- a downstream artifact cites another downstream artifact without an unbroken trace to the original requirement source or explicit human confirmation
- implementation content was added first and source artifacts were edited afterward to rationalize it
- direct-change or bugfix work expands beyond the explicit user instruction or observed / expected defect behavior
- sample content, neighboring-module behavior, or convention is used to add scope without confirmation

When approval fails because grounding is missing, roll back to `requirement-analysis`, `intake`, or `bugfix-intake` as the appropriate front-loaded confirmation gate.

## Direct-Change And Bugfix Rules

For direct-change requests, the direct user instruction is the primary source. Repository facts may identify impact scope, references, callers, and cleanup needs, but must not add unrelated refactors or neighboring improvements.

For bugfix requests, observed behavior, expected behavior, reproduction scope, and affected module are the primary source. Do not invent a broader redesign unless the defect source or a confirmed decision explicitly requires it.

For removal or cleanup requests, code reference analysis may discover dependent cleanup items. Cleanup is in scope only when it is part of the dependency closure of the requested change; adjacent opportunistic cleanup remains out of scope.
