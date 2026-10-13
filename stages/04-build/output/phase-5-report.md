# Phase 5 report — Barrier Atlas

Phase 5 implements validated read-only case comparison and a structured text diff report, previewed transactional find/replace, original CSV/XLSX case exchange, and Standard lookup ordering/assignment visibility. Run `006-build-phase5`; branch `feature/phase-5-case-tools`. Stop at this boundary for human review of implementation and AR64–AR69. Phase 6 remains unauthorized.

## Authority and starting evidence

Continuation TARGET GATE: PASS on 2026-10-13: sole root `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie`, clean feature branch at `ad08c71c025cd8733e5d3a475fed855b6e717d86`, expected origin `https://github.com/blackTieV2/bowtie.git`, build approved, no execution hold. The branch descends from approved Phase 4 `72e9a1c138b55aedd5e7979c18c4d473a5357b4a`.

Human D-007 confirms the original AR63 stop/citations and grants explicit per-feature Advanced exceptions for read-only comparison, find/replace and spreadsheet/CSV exchange; lookup completion remains Standard. The consolidated authorization is preserved verbatim in the task packet. Bootstrap PROJECT_STATUS next-action/Standard-only text is historical; explicit current human authority governs this bounded continuation. D-007 has human-approved provenance, not inferred memory promotion.

SPEC-013 §§19.1–19.4 documents comparison/reporting with no merge/copy/apply direction found; the human accepted this preflight. No such direction exists in the implementation. The earlier detailed requirements were absent from the saved packet and were supplied during this continuation before the dependent tools were implemented.

## Implemented

| Tool | Behavior |
|---|---|
| Comparison | Current live case (including unsaved edits) → validated second case opened with SQLite READ_ONLY flags in a pinned read transaction. Canonical primary-key records identify added/removed/changed fields: titles/descriptions/properties/positions, links, matrices/assessments, lookups and action entities. Original structured text report; changed inputs reject export; a report cannot overwrite a SQLite case. Second connection is never an editable session. |
| Find/replace | Literal case-sensitive whole-case text; complete field previews with path, before/after and occurrences; individual/selected/all apply in one transaction and post-apply counts. Case/entity/lookup/matrix text and typed text user data; dates/enums remain validated. IDs, geometry and numbers are not string-rewritten. Case/find/replacement-bound signature rejects stale/unreviewed apply; normalized titles are shown before apply. No undocumented case/scope switches or regex. |
| CSV/XLSX | Original documented eight-column layout exports entities/actions, links, matrices/assessments and lookup/custom-field dependencies. UTF-8 quoted CSV/BOM/newlines; selectable XLSX worksheets, shared/inline text, formatting ignored. Formulas require pasted values; no external relationships fetched. |
| Import | Parse/type/identity/hierarchy diagnostics with physical CSV line numbers; validate-first isolated in-memory dry run, creation counts and header-adoption notice. Exact input/current-case signature rechecked before shared CaseStore entity/property/lookup/matrix services apply one active transaction. Authoritative schema triggers retained; rejection preserves session/saved bytes. No automatic save or silent duplicate update/merge. |
| Standard lookups | Existing add/edit/delete/typed custom metadata retained; sparse-order-safe append, atomic up/down ordering and visible assignment counts. Reordering preserves assigned IDs; existing reference-safe deletion remains. |
| Tool undo | Last unchanged unsaved replace/import can be undone as a whole from a validated in-memory checkpoint. Later edits block undo; save/open/new/close clear it. No new schema/persistence/history store. |

Only interchange normalizes organization identity to `@root`. Pristine targets adopt package case/organization details; populated targets require matching organization details/properties/definitions and receive new IDs only. Full exported packages should be imported into new empty cases. `app/CASE-TABLE-FORMAT.md` documents the contract. AR64–AR69 record interpretations under D-006 for human review.

## Changes and invariants

New Rust modules `case_tools.rs`, `compare.rs`, `exchange.rs`; typed IPC/registration and shared transaction writers. New frontend views `FindReplace.tsx`, `CaseCompare.tsx`, `CaseExchange.tsx`, workbook adapter `tableWorkbook.ts`; workspace integration and lookup ordering. Original table documentation, traceability, D-007, ARs, run records, native acceptance scripts, stdlib artifact parser and generated examples/evidence added.

SQLite schema, dependency manifests/locks, spec corpus, governance contracts and accepted Phase 1–4 evidence are unchanged. Runtime remains offline with bundled assets and existing libraries. No legacy binaries/decompilers/source/assets/formats/branding were inputs.

## Producer evidence

| Check | Observed result |
|---|---|
| `cargo test --locked --offline` (installed cargo absolute path) | PASS: 37 Rust tests, 0 failures; library/main/doc harnesses complete. Preview/apply atomicity, query/stale guards, invalid-title rollback, read-only byte/session equality, bad-parent/cycle/identity/kind/CSV/property/header rejection, persisted whitespace and structural round trips, save/reopen and lookup identity preservation. |
| `npm run check`, `npm run build` | PASS; existing large-bundle advisory only. |
| `npm run tauri build -- --no-bundle` | PASS: final Windows release executable; hash in build-evidence.json. No installer packaging. |
| `./scripts/check_phase5_native.ps1` | PASS: twelve-step GUI/Phase 3 regressions; find/replace preview/path/apply/count; two-case comparison/report; CSV/XLSX GUI round trips; rejection byte equality; sparse-order lookup append/reorder. Save/reopen equality after each tool scenario. |
| `python scripts/check_phase5_artifacts.py` | PASS: independent stdlib CSV/ZIP/XML/JSON parsing; 21 entities, one lookup, seven links, one matrix, four assessments; exact CSV/reexport bytes, XLSX/CSV rows and known report change. |
| Screenshot inspection | PASS: readable replacement paths/values, comparison direction/source fields and scrollable import/dry-run controls; evidence images retained. |
| Hygiene | Whitespace, unchanged schema/locks/corpus/accepted evidence and source/output hashes checked. |

Producer results are separate files: producer-validation-results.json, gui-validation-results.json, comparison-gui-results.json, artifact-validation-results.json, regression records, build-evidence.json and runtime-case-hashes.json. Original CSV, identical reexport, XLSX and structured text report examples are in artifacts/. Output hashes cover changed artifacts/evidence.

One initial Rust assertion used a logical record index instead of the retained physical CSV line after multiline data; corrected. One native XLSX assertion compared DOM textarea LF against file CRLF; DOM comparison was normalized while file/structural equality stayed exact. Final suites passed. The first native invocation exited 1 without output; captured rerun exposed the textarea assertion above.

## Independent validation

Human authorized a fresh-context READ-ONLY validator, `/root/phase5_validator`, receiving requirements/spec citations without producer conclusions. It cannot edit tracked files/tests/schema/commits; producer fixes defects. Its own returned record is preserved verbatim in validator-report.md, with isolated generated evidence under validator-evidence/.

It initially found extra case-header validation, imported title whitespace loss and sparse-order lookup append defects. Producer fixed them and added regression coverage; final revalidation and independent PASS/FAIL per item are in the separate validator record. Producer evidence alone is not independent validation or human approval.

Final independent verdict: PASS for tested Phase 5 Windows scope, no outstanding blocking defects. It reran 37 locked/offline Rust tests, frontend checks, full isolated native acceptance and stdlib artifact parsers, plus its own read-only SQLite/CSV value/reference cross-check. Validator-returned `validator-erratum.md` corrects one transcribed source hash; the original report is preserved. All ten final app/executable hashes were independently matched during record preservation.

## Limits and boundary

Windows native execution is verified. macOS/Linux native builds/runs and native Browse-dialog interaction remain unverified host/manual checks; no cross-platform execution claim. XLSX is value interchange, not a calculation engine; 32,767 UTF-16 units per export cell and 20 MiB input/output/expanded workbook limits apply. XLSX diagnostics refer to its converted CSV lines. Future audit/incident exchange, fuzzy matching and general historical undo are outside Phase 5.

D-007 is approved; AR64–AR69 and this phase await human boundary review. Implemented/tested/parsed/independently validated evidence does not constitute human approval, deployment or release. Feature-branch commit/push is authorized, with exact origin ref verification in push-evidence.json. No main merge/push, Phase 6, deployment or release is authorized/performed.
