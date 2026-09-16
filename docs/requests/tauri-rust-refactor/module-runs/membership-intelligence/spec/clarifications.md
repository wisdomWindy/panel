# Membership Intelligence Clarifications

## Decision Register

| Question | Answer | Final decision | Affected area | Grounding status / required source |
| --- | --- | --- | --- | --- |
| What production membership endpoints, plans/currency, order states, payment payloads, token claims, public key, and manifest schema apply? | Not supplied. | Implement typed ports, local state machines, and unavailable production adapter. Do not ship fixture success. | purchase, entitlement, modules | `external-interface-pending`; official contract and Ed25519 key required |
| How can member center exist before its downloadable module? | Requirement analysis normalized PRD 5.1/5.2. | Compile minimal login/purchase/status/download shell in member build; signed resources may extend active state only. | member center/modules | `confirmed-decision` |
| Can downloaded frontend resources execute arbitrary JavaScript? | PRD does not authorize a plugin runtime. | No; only a fixed signed contribution/resource contract is allowed. | loader/CSP | source-derived security constraint; PRD 5.3/5.7/13 |
| What are exact empty messages/actions for catalogs, orders, modules, logs, and alerts? | Not supplied. | Keep stable empty geometry; author no new empty copy/action. | lists | `missing-source`; product copy approval required for future prose |
| What logout/module-cleanup confirmation wording applies? | Not supplied. | Use sourced action/state and standard feedback only; add no custom confirmation copy. | member center/modules | `missing-source` |
| Can in-progress download close/minimize and what happens on OS exit? | PRD only says `请勿关闭应用`. | Disable casual dialog close during non-terminal install; native lifecycle owns interruption/resume; add no copy. | download | partially source-backed; explicit exit UX needed for other behavior |
| What exact order/AI/moderation log columns exist? | Only PRD meanings and current SQL fields. | Render source-backed fields only; omit unsupported columns/actions. | audit pages | `missing-source`; official DTO/field table required to expand |
| What plan/provider/model/rule ranges are allowed? | Contracts/ranges absent; sourced defaults/styles/durations known. | Use validated native catalogs/ranges or unavailable state. Never invent presets/ranges. | forms | `external-interface-pending` / `missing-source` |
| How are API keys stored and returned? | PRD requires configuration but no storage contract. | Rust owns secret; Vue can replace but never receives/logs clear text after save. | AI config | security constraint; architecture selects platform storage |
| What Cookie contract enables moderation? | Login Cookie is memory-only; moderation says separate encrypted handling but gives no contract. | Never reuse/persist QR Cookie. Local alerts work; mute/auto remain unavailable until approved credential port exists. | moderation | `external-interface-pending` |
| Which Whisper artifacts ship? | No manifest/binary/hash supplied. | Production download/start is unavailable. PRD example metadata is test/display-only until signed metadata exists. | ASR | `external-artifact-pending` |
| What LLM streaming schema/retries apply? | No provider contract; PRD gives 5s timeout and rate-limit copy. | Provider port is unavailable without config; implement sourced timeout/rate-limit/local processing only. | AI transport | `external-interface-pending` |
| Do these gaps block implementation? | They block real external success and extra product copy/fields only. | Proceed with source-backed local/domain/UI behavior and fail-closed production ports; no user gate required by controlled-degradation Done Signal. | whole module | request Done Signal, PRD 14, module requirement |

## Approval State

- Open product behavior admitted into spec: none.
- External production success claims currently allowed: none for membership/payment/distribution, moderation transport, LLM, or Whisper distribution.
- Frontend/native local implementation may proceed: yes.
- Return-to-spec trigger: a new contract changes visible fields, status meaning, permission, validation, flow, or acceptance behavior.
