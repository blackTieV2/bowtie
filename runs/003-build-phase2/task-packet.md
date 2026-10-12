# Task Packet — 003-build-phase2

- **Project:** bowtie
- **Stage:** 04-build
- **Task:** Phase 2 ONLY: interactive diagram canvas
- **Starting state:** clean feature/phase-1-data-model, 21fe11430b8a8e02073100f2e942c60d75c8ae20; expected root and origin verified; TARGET GATE: PASS.
- **Tool profile:** build
- **Scope (approved):** Human approved Phase 1 runtime and AR34–36, authorizes Phase 2 on feature/phase-2-diagram-canvas. Konva rendering, add/edit/delete, persisted drag positions, classic geometry auto-layout, zoom/pan, existing CaseStore IPC, graceful RESTRICT diagnostics, SPEC-004 twelve-step GUI workflow. This instruction supersedes PROJECT_STATUS's older Phase 1 next action.
- **Stop conditions:** unexpected dirty work, wrong target, unclear ownership, secrets, scope beyond Phase 2, schema-trigger bypass. No legacy binaries/decompilers/branding; offline application. Resolve ambiguities under D-006 and record alternatives.
- **Evidence requirement:** starting target, changed files, Rust and frontend checks, Windows native build, scripted built-app GUI walkthrough with save/reopen and structure checks, manual gaps, Phase 2 report. Producer checks do not constitute independent QA or human approval.

## Optimization criteria

```yaml
priorities:
  correctness: critical
  preservation_of_existing_work: critical
  reversibility: high
  maintainability: high
  security: critical
  speed: secondary
  token_cost: secondary
```

## Dissent check (for high-risk work)
- Incorrect parentage/link endpoints or lost coordinates after save/reopen would invalidate acceptance; assert complete snapshots and typed-trigger rejection.
- SPEC-004 supplies no named dataset; AR37 uses original content for all twelve method steps, subject to human review.
- Unexpected branch/tree changes or a new execution hold would stop modifications; live starting gate was clean and current human scope explicit.
- Keep approved SQL and disk case intact, edit the transactional SQLite working copy, and isolate all changes on the new feature branch.

## Outcome

Windows producer validation passed: 13 Rust tests, TypeScript/Vite, native release build and twelve-step built-app GUI script. See phase-2-report.md and this run's evidence. Stop for Phase 2 review; Phase 3 remains unauthorized.

Implementation commit: `b78d18f` (`feat(canvas): implement Phase 2 diagram editing and persistence`). Evidence/report are committed separately. No push or merge authorized/performed.
