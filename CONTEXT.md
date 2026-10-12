# CONTEXT.md — Task → Stage Router

This file routes work to stages. It is a router, not a knowledge dump.

## Stages

| Stage | Purpose | Entry contract | Output location |
|---|---|---|---|
| `stages/00-triage/` | Classify task, risk, target, mode; choose next stage. No implementation. | `stages/00-triage/CONTEXT.md` | `stages/00-triage/output/` |
| `stages/01-intake/` | Define goal, scope, constraints, success criteria. Holds the legacy-spec references. | `stages/01-intake/CONTEXT.md` | `stages/01-intake/output/` |
| `stages/02-research/` | Gather and verify evidence. Evidence is not automatically durable memory. | `stages/02-research/CONTEXT.md` | `stages/02-research/output/` |
| `stages/03-design/` | Architecture, interfaces, risks, rollback, acceptance criteria. Does not authorize build. | `stages/03-design/CONTEXT.md` | `stages/03-design/output/` |
| `stages/04-build/` | Implement approved scope only, phase-gated. Requires `TARGET GATE: PASS`. | `stages/04-build/CONTEXT.md` | application code + `stages/04-build/output/` |
| `stages/05-qa/` | Independent validation against acceptance criteria. Producer ≠ validator. | `stages/05-qa/CONTEXT.md` | `stages/05-qa/output/` |
| `stages/06-handoff/` | Record accepted state, evidence, decisions, holds, next safe action. | `stages/06-handoff/CONTEXT.md` | `stages/06-handoff/output/` |

## Routing rules

1. Every task starts at `00-triage` unless the human explicitly assigns a stage.
2. Current stage and holds live in `PROJECT_STATUS.md` — check it before routing.
3. One run = one project, one stage, one task, one bounded packet (`runs/<run-id>/`).
4. Do not combine recon, design, build, QA, and handoff into one uncontrolled mega-run.

## Key current pointers (bootstrap state)

- Legacy behavioral spec (extracted manuals): `stages/01-intake/references/spec/`
- Evidence manifests for the source PDFs: `shared/memory/evidence/manifests/`
- First build prompt (for the build agent, Phase 1): `stages/04-build/references/REBUILD-MASTER-PROMPT.md`
- Known legal/scope constraints: `stages/01-intake/output/intake-brief.md`
