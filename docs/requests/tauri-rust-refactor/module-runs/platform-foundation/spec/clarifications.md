# Platform Foundation Clarifications

## Resolved Decisions

| Question | Answer | Final decision | Affected area | Grounding status / source |
| --- | --- | --- | --- | --- |
| Should the migration generate a second Vue application? | No. The repository already has the required Vue/Vite application. | Initialize the Tauri v2 backend against the existing frontend. | Scaffold | code-fact-backed; official Tauri existing-frontend workflow |
| Does the schema contain 17 or 21 tables? | The PRD narrative and SQL disagree; the explicit SQL defines 21 tables. | Implement and test the 21 explicit tables plus seven indexes. | SQLite | source-backed; PRD 12.1-12.2 explicit SQL is more specific |
| How can the member center exist before module download? | PRD 4.3 requires login/member center before payment, while 5.2 packages fuller member-center resources. | Member binary embeds the minimal pre-payment shell/contract; activated resources may extend it later. | Build feature boundary | source-backed and logically necessary source-derived reachability; no extra visible semantics |
| What is the malformed module directory example? | The intended hierarchy is module name followed by version. | Normalize to `modules/<module_name>/<version>/`. | Managed paths | source-ambiguity resolution; PRD 4.5 and 5.3 |
| May Electron stay as fallback host? | No production fallback is authorized. | Keep old files only until direct callers migrate; Tauri becomes the sole target runtime. | Migration cleanup | user intent + source-backed target architecture |

## External Interface Pending

### Question

What production endpoints, credentials and signed artifacts should the foundation use for OAuth, payments, membership tokens, module/update manifests, Bilibili Cookie moderation, FFmpeg and whisper?

### Answer

They are not present in the PRD or repository.

### Final Decision

Define narrow ports and explicit `external_service_unavailable`/artifact-missing states only. Do not create fake endpoints, credentials, signatures, successful payment results or placeholder binaries. Local architecture, free/common functionality and compile isolation may proceed. Each owning later module must return to spec if supplied contracts change semantics.

### Affected Spec Area

External adapters, module 5 membership/AI and module 6 update/plugin/release integration.

### Grounding Status And Required Source

- Status: `external-interface-pending`, non-blocking for platform foundation.
- Required later: authoritative OpenAPI/protocol, environment/endpoint ownership, Ed25519 public key and rotation policy, signed manifest format, approved Sidecar binaries/checksums and license/source notices.

## No Open Product-Semantic Questions

The platform foundation introduces no visible product flow. All remaining unknowns are external deployment artifacts or reversible implementation details bounded by the approved architecture stage; there is no `missing-source` item that requires inventing user-visible behavior.
