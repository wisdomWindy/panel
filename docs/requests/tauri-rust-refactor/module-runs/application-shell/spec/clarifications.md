# Application Shell Clarifications

## Clarification 1

- question: Do module references `9.16/9.19/9.20` override the current PRD section numbering?
- answer: No. The module boundary uses feature numbers; the original PRD locates configuration at 9.17, multilanguage at 9.20 and base UI at 9.21.
- final decision: Original PRD 8, 9.17, 9.20 and 9.21 are authoritative.
- affected spec area: scope, source mapping and acceptance.
- grounding status and required source: `source-backed`; current PRD and requirement-map/module split.

## Clarification 2

- question: Which font-size values are allowed when the wireframe only displays `中`?
- answer: The PRD authorizes a closed font-size selector and shows medium; the existing preserved configuration uses numeric 14 as its default.
- final decision: use `small/medium/large`, preserve `medium=14px`, and let architecture lock the two adjacent technical scale values without adding arbitrary/custom font input.
- affected spec area: UI preference DTO, validation and responsive tests.
- grounding status and required source: `source-backed` control/default plus `code-fact-backed` preserved configuration; no additional product option family.

## Clarification 3

- question: What may the drag layout editor change while PRD 8 fixes four regions?
- answer: It cannot add, remove, rename or change the semantic position of toolbar/navigation/content/status.
- final decision: custom layout edits only bounded region sizes in the fixed four-region arrangement; architecture chooses safe numeric bounds needed to preserve 1024x680.
- affected spec area: layout type, editor and acceptance AS-A11.
- grounding status and required source: `source-backed` PRD 8 and 9.21; technical bounds are reversible `code-fact-backed` constraints.

## Clarification 4

- question: What does the shell render inside the notification surface?
- answer: The PRD requires the toolbar trigger and a notification-center surface but does not define entry fields, history, read state, actions or empty copy.
- final decision: this module owns trigger/open/close/focus only. Content is `missing-source/downstream` and must not be fabricated; `extensions-release-resilience` supplies it later.
- affected spec area: toolbar, notification extension input and AS-A16.
- grounding status and required source: trigger is `source-backed`; content remains `missing-source` pending its owning module/source.

## Clarification 5

- question: How can member navigation be verified before membership implementation?
- answer: Shell visibility rules are source-backed, while real identity/payment/token state is external and owned by the next module.
- final decision: define/test a typed member-shell projection in member builds; production `unavailable` never maps to logged-out/active success. The next module supplies the producer and routes.
- affected spec area: capability input, navigation and AS-A05/AS-A16.
- grounding status and required source: `source-backed` matrix; real service is `external-interface-pending`.

## Clarification 6

- question: Does application-shell finish every backup/migration/update/plugin configuration behavior?
- answer: It owns the source-backed configuration UI, typed common contract and safe current-scope operations. Complete migration/disaster-recovery and update/plugin integrations remain in the final resilience module.
- final decision: implement native path/transaction UI and common scopes now; expose only typed extension boundaries for later scopes and never claim downstream integration complete.
- affected spec area: config operations, handoff and AS-A07/AS-A08/AS-A16.
- grounding status and required source: `source-backed` PRD 9.17 plus requirement-map downstream ownership.

## Final Clarification Status

- blocking product questions: none.
- external/downstream pending: member state producer and notification/config-resilience extensions.
- assumptions allowed downstream: only reversible numeric layout/font technical bounds that preserve all visible source semantics and are frozen by contract tests.
