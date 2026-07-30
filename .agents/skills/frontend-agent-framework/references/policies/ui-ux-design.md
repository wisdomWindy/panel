# UI / UX Design Policy

## Scope

Use this policy when a delivery unit enters `page-design`, or when scoped work materially changes layout, visual hierarchy, interaction structure, navigation, empty / loading / error presentation, responsive behavior, or user-facing task flow.

This policy raises `page-design` from layout description to UI / UX design review. It must improve user comprehension, task completion, visual clarity, and interaction confidence without inventing product scope beyond the approved sources.

## Core Principles

- Business task first: design around the user's actual job, decision, risk, and next action, not around available fields or component defaults.
- Information scent: users should understand where they are, what matters most, what can be acted on, and what will happen next without reading implementation details.
- Visual hierarchy: primary content, primary actions, secondary actions, metadata, warnings, and destructive operations must have distinct hierarchy and spacing.
- Interaction confidence: identify the preconditions, feedback, disabled reasons, loading ownership, success result, failure recovery, and cancellation / retry decisions required for meaningful actions; when the source does not authorize one of these product semantics, record it as missing instead of designing it from convention.
- Cognitive load control: group fields and actions by business meaning, frequency, dependency, and decision order; avoid dense undifferentiated lists of controls.
- Consistency with intent: reuse product conventions and design-system patterns when they match the current business meaning; do not copy neighboring UI when the workflow or field semantics differ.
- Accessibility baseline: keyboard, focus, semantic controls, color contrast, text clarity, target size, and screen-size behavior must remain usable for the scoped surface.
- Visual restraint: improve emphasis and scanability with layout, spacing, typography, color, icons, and states only when they clarify task priority or business meaning.

## Required Page-Design Reasoning

Before `spec`, page-design must record these decisions when applicable:

- User journey map: user role, entrypoint, intent, preconditions, primary decision, primary action, secondary action, completion signal, and recovery path.
- Information architecture: page regions, content grouping, reading order, field grouping, dependency order, progressive disclosure, and cross-section relationships.
- Visual hierarchy system: page title, section titles, primary action, secondary action, data density, metadata, warnings, status emphasis, destructive emphasis, and empty-space strategy.
- Interaction model: click, submit, select, search, filter, sort, pagination, tab, drawer, modal, inline edit, upload, keyboard, focus, confirmation, cancellation, retry, and navigation behavior that affects user understanding.
- State model: first load, partial load, empty, filtered empty, error, permission-denied, readonly, disabled, validation error, dirty, submitting, success, stale data, and optimistic / pessimistic update stance.
- Feedback copy stance: success, failure, validation, disabled reason, empty-state guidance, destructive confirmation, and irreversible-action warnings must be concrete enough for `spec` to carry forward.
- Responsive and density behavior: desktop, narrow viewport, table overflow, toolbar wrapping, modal / drawer sizing, sticky action placement, and touch target risks.
- Accessibility and usability: keyboard path, focus order and restoration, semantic control choice, contrast risk, target size, error association, and non-color-only signaling.
- Visual design direction: layout rhythm, spacing scale, typography emphasis, color / status use, icon use, border / surface strategy, skeleton / placeholder style, and animation / transition policy when material.
- UX risk register: likely user mistakes, ambiguous labels, hidden dependencies, irreversible actions, overload points, scanability risks, and mitigation.

## Source-Grounding Rules

- Product-facing UI / UX content must be source-backed or explicitly confirmed. Code facts and safe source-derived consequences may constrain layout, component compatibility, and behavior-preserving presentation, but cannot authorize new product content.
- Page-design may refine presentation and interaction clarity, but it must not add new product behavior, new fields, new actions, new permissions, new validation rules, or new acceptance criteria without source authority.
- Page-design must map every proposed field, label, copy item, control, action, interaction, state, default, validation, permission presentation, navigation target, status meaning, and feedback behavior to the closed-world product scope allowlist.
- If better UX requires a product decision that is not explicitly in the source or confirmed by a human, record it as an open UI / UX question and route to the front-loaded confirmation gate when it affects visible behavior or acceptance. Do not infer it from code facts.
- Existing product conventions are constraints or comparison evidence; they are not authority to override the current requirement's business meaning.

## Review Blockers

Treat these as page-design blockers:

- The artifact describes component placement but not the user's task flow, decision order, or success / failure path.
- Primary action, secondary action, destructive action, or disabled condition is visually or behaviorally ambiguous.
- Empty, loading, error, permission, readonly, validation, or success state occurrence was not analyzed. The design may record an unresolved state as `missing-source`; it must not invent its copy, actions, visibility, or business outcome merely to satisfy this review item.
- A field, label, copy item, control, action, state, validation, navigation target, or feedback behavior lacks a closed-world product scope allowlist reference.
- Visual hierarchy is a flat list of sections or fields with no scan order, grouping rationale, or emphasis rule.
- Dense forms, tables, or detail pages lack grouping, progressive disclosure, sticky actions, filtering / search placement, or overflow strategy when needed.
- Interaction design relies only on hover or color to communicate meaning.
- Responsive, keyboard, focus, or target-size risks are ignored when the surface is user-facing.
- The design copies neighboring UI conventions despite different business meaning.
- The design proposes visual novelty that conflicts with project constraints, Tailwind utility styling rules, or source-grounding limits.
