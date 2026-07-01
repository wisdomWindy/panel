# Framework Overview

This framework exposes one default entry skill plus stage-specific subskills:

- the main `backend-agent-framework` skill routes work
- `subskills/` own stage execution
- `references/` hold durable workflow, policy, and template material

All implementation work is tracked under `docs/requests/<request-id>/`.

This framework separates three independent control dimensions:

- goal: what counts as done for the request
- loop: how the main skill keeps re-evaluating progress and gates
- workflow: whether execution stays single-threaded or scales out into parallel work units

The canonical request lifecycle is:

`intake -> requirement-splitting -> spec -> plan -> execute -> verify -> review -> complete`

For backend-structural requests, insert the optional design stage before spec:

`intake -> requirement-splitting -> architecture-design -> spec -> plan -> execute -> verify -> review -> complete`

The lifecycle above is the delivery path. The main `backend-agent-framework` skill controls that path through a loop orchestrator:

`observe state -> decide next action -> dispatch one stage or gate recovery action -> observe result -> iterate`

In this framework:

- `subskills/` own stage execution only
- `requirement-splitting` is the source-preserving decomposition stage for PRD-driven work and feeds both `architecture-design` and `spec`
- the goal contract is established in request/spec artifacts and proved by verification/review/release-readiness artifacts
- the main skill owns loop control, gate recovery, and rollback routing
- stage completion should auto-dispatch the next unblocked stage in the same run by default
- workflow-style parallelization is optional execution scale, not a second lifecycle and not a default
- `verify -> execute`, `review -> execute`, and `execute -> spec/plan` are loop re-entry paths, not a second lifecycle
- `state.json.loop` is the durable loop audit trail for iteration count, blocking gate, and re-entry reason

## Sample Request Reference

The repository includes a finished sample request at:

`docs/requests/prd-material-delivery-config-center/`

Use this sample to understand:

- the expected request directory shape
- the quality bar for stage artifacts
- how a request looks after it reaches `complete`

Do not treat the sample as reusable downstream content for a new request.
