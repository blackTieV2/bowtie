# PROJECT_STATUS.md

Concise current project profile. This is NOT the historical log — history lives in `shared/knowledge/okf/log.md` and `runs/`.

## Profile

```yaml
project: bowtie
repository: https://github.com/blackTieV2/bowtie
default_branch: main
bootstrap_branch: feature/project-governance-bootstrap  # merged via PR #1
timezone: unresolved
recorded_at: 2026-10-12T00:00:00Z
updated_at: 2026-10-12T00:00:00Z   # rev 4: build authorized; design folded into Phase 1 gate (D-006)

current_stage: 04-build
execution_hold: false
build_approved: true
qa_approved: false
deployment_approved: false
release_approved: false
```

## What is the project?

Clean-room reimplementation of the documented behavior of an end-of-life, Windows-only bow-tie risk-analysis suite as a modern 64-bit multiplatform (Windows/macOS/Linux) offline desktop app. Three modules on a shared bow-tie data model: **bow-tie editor** (core), **audit module** (surveys/audits derived from barriers), **incident-analysis module** (timelines, BSCAT/event trees). The repository owner is a legitimate license holder of the legacy software; the original vendor's product line was acquired and these old versions are unsupported.

## What is the target?

- Stack: Tauri 2 (Rust) + TypeScript/React frontend, Konva canvas for diagrams, SQLite case files. Fully offline.
- Spec baseline: v8.2-era corpus, SPEC-013..023 (see `stages/01-intake/references/spec/README.md`). Incident module uses SPEC-002 (10.4) as version-authoritative.
- Scope: Phases 1–5 bow-tie editor (Standard edition), Phase 6 audit module, Phase 7 incident module.

## What is approved / prohibited?

- **Approved:** governance bootstrap (merged PR #1); spec corpus; scope = editor + audit + incident modules (phased); **Phase 1 build work per `stages/04-build/references/REBUILD-MASTER-PROMPT.md` v3**.
- **Prohibited:** merging build branches to `main` without human PR approval; copying any code/assets/branding/trademarks of the legacy application; **any use of the legacy binary, decompilers, or decompiled output as input (hard stop — D-001)**; use of legacy product names for the new app or packages; secrets in the repo.

## Major decisions currently applying

| # | Decision | Authority |
|---|---|---|
| D-001 | Clean-room from documentation only; legacy binary is never an input; decompilation prohibited | human (approved) |
| D-002 | New name/visual identity required; legacy names are live third-party trademarks | human (approved) |
| D-003 | Governance model: 7-stage workflow + gated persistent memory | human (approved) |
| D-004 | Scope includes audit and incident modules as Phases 6–7 | human (approved) |
| D-005 | v8.2-era markdown corpus is the spec baseline; older extractions are provenance only | human (approved) |
| D-006 | 03-design folded into Phase 1 gate: Phase 1 output (feature inventory, SQLite schema, scaffold) serves as the architecture review before application code. Spec ambiguities are resolved by the build agent choosing the interpretation closest to the manual text, each logged in `stages/04-build/output/AMBIGUITY-RESOLUTIONS.md` with alternatives, for human review after the fact. | human (approved, 2026-10-12) |

## Known risks

- Legacy manuals carry a restrictive "training use only" license — keep repo private, work personal/internal.
- Legacy product line continues under its acquirer; no distribution of any clone; no legacy names/branding.
- Diagram canvas auto-layout is the expected hardest engineering problem.
- IncidentXP 8.2 vs 10.4 behavioral differences: default to 10.4, flag conflicts.

## Last accepted checkpoint

- PR #1 merged: governance + spec corpus + build prompt installed.
- Intake outputs produced (intake brief, feature inventory: 44 matrix rows + 51 comparison rows, 30 ambiguities, run packet 001) — superseded in part by D-006 ambiguity-resolution policy.

## Exact next safe action

Build agent executes **Phase 1 only** of `stages/04-build/references/REBUILD-MASTER-PROMPT.md` on a `feature/phase-1-*` branch, per run packet `runs/002-build-phase1/`. Stops at the phase boundary for human review.
