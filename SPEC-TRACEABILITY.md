# Spec traceability — Barrier Atlas (Phases 1–4)

Run `002-build-phase1`. The human approved schema/scaffold and AR01–AR33 and authorized runtime case IPC/tree work under the selected name Barrier Atlas. Schema checks, runtime service tests and Windows built-app smoke are distinct evidence; none establishes independent QA or deployment. Source filenames expand in `stages/04-build/output/feature-inventory.md`.

| Artifact / represented behavior | Exact spec locator | Status / evidence |
|---|---|---|
| Organization/group hierarchy, hazard/top-event combinations | Master core model; SPEC-013 §§3.4–3.5, 7.2; SPEC-014 §§3.4–3.5; SPEC-019 Guiding Principles | entity parent rules; AR18; schema test |
| Threats, consequences, preventive/recovery barriers | SPEC-013 §§3.6–3.8; SPEC-014 §§3.6–3.8 | typed hierarchy; schema fixture steps 3–6 |
| Escalation factors and their barriers | SPEC-013 §3.9; SPEC-014 §3.9 | typed nesting/cycle checks; steps 7–8 |
| Activities, documents, systems, competencies, job titles | SPEC-013 §§3.11.2–3.11.4, 7.2–7.4, 15.6 | central entity/link schema; steps 9–10 |
| Activity inputs/outputs and verification records | SPEC-013 §7.2; SPEC-004 steps 10–11; provenance Quick Start §3.10.3.4 for explanation absent from primary list | entity/link schema; AR10/19; step 11 |
| One-off improvement actions | SPEC-013 §3.12; SPEC-004 step 12; SPEC-016 §§6.2.4–6.2.5 | action properties/rationale fixture; no ALARP calculation |
| Risk axes/cells/categories and inherent/residual assessment | SPEC-013 §7.12; SPEC-015 GENERAL Risk Matrices | composite keys, eight limit, typed targets; schema tests |
| Configurable lookups and custom metadata | SPEC-013 §§7.3, 7.4.1, 15.2 | lookup/property schema; no runtime editor |
| Tree structure and sibling order | SPEC-013 §§7.2, 7.9 | SQL case_tree through Tauri IPC; expandable/selectable React explorer; native smoke verifies saved/reopened hierarchy |
| Local case data persistence | SPEC-013 §§6.4.1, 6.4.3; master SQLite storage target | rusqlite-backed create/open/save/save-as/close; memory working copy, validated atomic Save; native round-trip tests |
| Unsaved-change prompts and dirty marker | SPEC-013 §§6.2.2, 6.4.3 | Native file controls/window close guard; smoke tests dirty Close case prompt and Cancel |
| Opaque IDs, foreign keys, transactional diagram/top-event creation | Human continuation constraints; SPEC-014 §§3.4–3.5 | UUID v4; configured connections; transaction failure-injection and connection tests |
| Future audit target associations/survey subset | SPEC-021 §§24.3, 24.5–24.8; SPEC-023 audit preparation workflow | reference-contract tables/negative tests only; Phase 6 not implemented |
| Future incident/barrier association | SPEC-021 §29.2; SPEC-002 §10.2; SPEC-022 linking chapter | separate incident identity/join tables; Phase 7 not implemented |
| Offline Barrier Atlas Tauri/React shell and original icons | Fixed stack/standing offline and clean-room rules; human product-name decision; SPEC-013 §6.6 | Windows native build and IPC smoke passed; local dialogs/case commands only; Konva declared but no canvas code |

The Phase 1 evidence above is retained as history. The human approved its runtime boundary and authorized Phase 2.

## Phase 2 interactive canvas — run 003-build-phase2

| Implemented behavior / artifacts | Spec locator | Verification |
|---|---|---|
| Konva shapes/paths and classic auto-layout | SPEC-013 §§3.5–3.9, 8.2.4 | DiagramCanvas.tsx/model.ts; twelve-step structure and pointer test |
| Add/edit/delete typed entities; empty diagram deletion as a unit | SPEC-013 §§3.4–3.12, 7.4 | CaseStore editing.rs, EntityEditor.tsx; trigger/RESTRICT tests and GUI edit/delete |
| Persist drag coordinates and explicit auto-layout via IPC/SQLite | SPEC-013 §§3.8–3.9, 8.2.4; human coordinate requirement | Atomic diagram-scoped position batch; pointer drag and save/reopen |
| Fit/100%/zoom, Ctrl-wheel pointer zoom; wheel/Shift-wheel, background-drag and arrows pan | SPEC-013 §7.5 | DiagramCanvas.tsx; built-app pointer assertions; AR42 |
| Central activity/document/job title/product/system/competency editing, typed links/backlinks/unlink | SPEC-013 §§3.11.2–3.11.4, 7.2, 7.4.2 | EntityEditor/LinkEditor; SQLite properties/link snapshot; GUI steps 9–11 |
| Recurring task timing/instructions/responsible role, verification/output evidence | SPEC-004 steps 9–11; SPEC-013 §§3.11, 7.2 | GUI walkthrough activity, responsibility/document/output links; save/reopen |
| One-off action and manually entered improvement/ALARP rationale | SPEC-004 step 12; SPEC-013 §§3.12.1–3.12.4 | GUI action/hazard rationale; no automatic ALARP or Phase 3 register |
| Named RESTRICT blockers, explicit owned-property cleanup, shared-item preservation | SPEC-013 §§3.11.2–3.11.3.3; human RESTRICT requirement | Rust rollback/named-reference tests; GUI blocked/successful deletion |

Source filenames are indexed in stages/01-intake/references/spec/README.md. Actual outcomes are recorded in the Phase 2 report/run packet. Producer validation is not independent QA; native chooser UX and macOS/Linux execution remain manual/pending.

## Phase 3 properties/risk/register — run 004-build-phase3

The human approved Phase 2/AR37–AR44 and corrected AR45: risk assessments apply to top events and consequences. The SQL schema remains unchanged.

| Implemented behavior / artifacts | Exact spec locator | Verification |
|---|---|---|
| Typed per-kind scalar controls, description, code, taxonomy/reference selectors and manual criteria/sign-off/revision fields | SPEC-013 §§3.11.2, 3.12.4.1, 7.4, 7.13, 7.16, 15.2–15.7 (including unnumbered Fields on Threats) | property-catalog.json + properties.rs + PropertyFields.tsx; detailed phase-3-property-inventory.md; typed/rollback tests |
| Type-wide user-data text/number/boolean definitions, add/rename/delete and values | SPEC-013 §7.4.1 | CustomFields + case_custom_field; entity/taxonomy kind-wide rename/delete and invalid-type tests |
| Central property choices and effectiveness metadata | SPEC-013 §§3.11.2, 7.3, 7.4.1, 15.6 | LookupManager + case_save_lookup/delete_lookup; taxonomy-type/ref/metadata checks |
| Barrier effectiveness FK, annotation and color | SPEC-013 §§3.11.2, 7.5.2, 15.6 | Transactional field/FK synchronization; native GUI assignment; canvas screenshot |
| Named configurable matrices, axes, cells, low-to-high category ranks; maximum eight | SPEC-013 §7.12 | RiskPanels + risk.rs; authoritative trigger-limit and complete-cell validation tests |
| Soft-hide by clearing displayed name, restore and retained assessments | SPEC-013 §7.12; current human clarification | AR46 metadata representation; Rust/native GUI preservation tests |
| Horizontal likelihood / vertical severity, custom codes/labels, XY/YX display | SPEC-013 §7.12 | Matrix editor/YX coordinate helper; Rust/native GUI and editor screenshot |
| Category selection/cell painting, cell labels, ordered category updates | SPEC-013 §7.12 | Direct cell model, rank upserts and duplicate-ID rejection; native authoring |
| Top-event/consequence inherent/residual assessment create/update/delete | SPEC-013 §7.12; human AR45 correction | Existing target/cell triggers; Rust CRUD/rejection and GUI four-rating workflow |
| Clear all ratings with confirmation, including hidden matrices; referenced-cell resize/delete restrictions | SPEC-013 §7.12; current human clarification; approved RESTRICT contract | Global transactional clear, GUI cancel/confirm, Rust hidden/resize preservation |
| Case-wide action register, all documented action fields, typed roles, inline create/edit and sort/filter | SPEC-013 §§3.12.4.1, 7.9 | SQLite-derived action rows; existing entity IPC; Rust register/tree equality and native inline/filter tests |
| Ordinal category-rank sorting per selected matrix/phase | SPEC-013 §7.12; current human clarification | AR49 contextual parent assessment; native low-before-high register check; properties rank order |
| Manual ALARP rationale and local references | SPEC-004 step 12; accepted AR20/AR43 | Typed free-text rationale retained; no automatic ALARP computation/display, network fetch or service |

The native Phase 3 script first reruns all twelve GUI method steps, then authors the above Phase 3 example and verifies tree/links/matrices/assessments/lookups/actions/custom-definition equality after Save/Open. Final results, commands, screenshots and gaps are in phase-3-report.md and run 004. Advanced/server/audit behavior remains in its assigned scope; metadata fields do not enable those modules.

## Phase 4 offline reporting — run 005-build-phase4

| Implemented behavior / artifact | Exact spec locator | Verification |
|---|---|---|
| Whole-diagram PNG export with 1–4 size multiplier, persisted layout and annotations | SPEC-013 §§7.5.6, 13.4; current human scope | Shared diagramScene; native GUI export; PNG dimensions and saved-position check |
| Vector SVG and text/vector PDF replacing proprietary output formats | SPEC-013 §§13.4, 13.6.1.1–13.6.1.2; explicit SVG/PDF authority | SVG text/paths/embedded font, PDF text/vector primitives; XML and PDF parser/render checks |
| Case/group/hazard/diagram summary and threat/consequence/barrier/escalation hierarchy | SPEC-013 §13.2.2 Hazards/Register/BowTie Diagrams/Threats and Consequences; SPEC-006 management communication context | Original caseReport OOXML layout from live snapshot; known case strings and Word rendering |
| Risk assessments, axes/coordinates/cells/categories and matrix context | SPEC-013 §13.2.2 Risk Assessment reports; §7.12 | Named matrix/rating sections; rank ordering; hidden matrices omitted as documented |
| Barrier register/effectiveness, definitions and recorded metadata | SPEC-013 §13.2.2 Barriers/Barrier overview/Color Legends; §§15.6, 13.5 | Original barrier tables and effectiveness context; generated XML/render review |
| All actions with parent, party, priority/category, dates/completion/sign-off/rationale | SPEC-013 §13.2.2 Actions/All Actions/Actions in context; §3.12.4.1 | Live SQLite-derived action rows/properties/links; known new action and role in DOCX |
| Linked activities/documents/systems/competencies/job titles/products and assignments | SPEC-013 §13.2.2 Activities/Document Links/Job Titles/Lists/Systems; §7.2 | Supporting-item detail and relationship tables; no remote document fetching |
| Read-only signed export snapshot and atomic destination writing | Current no-new-persistence/live-case authority; SPEC-013 §§13.2.1, 13.4 | Three new Rust tests; unchanged snapshots/schema after native export |
| Stable serialization/container dates, no generation timestamps, two-run hashes | Current deterministic-output requirement | Fresh-frontend repeat and independent SHA-256 comparisons for PNG/SVG/PDF/DOCX; AR59 qualifies platform scope |
| Original offline font/layout/branding | Standing clean-room/offline authority; SPEC-013 §7.5.3 | Bundled OFL font, local font loading/embedding, no legacy templates/assets or network services |

This is the requested consolidated core case report, not a claim to replicate every historical report variant or later module. Content/schema review found no gap requiring a migration. Native chooser, cross-platform raster/Office layout and independent QA limits appear in phase-4-report.md. App-generated artifacts are separate from Word/PDF raster QA intermediates.

# Phase 5 continuation — D-007

| Feature | Spec authority | Implementation | Evidence |
|---|---|---|---|
| Validated read-only case comparison, added/removed/changed field view, textual diff report | SPEC-013 §§19.1, 19.2.2, 19.3; SPEC-015 case comparison Advanced row; human D-007 per-feature exception | `app/src-tauri/src/compare.rs`, `app/src/CaseCompare.tsx`, typed CaseStore IPC | `compare::tests`; native reference-byte/session equality and save/reopen; original parsed diff report |
| Text find/replace preview with entity paths, individual/selected/all fields, transaction and count | SPEC-013 §§9.4–9.4.1; SPEC-015 Find & Replace one-by-one/en masse; D-007 | `case_tools.rs`, `FindReplace.tsx`, shared typed property/lookup/matrix writers | Atomic failure rollback, unchanged preview, stale/query-bound preview, individual/all tests; native preview/apply/save/reopen |
| Original CSV/XLSX export of entities/actions, links, matrices and their dependencies | SPEC-013 §§12.3,19.3; current original-layout authority; D-007 | `exchange.rs`, `tableWorkbook.ts`, `CaseExchange.tsx`, `app/CASE-TABLE-FORMAT.md` | Native CSV/XLSX round trips, repeat workbook bytes, stdlib CSV/ZIP/XML/JSON parser equality |
| Validate-first import, line diagnostics, dry-run counts, one CaseStore transaction with triggers | SPEC-013 §12.3; current strict duplicate rejection and transactional import authority; D-007 | Shared entity/property/lookup/matrix writers; isolated scratch preview and revalidated active apply | Bad parent/cycle/identity/kind/CSV/property/header rejection; session/saved-byte equality; structural round-trip and save/reopen |
| Standard lookup add/edit/delete/order and assignment visibility | SPEC-013 §7.3; SPEC-015 Standard Lookup Tables Basic | `LookupManager.tsx`, `risk.rs`, existing Phase 3 validators | Sparse-order append native check, atomic move test, unchanged assigned entity identities and save/reopen |
| Whole-operation session undo for replacement/import | SPEC-013 §§9.4.1,12.3 | Validated bounded in-memory tool checkpoint in `case_tools.rs` | Replace undo equality; stale guard and save/open/new/close invalidation; no schema additions |

Comparison uses the application's original schema and stored primary-key identities. Semantic JSON comparison avoids whitespace/key-order false differences. Report export revalidates both case contents against the preview signature. No comparison merge/copy/apply path exists. AR64–AR69 document the bounded interpretations for human phase review. All Phase 5 examples and results are in `runs/006-build-phase5/`.

## Phase 6 audit module — run 007-build-phase6

| Feature | Spec authority | Implementation and verification |
|---|---|---|
| Multiple normal audits, question hierarchy, criteria and ordered choices | SPEC-021 §§24.3–24.6.1; SPEC-023 Preparation: Define questions | audit.rs/AuditPanel; hierarchy/cycle tests and native preparation |
| Positive/negative keying, midpoint and explicit @ center | SPEC-021 §24.6.1; SPEC-023 Preparation: Define questions | Ordinal tests: eight positive, eight neutral, balanced four/four; no numerical score |
| Most-negative and contextual/shared responses | SPEC-021 §24.6.2 | Typed modes/exact slots; Rust/native worst-choice checks |
| Barrier/activity/document links and question labels beneath barriers | SPEC-021 §§24.3,24.7; SPEC-023 Preparation: Link questions; approved target trigger | Relational writer, AuditDiagram/scene; target-kind tests and canvas |
| Flat survey subset, code/name/description, combined target/role/property criteria | SPEC-021 §24.8; SPEC-023 Preparation: Create surveys | AuditPanel/ordered membership; survey CRUD/native subset |
| Respondent/date/unit, answers/remarks, drafts/completion/edit/delete and persistence | SPEC-021 §§24.4,24.9; SPEC-023 Conduct the audit | Typed audit-owned metadata, transactional checks/full reopen equality |
| Original XLSX dropdown forms and validated multi-form import | SPEC-021 §24.9.1; SPEC-023 Conduct the audit: Export/import surveys | auditWorkbook; styled Excel-date fixture, invalid-choice rollback and ZIP/XML checks |
| Qualitative counts/details and survey/unit/date/fillout/negative filters | SPEC-021 §§24.10–24.10.2; SPEC-023 Review the audit: Analyse results | Rust count/filter tests, editor/target review; no invented weighted/risk formula |
| Diagram filter, supporting context and graph-click answer dialog | SPEC-021 §24.10; SPEC-023 Review the audit: Display results | AuditDiagram/DiagramCanvas; native details screenshot |
| Saved review/diagram profiles | SPEC-023 Review the audit: Display results (save filter settings) | Validated audit-owned profiles, deletion/reference guards, native/Rust reopen |
| Audit-only and integrated case DOCX review | SPEC-023 Review the audit: Review & Report; SPEC-021 §24.10; Phase 4 report authority | Existing caseReport/signed atomic exports, native artifact/parser checks |
| Full audit preservation in original core exchange | SPEC-021 §24.4 all audit data in case; standing exchange consistency | exchange audit bundle; Rust/native export/import/export equality |

Storage contract: app/AUDIT-DATA-DESIGN.md. Approved schema/trigger files remain unchanged. AR70 onward records boundary interpretations; run 007 separates producer/validator evidence. Server-only surveys and incident audits remain outside Phase 6.

## Phase 7 constrained migration — AR78 resolved by human authorization

SPEC-002 §§7.1.1,8.1.2,ch.10 requires ordinary/categorized RCA events to link to case barriers. schema/002-rca-event-links.sql replaces ONLY the two link triggers and sets user_version=2. Source is a barrier of any method or an event whose owning incident method_key is rca/categorized_rca; target remains a case barrier. Identity/FKs unchanged. case.rs accepts exact v1/v2 signatures, migrates the working-copy backup transactionally on open, and persists on explicit atomic Save. migration_tests covers v1 file roundtrip, all source kinds/methods, insert/update and schema-change scope; original 43 tests unchanged.
