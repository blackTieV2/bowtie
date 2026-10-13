# PROJECT_STATUS.md

Concise current project profile. This is NOT the historical log — history lives in `shared/knowledge/okf/log.md` and `runs/`.

## Profile

```yaml
project: bowtie
repository: https://github.com/blackTieV2/bowtie
default_branch: main
timezone: unresolved
recorded_at: 2026-10-12T00:00:00Z
updated_at: 2026-10-13T00:00:00Z   # rev 5: all seven build phases approved and merged via PR #3

current_stage: 06-handoff
execution_hold: false
build_approved: true
qa_approved: false        # producer + independent validators passed; human acceptance testing pending
deployment_approved: false
release_approved: false
```

## What is the project?

Clean-room reimplementation of the documented behavior of an end-of-life, Windows-only bow-tie risk-analysis suite as a modern 64-bit multiplatform (Windows/macOS/Linux) offline desktop app. Three modules on a shared bow-tie data model: **bow-tie editor** (core), **audit module** (surveys/audits derived from barriers), **incident-analysis module** (timelines, event trees, RCA). Application name: **Barrier Atlas**. The repository owner is a legitimate license holder of the legacy software; the original vendor's product line was acquired and these old versions are unsupported.

## What is the target?

- Stack: Tauri 2 (Rust) + TypeScript/React frontend, Konva canvas for diagrams, SQLite case files. Fully offline.
- Spec baseline: v8.2-era corpus, SPEC-013..023 (see `stages/01-intake/references/spec/README.md`). Incident module uses SPEC-002 (10.4) as version-authoritative.
- Build complete: Phase 1 data model → Phase 7 incident module, all human-approved and merged to `main` via PR #3 (24 commits).

## What is approved / prohibited?

- **Approved:** governance bootstrap (PR #1); spec corpus; Phases 1–7 (each stopped at its boundary and was human-approved); D-007 Advanced-edition exceptions (read-only case compare, find & replace, CSV/XLSX exchange); the AR78 constrained migration (`user_version` 2, RCA event↔barrier links); three-platform CI.
- **Prohibited:** copying any code/assets/branding/trademarks of the legacy application; **any use of the legacy binary, decompilers, or decompiled output as input (hard stop — D-001)**; use of legacy product names for the new app or packages; secrets in the repo; distribution of the app (personal/internal use only).

## Major decisions currently applying

| # | Decision | Authority |
|---|---|---|
| D-001 | Clean-room from documentation only; legacy binary is never an input; decompilation prohibited | human (approved) |
| D-002 | New name/visual identity required; legacy names are live third-party trademarks | human (approved) |
| D-003 | Governance model: 7-stage workflow + gated persistent memory | human (approved) |
| D-004 | Scope includes audit and incident modules as Phases 6–7 | human (approved) |
| D-005 | v8.2-era markdown corpus is the spec baseline; older extractions are provenance only | human (approved) |
| D-006 | Design folded into Phase 1 gate; agent resolves spec ambiguities with logged ARs for after-the-fact human review | human (approved, 2026-10-12) |
| D-007 | Advanced-edition features require explicit per-feature human exception; granted: read-only compare, find & replace, CSV/XLSX exchange | human (approved, 2026-10-13) |

Ambiguity register: AR01–AR85 in `stages/04-build/output/AMBIGUITY-RESOLUTIONS.md` (AR45 resolved-by-correction; AR78 resolved-by-migration).

## Known risks / remaining debt

- Legacy manuals carry a restrictive "training use only" license — keep repo private, work personal/internal.
- Legacy product line continues under its acquirer; no distribution; no legacy names/branding. Third-party methodology marks (BSCAT, Tripod Beta) used nominatively only.
- macOS/Linux: builds and cargo tests pass in CI; **native GUI walkthroughs verified on Windows only**.
- Native file-chooser UX, real Microsoft Excel interop, large-case performance: untested.
- Independent (fresh-context) validation passed per phase; no external QA team review.

## Last accepted checkpoint

- PR #3 merged: Phases 1–7 complete. 49 Rust tests, native GUI walkthroughs (12-step method, risk, exports, exchange, audit, incident), deterministic exports, independent per-phase validation, three-platform CI green.
- Evidence: `runs/002`–`008` packets; phase reports in `stages/04-build/output/`.

## Exact next safe action

Human acceptance testing: download a CI build artifact (Actions → latest run on `main`) or build locally (`cd app && npm ci && npm run tauri build`), exercise the app against real case work, and record the verdict. Defects become new ARs/issues under 05-qa. No new feature phases are authorized.
