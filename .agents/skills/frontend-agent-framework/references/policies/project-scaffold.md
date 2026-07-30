# Project Scaffold Policy

## Scope

Use this policy when scoped work creates, initializes, or starts from scratch any project, app, package, route-level frontend surface, micro frontend, plugin, demo app, component library, SDK package, or buildable frontend entry.

The goal is to prevent hand-written greenfield bootstrap when a team-approved scaffold, starter, template, generator, sibling project baseline, or monorepo creation command exists.

## Trigger Signals

Treat the request as scaffold-sensitive when the source or user instruction includes signals such as:

- create project, new project, initialize app, bootstrap app, start from scratch, greenfield
- 新建项目、创建项目、初始化项目、搭建应用、从 0 开始、脚手架、starter
- new package, new app, new workspace, new route surface, micro frontend, component library, SDK package
- adding a buildable frontend surface rather than only editing an existing page / component

If any signal appears, `intake` must record `greenfield_scaffold_sensitive=true` or an equivalent artifact note before downstream stages.

## Discovery Order

Before `spec` can approve a greenfield delivery unit, discover scaffold candidates in this order:

1. Source-stated scaffold, starter, generator, template, or repository baseline.
2. Repository-local project documentation, root `AGENTS.md`, README, package scripts, monorepo config, workspace generators, or `.agents` guidance.
3. Existing sibling apps / packages of the same project type, framework, deployment target, routing model, state management, component library, styling scheme, request layer, and build tool.
4. Team or organization templates already present in the repository, scripts, internal packages, or documented commands.
5. External framework scaffold only when no repository / team scaffold exists and the approved source allows creating a new technology surface.

Use scoped discovery. Do not perform broad internet or registry exploration unless the approved task explicitly requires selecting a new external stack and network use is allowed.

## Decision Rules

- If a matching scaffold or starter exists, scaffold reuse is the default.
- A sibling app can be a scaffold when it represents the same architecture family and copying / trimming it is safer than hand-writing bootstrap.
- Rejecting a suitable scaffold requires a recorded reason: wrong project type, incompatible stack, unavailable source, security / licensing issue, excessive adaptation cost, or explicit source decision.
- Multiple viable scaffolds with materially different stack, deployment, ownership, or long-term maintenance consequences require front-loaded human confirmation.
- No scaffold found is a decision, not an assumption. The artifact must record where discovery looked and why the search was sufficient.
- Implementation may trim, replace, or extend scaffold output only within approved deviations.

## Stage Responsibilities

- `intake`: detect scaffold-sensitive greenfield work, record the signal, and capture source-stated scaffold requirements or missing scaffold information.
- `requirement-analysis`: for PRD-driven greenfield work, record project type, ownership, target runtime, stack constraints, and scaffold candidate discovery needs.
- `spec`: choose the scaffold / starter or explicitly reject all discovered candidates before approval.
- `architecture-design`: align module boundaries, dependency direction, and ownership with the approved scaffold when architecture design is required.
- `plan`: break scaffold application, trimming, dependency changes, directory adjustments, configuration, and verification into explicit tasks.
- `execute`: apply the approved scaffold first; do not hand-write bootstrap unless the approved spec / plan rejected available scaffolds.
- `verify`: check the generated / initialized structure against the scaffold decision and approved deviations.
- `review`: block hand-written greenfield bootstrap when a suitable scaffold was available or when discovery evidence was missing.

## Required Artifact Evidence

For greenfield work, artifacts must record:

- project type and target surface
- scaffold-sensitive signal source
- candidate list and discovery locations
- selected scaffold / starter and why it matches
- rejected candidates and reasons
- approved deviations
- initialization command or copy strategy
- post-scaffold trimming / replacement tasks
- verification evidence

## Blockers

Treat these as blockers:

- Starting greenfield implementation before scaffold discovery and decision are recorded.
- Writing custom bootstrap files while scaffold candidates are unknown.
- Rejecting a matching scaffold with vague reasons such as "simpler to write manually" or "not needed".
- Using an external scaffold while a repository / team scaffold exists and has not been rejected with evidence.
- Copying a sibling project without identifying which parts are scaffold baseline and which are unrelated business code to remove.
- Leaving scaffold choice for `execute` to decide.
