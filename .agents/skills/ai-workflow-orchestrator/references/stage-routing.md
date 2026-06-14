# Stage Routing

## Route Table

- `intake_analysis`: new requirements, requirement clarification, solution framing
- `feishu_spec_sync`: create or update Feishu tech docs and project sync items
- `implementation`: plan, implement, review, PR preparation
- `bugfix`: defect-ID-driven diagnosis and repair
- `release`: release notes, risk checks, rollback focus

## Internal Subskill Mapping

- `intake_analysis` -> `subskills/ai-workflow-intake/SKILL.md`
- `feishu_spec_sync` -> `subskills/ai-workflow-feishu-spec/SKILL.md`
- `implementation` -> `subskills/ai-workflow-implementation/SKILL.md`
- `bugfix` -> `subskills/ai-workflow-bugfix/SKILL.md`
- `release` -> `subskills/ai-workflow-release/SKILL.md`

## Backflow

- `implementation -> intake_analysis`
- `implementation -> feishu_spec_sync`
- `bugfix -> intake_analysis`
- `review feedback -> feishu_spec_sync`
- `release -> implementation`
