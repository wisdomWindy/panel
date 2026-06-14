# Framework Overview

This framework exposes one default entry skill plus stage-specific subskills:

- the main `frontend-agent-framework` skill routes work
- `subskills/` own stage execution
- `references/` hold durable workflow, policy, and template material

All implementation work is tracked under `docs/requests/<request-id>/`.

The canonical request lifecycle is:

`intake -> spec -> plan -> execute -> verify -> review -> complete`

For page-oriented requests, insert the optional design stage before spec:

`intake -> page-design -> spec -> plan -> execute -> verify -> review -> complete`

## Sample Request Reference

The repository includes a finished sample request at:

`docs/requests/prd-material-delivery-config-center/`

Use this sample to understand:

- the expected request directory shape
- the quality bar for stage artifacts
- how a request looks after it reaches `complete`

Do not treat the sample as reusable downstream content for a new request.
For a new request, only reuse the structure and quality baseline, then rewrite the request-specific artifacts from the current PRD and clarifications.
