# Intake Brief (draft v2, awaiting 01-intake agent refinement)

- **Goal:** recreate the documented behavior of the EOL bow-tie risk-analysis desktop tool as a modern 64-bit multiplatform offline desktop app.
- **Why:** owner holds a legitimate license; the owned versions are EOL; activation infrastructure is gone; continued professional use requires a working modern tool.
- **Spec baseline:** v8.2-era corpus (SPEC-013..020) — Software Manual Rev 31 for v8.2 is authoritative; Standard-edition column of the Compare chart defines v1 scope.
- **Scope v1:** Standard-edition bow-tie editor per the Compare chart (SPEC-015) + Software Manual 8.2 (SPEC-013); acceptance = "12 steps" walkthrough (SPEC-004) reproducible.
- **Out of scope v1:** Advanced-edition-only features (◦ marks in the Compare chart), WebViewer portal, SharePoint add-on, CHIMP/AuditXP modules, IncidentXP module (Phase 6, separately gated).
- **Constraints:** clean-room only; no legacy code/assets/branding; new name; offline; private repo; no distribution.
- **Success criteria:** every walkthrough in the Quick Start 8.0 + 12-steps docs executable in the new app with matching model/diagram structure; SPEC-TRACEABILITY.md complete for shipped features.
- **Unresolved:** final product name; exact risk-matrix defaults; whether legacy case-file import is attempted (requires file-format evidence — currently none; treat as out of scope unless human supplies sample files).
