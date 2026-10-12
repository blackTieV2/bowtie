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

current_stage: 00-triage
execution_hold: true
build_approved: false
qa_approved: false
deployment_approved: false
release_approved: false
```

## What is the project?

Clean-room reimplementation of the documented behavior of an end-of-life, Windows-only bow-tie risk-analysis desktop application (and its companion incident-analysis tool) as a modern 64-bit multiplatform (Windows/macOS/Linux) offline desktop app. The repository owner is a legitimate license holder of the legacy software; the original vendor's product line was acquired and these old versions are unsupported.

## What is the target?

- Stack (proposed, awaiting 03-design approval): Tauri 2 (Rust) + TypeScript/React frontend, Konva canvas for diagrams, SQLite case files. Fully offline.
- Scope v1: bow-tie diagram editor per the legacy Standard-edition feature set, verified against the extracted manuals in `stages/01-intake/references/spec/`.

## What is approved / prohibited?

- **Approved:** repository governance bootstrap (this branch); spec extraction from owner-supplied manuals.
- **Prohibited:** application implementation before `build_approved: true`; merging to `main` without human approval; copying any code/assets/branding/trademarks of the legacy application; use of the legacy product name for the new app or packages; storing secrets anywhere in the repo.

## Major decisions currently applying

| # | Decision | Authority |
|---|---|---|
| D-001 | Clean-room from documentation only; legacy binary is never an input | human (approved) |
| D-002 | New name/visual identity required; legacy name is a live third-party trademark | human (approved) |
| D-003 | Governance model: 7-stage workflow + gated persistent memory (see AGENTS.md) | human (approved) |

## Known risks

- Legacy manuals carry a restrictive "training use only" license — keep this repo private and the work personal/internal.
- The legacy product line continues under its acquirer; do not distribute or sell any clone.
- Diagram canvas auto-layout is the expected hardest engineering problem (02/03 stages must size it).

## Last accepted checkpoint

None yet. Bootstrap pending human review of the pull request from `feature/project-governance-bootstrap`.

## Exact next safe action

Human reviews and merges the bootstrap PR, then assigns the intake agent to `stages/01-intake/` using `runs/<run-id>/` packet per AGENTS.md loading order.
