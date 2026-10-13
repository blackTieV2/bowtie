# Task Packet — 006-build-phase5

Stage: 04-build. Task: Phase 5 ONLY.
Historical initial authority: Phase 4 and AR55–AR62 approved; Phase 5 authorized with an explicit Standard-edition stop check. Current authority is D-007 and the consolidated instruction below.
Starting state: sole root C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie; clean feature/phase-4-reporting at 72e9a1c138b55aedd5e7979c18c4d473a5357b4a; origin https://github.com/blackTieV2/bowtie.git. TARGET GATE: PASS before modifications.
Branch: feature/phase-5-case-tools created from approved tip.
Historical stop: SPEC-015 marks case compare, replacement, and external spreadsheet import Advanced-only. SPEC-013 §19.1 confirms case compare. AR63 resolved by human D-007 on 2026-10-13.
Historical blocker checkpoint: read-only governance/target/spec recon; branch/packet and blocker documentation only. At that checkpoint no app/schema implementation or tests changed and no completion push was performed. Current continuation implements authorized Phase 5 tools; schema remains unchanged.
Constraints: no legacy binaries/decompilers/branding; original code/formats; offline; parameterized SQL and authoritative triggers; no merge/main push or Phase 6.
Resume authority: current human instruction grants D-007 per-feature exceptions: validated read-only second case, diff view/report only (no merge/copy/apply); find/replace preview and transactional individual/all apply; original spreadsheet/CSV layout, validate-first, dry-run summary, single-transaction CaseStore apply; Standard lookup completion. Stop and log an AR if spec documents compare merge/copy/apply direction. All earlier Phase 5 requirements remain binding.
Resume target gate: PASS, 2026-10-13. Clean feature/phase-5-case-tools at ad08c71c025cd8733e5d3a475fed855b6e717d86, same sole root and expected origin, build approved, no execution hold. PROJECT_STATUS bootstrap next-action text is historical and superseded for this run by explicit current human authority.
SPEC-013 §§19.1–19.4 preflight: describes comparing and reporting differences, with no merge/copy/apply direction found.
Next safe action: implement authorized Phase 5 scope; verify and report; push feature branch; stop before Phase 6.

## Consolidated current human authorization — preserved verbatim

PHASE 5 FULL AUTHORIZATION (consolidated  preserve this text in the run packet)

AR63 RESOLVED per D-007: Advanced-edition features require explicit per-feature human exception. Granted for Phase 5: case-file compare (read-only), find & replace, spreadsheet/CSV import & export. Lookup-table tooling completion is Standard scope. Your confirmation that SPEC-013 ch. 19 documents no merge/apply-changes direction is accepted.

SCOPE:
a. Find & replace: per SPEC-013 9.4.1 and the feature matrix ("one-by-one or en masse")  case-sensitive/insensitive and scope options as the spec documents, across titles/descriptions/properties. Mandatory preview listing every pending change with entity path before apply; apply is one transaction (all-or-nothing) with a post-apply count. No regex mode unless the spec documents one.
b. Case-file compare: open a second .sqlite case READ-ONLY (validated open, never written); produce a structured diff  entities added/removed/changed (title/description/properties/positions), links, matrices/assessments, actions  plus a diff report. COMPARE ONLY: no merge, copy, or apply-changes direction. If you find one documented anywhere, log an AR and stop.
c. CSV/spreadsheet import & export: define an original, documented CSV layout (NOT the legacy format  Rule 2). Export: entities, links, matrices, actions. Import: validate everything FIRST (unknown kinds, bad parents, duplicate IDs, malformed rows reported with line numbers), show a dry-run summary of what would be created, then apply through the existing CaseStore services in a single transaction so schema triggers remain the enforcement layer. A rejected import must leave the case byte-identical.
d. Lookup tables: complete any Standard-edition lookup tooling gaps left from Phase 3.

STANDING RULES (unchanged): no decompilers, no legacy binaries as input, no legacy names/branding, offline-only, parameterized SQL only, schema triggers authoritative, producer  validator.

VALIDATION BAR: new cargo tests for find/replace preview-apply atomicity; import rejection cases (bad parent, cycle attempt, duplicate identity, malformed CSV) with rollback equality; exportimportexport round-trip structural equality. Extend the native GUI script to run a find/replace, a two-case compare, and a CSV round-trip; verify save/reopen equality after each. Commit evidence and hashes to runs/006-build-phase5/.

AMBIGUITIES: resolve per D-006, append to AMBIGUITY-RESOLUTIONS.md continuing from AR64 (AR63 is resolved by D-007).

STOP at the phase boundary with stages/04-build/output/phase-5-report.md. PUSH feature/phase-5-case-tools to origin when done (feature-branch push authorized; prohibition covers main only). No merge, no Phase 6.

## Current human validator authorization — preserved verbatim

Validation sub-agent authorized with these constraints:
1. READ-ONLY: no file modifications, no commits, no schema or test changes. It may run builds, tests, the native GUI scripts, and independent parsers, and may inspect anything in the repo.
2. Fresh context: it must not inherit your implementation narrative  give it the phase requirements and the spec citations, not your conclusions. Its verdict must be its own.
3. Its findings go into runs/006-build-phase5/ as a separate validation record (validator id/context, checks run, PASS/FAIL per item, defects found). Producer and validator evidence stay in separate files.
4. Defects it finds are reported to you for fixing; after fixes, it re-validates. It never fixes anything itself.
5. This does NOT replace the human phase-gate review: you still stop at the Phase 5 boundary with phase-5-report.md and push the branch. Human approval remains the only authority to start Phase 6.
