# Framework Overview

This framework exposes one default entry skill plus stage-specific subskills:

- the main `frontend-agent-framework` skill routes work
- `subskills/` own stage execution
- `references/` hold durable workflow, policy, and template material

All implementation work is tracked under `docs/requests/<request-id>/`.

This framework separates three independent control dimensions:

- goal: what counts as done for the request
- loop: how the main skill keeps re-evaluating progress and gates
- workflow: whether execution stays single-threaded or scales out into parallel work units

It also treats proactive workflow design as a first-class concern:

- trigger: what event or condition starts work
- context: what information sources the workflow can rely on
- steerability: how humans inspect and redirect work in progress
- handoff: where results are delivered for review or downstream action

The canonical request lifecycle is:

`intake -> spec -> plan -> execute -> verify -> review -> complete`

For page-oriented requests, insert the optional design stage before spec:

`intake -> page-design -> spec -> plan -> execute -> verify -> review -> complete`

The lifecycle above is the delivery path. The main `frontend-agent-framework` skill controls that path through a loop orchestrator:

`observe state -> decide next action -> dispatch one stage or gate recovery action -> observe result -> iterate`

In this framework:

- `subskills/` own stage execution only
- the goal contract is established in request/spec artifacts and proved by verification/review artifacts
- the main skill owns loop control, gate recovery, and rollback routing
- workflow-style parallelization is optional execution scale, not a second lifecycle and not a default
- proactive workflows must make trigger, context, steerability, and handoff explicit in repository artifacts
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
For a new request, only reuse the structure and quality baseline, then rewrite the request-specific artifacts from the current PRD and clarifications.
