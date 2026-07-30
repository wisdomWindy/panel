# Page Design Template

## Required Artifact Path

- `docs/requests/<request-id>/design/page-design.md` for non-split work
- `docs/requests/<request-id>/module-runs/<module-id>/design/page-design.md` for split PRD-driven work

## `design/page-design.md` or `module-runs/<module-id>/design/page-design.md`

Required sections:

- delivery unit identifier
- page objective
- target users or scenarios
- source and upstream constraints
- product-content allowlist mapping
- user journey and task success
- information architecture and content strategy
- layout structure
- visual hierarchy
- visual design direction
- section breakdown
- styling direction
- Tailwind CSS-style styling constraints
- class length and structure strategy
- interaction skeleton
- state and feedback model
- UX writing and feedback copy stance
- responsive behavior
- accessibility and keyboard considerations
- usability heuristics and risk controls
- component reuse and split strategy
- design risks
- open UI questions

### `source and upstream constraints`

Document which upstream artifacts shaped this design:

- PRD snapshot or bugfix source
- requirement module, if split
- requirement-analysis decisions, if present
- existing page or component constraints
- product or design-system constraints that must not be re-decided in spec or execute

### `product-content allowlist mapping`

Map every proposed field, label, copy item, control, action, interaction, state, default, validation, permission presentation, navigation target, status meaning, and feedback behavior to the upstream closed-world product scope allowlist and its exact original requirement or confirmation reference.

Layout, spacing, hierarchy, responsive behavior, and component selection may refine presentation within project constraints, but they must not create product semantics. Anything not present in the allowlist must be recorded under `open UI questions` as `missing-source`; do not place it into the design as an approved element.

### `layout structure`

Document:

- page regions
- primary and secondary content hierarchy
- form, table, detail, modal, drawer, card, or toolbar placement
- empty, loading, error, readonly, disabled, and permission-limited layout states when relevant

### `user journey and task success`

Document the page from the user's task perspective:

- target role and scenario
- user entrypoint and intent
- primary task and decision the page must support
- primary action, secondary actions, destructive actions, and non-goal actions
- preconditions, completion signal, handoff, cancellation, retry, and recovery path
- likely user mistakes and how the design prevents or recovers from them

Do not stop at "show a form / table / detail page". Explain how the page helps the user complete the business task.

### `information architecture and content strategy`

Document:

- business grouping of fields, controls, actions, and messages
- reading order and operation order
- which information must be visible first, which can be secondary, and which can be progressively disclosed
- cross-section dependencies, field dependencies, and action dependencies
- label clarity, ambiguous copy risks, terminology that must match the requirement source, and content density tradeoffs

### `visual design direction`

Document the intended visual effect at a reviewable level:

- visual tone within project constraints, such as compact operations console, guided form, audit workspace, data-heavy table, or decision dashboard
- spacing rhythm, density stance, section surfaces, divider / border usage, and empty-space strategy
- typography emphasis for titles, section headers, key values, metadata, helper text, and warnings
- color / status / icon usage and any non-color fallback for meaning
- skeleton, placeholder, empty-state illustration, or transition policy when material

This is not a pixel-perfect design file. It is a design contract that prevents the implementation from becoming visually flat, noisy, or misleading.

### `Tailwind CSS-style styling constraints`

Required when the page design adds or changes authored styling.

Document:

- styling must use Tailwind CSS-style utility classes
- no new scoped CSS, CSS modules, Sass/Less, inline style objects, or non-utility semantic class names
- conditional class bindings may only express small state toggles
- any existing project style or component convention that constrains utility choices

### `class length and structure strategy`

Required when layout or styling complexity could produce long class values.

Document:

- where markup should be split into smaller components or subcontainers
- where visual complexity should be reduced
- how class values remain reviewable inline
- confirmation that overlong class values must not be hidden in constants, maps, computed properties, helpers, or imports

### `interaction skeleton`

Document:

- user entrypoints
- click, submit, select, hover, focus, keyboard, and navigation behaviors that shape the spec
- confirmation, cancellation, retry, and disabled behaviors
- loading start and end conditions when visible in UI
- disabled reason, permission-denied reason, destructive-action warning, dirty-state handling, and focus restoration when material
- interaction priority: which actions are primary, secondary, inline, batch, contextual, or destructive

### `state and feedback model`

Document:

- empty state
- filtered empty state
- loading state
- partial loading or section-level loading state
- error state
- retryable vs non-retryable failure state
- success feedback
- validation feedback
- permission or readonly state
- pending async operation state
- stale data or refresh-needed state when relevant
- optimistic vs pessimistic update stance when relevant

For each state, record its product-content allowlist reference. If the source does not define whether the state exists, its copy, available actions, visibility, or business outcome, record the missing decision instead of filling it with a conventional empty state, retry button, success toast, confirmation, or fallback.

### `UX writing and feedback copy stance`

Document:

- empty-state guidance
- loading wording when a skeleton or spinner is not enough
- validation error tone and specificity
- API / system failure message stance
- success feedback and next-step guidance
- destructive confirmation wording and irreversible-action warning
- disabled or permission-denied explanation

Do not author new product copy in this section unless the exact wording or writing authority is source-backed or explicitly confirmed. Otherwise record the required copy as `missing-source` and block downstream use.

Copy does not need final product approval here, but the intent must be specific enough for `spec` to preserve.

### `usability heuristics and risk controls`

Document:

- cognitive load risks and grouping mitigations
- scanability risks in dense forms, tables, cards, or detail pages
- hidden dependency risks between fields, filters, actions, or workflow steps
- accidental operation risks and how confirmation, disabled state, copy, or placement reduces them
- accessibility risks: keyboard trap, focus loss, color-only signaling, small target size, unreadable contrast, or unclear error association
- responsive risks: overflow, toolbar wrapping, sticky action collision, drawer / modal sizing, and table scroll behavior

### `component reuse and split strategy`

Document:

- existing shared components or templates to prefer
- page-local components that should be extracted, including the exact trigger: `2+` same-semantics production use, or one single-caller exception such as cohesive business section, interaction lifecycle boundary, slot / composition variation, focused test surface, page readability boundary, or project convention boundary
- page-local sections that must stay inline, including the exact blocker: one-off markup, one-caller pass-through, generic wrapper, broad optional props, DTO / form-model passthrough, different business semantics, local rule hiding, prop / emit chain longer than local markup, or extraction only to make the file shorter
- component boundaries needed to keep Tailwind class values short and readable
- boundaries that should not be extracted because they would hide simple local behavior
- for each Vue extraction candidate: candidate name, owning page / feature, real production caller count (`1`, `2+ same semantics`, or `2+ different semantics`), single-caller exception if any, state owner, props contract, emits contract, slot usage if any, data normalization boundary, styling impact, and verification surface

Vue extraction rules:

- Extract only after `vue-component-extraction.md` decision algorithm passes: real concept name, valid ownership, real caller count, approved `2+` same-semantics reuse or a concrete single-caller exception.
- Do not extract when the component would only pass props / emits through, wrap an existing component without new semantics, accept parent DTO / form model as a passthrough, hide a short local rule, move adapter / request / permission logic into a presentational SFC, or serve unrelated callers through broad flags.
