# Intake Brief (draft v3, awaiting 01-intake agent refinement)

- **Goal:** recreate the documented behavior of the EOL bow-tie risk-analysis desktop tool as a modern 64-bit multiplatform offline desktop app, **including its audit and incident-analysis modules** — bow-ties are the shared data model: audits verify barrier health, incident analysis reuses the same barriers/BSCAT structures.
- **Why:** owner holds a legitimate license; the owned versions are EOL; activation infrastructure is gone; continued professional use requires a working modern tool.
- **Spec baseline:** v8.2-era corpus (SPEC-013..023) — Software Manual Rev 31 for v8.2 is authoritative; Standard-edition column of the Compare chart defines core scope.
- **Scope (phased):**
  - Phases 1–5: Standard-edition bow-tie editor (data model, canvas, properties/risk matrices/action register, reporting, compare/find/import-export/lookup tables).
  - Phase 6: **Audit module** — audit surveys/questions derived from bow-tie barriers, audit execution, scoring, and reporting (SPEC-021 ch. 24, SPEC-023).
  - Phase 7: **Incident module** — incident diagrams, timelines, BSCAT/event trees, linkages to bow-tie barriers (SPEC-002 version-authoritative, SPEC-022 structure reference).
- **Out of scope:** Advanced-edition-only features (◦ marks in the Compare chart), WebViewer portal, SharePoint add-on, CHIMP module.
- **Constraints:** clean-room only; no legacy code/assets/branding; new name; offline; private repo; no distribution.
- **Success criteria:** every walkthrough in the Quick Start 8.0 + 12-steps docs executable in the new app with matching model/diagram structure; audit and incident workflows reproducible per their manuals; SPEC-TRACEABILITY.md complete for shipped features.
- **Unresolved:** final product name; exact risk-matrix defaults; whether legacy case-file import is attempted (requires file-format evidence — none available; out of scope unless human supplies sample files); IncidentXP 8.2-vs-10.4 behavioral differences (flag conflicts to human, default to 10.4).
