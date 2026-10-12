# PROJECT_STATUS.md

Concise current project profile. This is NOT the historical log — history lives in `shared/knowledge/okf/log.md` and `runs/`.

## Profile

```yaml
project: bowtie
repository: https://github.com/blackTieV2/bowtie
default_branch: main
bootstrap_branch: feature/project-governance-bootstrap
timezone: unresolved
recorded_at: 2026-10-12T00:00:00Z
updated_at: 2026-10-12T00:00:00Z   # rev 3: scope expanded to audit + incident modules

current_stage: 00-triage
execution_hold: false
build_approved: true
qa_approved: false
deployment_approved: false
release_approved: false
```

## What is the project?

Clean-room reimplementation of the documented behavior of an end-of-life, Windows-only bow-tie risk-analysis suite as a modern 64-bit multiplatform (Windows/macOS/Linux) offline desktop app. Three modules on a shared bow-tie data model: **bow-tie editor** (core), **audit module** (surveys/audits derived from barriers), **incident-analysis module** (timelines, BSCAT/event trees). The repository owner is a legitimate license holder of the legacy software; the original vendor's product line was acquired and these old versions are unsupported.

## What is the target?

- Stack (proposed, awaiting 03-design approval): Tauri 2 (Rust) + TypeScript/React frontend, Konva canvas for diagrams, SQLite case files. Fully offline.
- Spec baseline: v8.2-era corpus, SPEC-013..023 (see `stages/01-intake/references/spec/README.md`). Incident module uses SPEC-002 (10.4) as version-authoritative.
- Scope: Phases 1–5 bow-tie editor (Standard edition), Phase 6 audit module, Phase 7 incident module.

## What is approved / prohibited?

- **Approved:** repository governance bootstrap (this branch); spec extraction and owner-supplied markdown conversions; scope = editor + audit + incident modules (phased).
- **Prohibited:** application implementation before `build_approved: true`; merging to `main` without human approval; copying any code/assets/branding/trademarks of the legacy application; use of legacy product names for the new app or packages; storing secrets anywhere in the repo.

## Major decisions currently applying

| # | Decision | Authority |
|---|---|---|
| D-001 | Clean-room from documentation only; legacy binary is never an input | human (approved) |
| D-002 | New name/visual identity required; legacy names are live third-party trademarks | human (approved) |
| D-003 | Governance model: 7-stage workflow + gated persistent memory (see AGENTS.md) | human (approved) |
| D-004 | Scope expanded to include audit and incident modules as Phases 6–7 | human (approved, 2026-10-12) |
| D-005 | v8.2-era markdown corpus is the spec baseline; older extractions are provenance only | human (approved, 2026-10-12) |

## Known risks

- Legacy manuals carry a restrictive "training use only" license — keep this repo private and the work personal/internal.
- The legacy product line continues under its acquirer; do not distribute or sell any clone.
- Diagram canvas auto-layout is the expected hardest engineering problem (02/03 stages must size it).
- IncidentXP 8.2 vs 10.4 behavioral differences may conflict; flag to human, default to 10.4.

## Last accepted checkpoint

None yet. Bootstrap pending human review of the pull request from `feature/project-governance-bootstrap`.

## Exact next safe action

Human reviews and merges the bootstrap PR, drops the spec `.md` files into `stages/01-intake/references/spec/`, then assigns the intake agent to `stages/01-intake/` using a `runs/<run-id>/` packet per AGENTS.md loading order.
