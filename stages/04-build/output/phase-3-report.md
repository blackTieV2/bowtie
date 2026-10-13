# Phase 3 review report — Barrier Atlas

Phase 3 implementation is ready for boundary review, with producer evidence recorded in run `004-build-phase3`. Branch `feature/phase-3-properties-risk`. **Stop before Phase 4.** Human acceptance of this implementation and AR46–AR54 remains pending.

Implementation commit: `834071c` — `feat(properties): add typed panels, risk matrices and action register`. Review records/evidence are committed separately.

## Authority and target

The human approved Phase 2 and AR37–AR44, then authorized Phase 3 only. Initial recon confirmed the sole root `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie`, a clean Phase 2 branch at `1fc5d75a91d25ac031cc47e644b1c9b4766461d7`, and expected origin `https://github.com/blackTieV2/bowtie.git`. TARGET GATE: PASS preceded branch/packet creation.

The first Phase 3 turn correctly stopped at AR45 because the request said threats/consequences while SPEC-013 §7.12 and both approved triggers allow top events/consequences. The human explicitly corrected that reviewer error, accepted Option 1, and authorized continuation without migration. Renewed recon verified a clean Phase 3 branch at `7e8e2d1992726502dd4709a95bdaf8223ea4f025`; **TARGET GATE: PASS** before continued changes. The historical proposal remains in risk-target-review.md with its resolved status.

Current human scope supersedes the profile's older Phase 1 next action. The human additionally confirmed soft-hide, global clearing, XY/YX and custom axis labels, low-to-high category rank sorting, and direct cell editing. No migration, trigger bypass or automatic ALARP behavior was authorized or implemented.

## What was built

**Typed properties.** The entity editor exposes spec-cited common and per-kind controls: text/descriptions, checkbox criteria/completion, validated dates, integer system thresholds, taxonomy selectors, typed equivalent top-event references, manual sign-off/revision/rationale fields, and relationship selections. A shared property catalog feeds React controls and Rust validation. A complete control/storage/spec mapping is in phase-3-property-inventory.md. Shared hazard sign-off/revision fields are accessible from the top event through Edit hazard details. Documents/attachments/Advanced/server-associated fields are stored as local reference metadata; this does not enable server, SOOB, bundling or audit execution.

User-data fields can be added, renamed and deleted for an entire entity kind or taxonomy table. Text, number and boolean controls store typed values. Renaming/deletion applies to values on all instances in a transaction. Definitions occupy an existing organization property; values remain in entity_property or lookup metadata. No raw JSON editor or executable formula is exposed.

**Risk matrices.** The case-level matrix manager supports named matrices, editable likelihood/frequency horizontal and severity vertical axes, custom codes/labels, XY/YX coordinates, ordered categories with names/colors, category selection and cell painting, cell labels and dimension changes. The eight-matrix trigger remains authoritative. Category ranks are unique and contiguous from low to high; existing matrix rows are updated transactionally without discarding retained assessments. Removing assessed cells is blocked by RESTRICT with an explanatory message.

Clearing an existing displayed name sets its hidden metadata while retaining the nonblank historical database title required by the approved schema. Hidden matrices disappear from rating and register pickers; assessments remain stored and the management view can restore the name. Clear all risk assessments confirms removal of all inherent/residual rows, including hidden matrices, while preserving definitions.

Only top events and consequences receive inherent/residual assessments through risk_assessment. The panel supports assignment, update and removal; displays coordinate/category/rank; and orders ratings by category rank within each matrix. No threat assessment target was added.

**Effectiveness.** Property choices include configurable taxonomies and effectiveness code/name/color/order/description, optional non-negative frequency metadata, and typed custom data. Selecting an effectiveness value updates the barrier property and authoritative entity.effectiveness_id FK in one transaction; its title/color appear on the canvas and in properties. Referenced choices cannot be deleted until unassigned.

**Action register.** A case-wide table queries action entities and their properties/links from SQLite. It includes code, name, description, parent, category, responsible party, target date, completed status, priority, external reference, sign-off authority/date/comment and rationale. Filters and header sorting operate on those rows. The documented Completed boolean supplies Open/Complete; no additional workflow states were invented. Creation and expanded inline editing reuse the same case_add_entity/case_edit_entity IPC, including transactional role assignments.

Register risk context is derived from the nearest assessed ancestor of an action's parent for the selected matrix/phase. Sorting uses that matrix's category rank, not axis multiplication or a cross-matrix universal score. Action records remain tree entities; no register table, schema view migration or duplicate store exists.

All mutation paths use CaseStore, parameterized values, UUID runtime identities, enabled foreign keys and the existing triggers. The approved `001-initial.sql` and format version are unchanged. Phase 1 atomic Save and dirty-session semantics are preserved; ALARP rationale remains manual text.

## Files changed

| Files | Purpose |
|---|---|
| `app/src/property-catalog.json`, `PropertyFields.tsx` | Per-kind typed field/spec catalog, relationship controls, type-wide custom fields |
| `app/src/EntityEditor.tsx`, `App.tsx`, `model.ts`, `style.css` | Modal/inline editors, workspace views, typed snapshot additions, display/layout |
| `app/src/RiskPanels.tsx`, `LookupManager.tsx`, `ActionRegister.tsx` | Matrix/assessment editing, taxonomy/effectiveness configuration, derived action table |
| `app/src/DiagramCanvas.tsx` | Effectiveness annotation/color and action-code display |
| `app/src-tauri/src/properties.rs`, `risk.rs`, `phase3_tests.rs` | Typed properties/custom definitions, transactional matrix/assessment/lookup service, register query and nine Phase 3 tests |
| `app/src-tauri/src/case.rs`, `editing.rs`, `commands.rs`, `lib.rs` | Extended snapshots/IPC, shared validation, property-reference deletion diagnostics |
| `scripts/check_phase3_native.mjs`, `check_phase3_native.ps1` | Phase 3 built-app GUI authoring, pointer regression, persistence equality and cleanup |
| `scripts/check_phase2_native.mjs` | Optional output-path overrides so Phase 3 regression does not overwrite accepted run 003 evidence |
| `SPEC-TRACEABILITY.md`, stage Phase 3 inventory/report, `AMBIGUITY-RESOLUTIONS.md`, `runs/004-build-phase3/` | Feature/spec mapping, review candidates, authority, results, hashes and screenshots |

Source corpus, approved SQL, dependency locks and canonical governance files remain unchanged. The accepted run 003 packet/evidence was preserved.

## Validation evidence

| Check | Observed result |
|---|---|
| TypeScript/Vite via `npm run check` and native beforeBuildCommand | PASS; production frontend |
| `cargo test --offline` in `app/src-tauri/` | PASS: 22 tests, zero failures (13 prior-phase + nine Phase 3) |
| Matrix/assessment tests | Eight-limit trigger; CRUD and invalid threat/cell rejection; hiding, XY/YX, ranks, duplicate identity rejection; resize/delete RESTRICT rollback; hidden/global clear |
| Properties/register tests | Typed values and date/link failure rollback; effectiveness FK/ref deletion; entity/taxonomy-wide custom rename/delete; register/tree equality and responsible roles; unchanged-schema round trip |
| `npm run tauri build -- --no-bundle` in `app/` | PASS: final Windows native release build reported 53.05s; executable hash in build-evidence.json |
| `./scripts/check_phase3_native.ps1` from repository root | PASS against final build: full twelve-step native GUI regression, followed by matrix/rating/effectiveness/register authoring; 21 entities, one matrix, four ratings, two actions |
| Save/reopen comparisons | Tree/properties/positions, typed links, matrices/categories/cells, assessments, lookups/metadata, action rows and custom definitions compared for equality |
| Source/hygiene checks | Approved SQL unchanged; no legacy product names in application/new tests; whitespace and artifact hashes checked before commit |

The GUI example extends the original liquid-transfer case: 21 entities, a custom 2×2 Safety assessment matrix, Low/High categories, YX codes, four top-event/consequence ratings, an Effective inspection barrier and two actions. The new action is created/edited in-place with responsible/sign-off links and dates/completion. The test explicitly checks low-before-high register ordering, text/status filtering, hiding preservation, clear cancellation/confirmation and full persistence equality.

Tests launch only the original built app with an isolated local diagnostic port/profile. Entity/property/matrix/assessment/action authoring uses GUI controls. Save/Open chooser paths are supplied through test IPC; Save on an established path and Close case use GUI controls. The first Rust compile surfaced a query lifetime issue, corrected before successful tests. Screenshot review prompted heading/capture improvements and backend checks for duplicate categories/invalid metadata before final verification.

Committed evidence resides in run 004: validation-results.json, build-evidence.json, input/output hashes and evidence/*.png. Generated own-case files and diagnostic profiles stay ignored under `.runtime/phase3-walkthrough/` and `.runtime/phase3-webview/`. Producer checks are not independent QA or human approval.

## Manual/pending checks

Native Open/Save As chooser selection/cancellation, detailed keyboard/touch ergonomics, untested combinations of every per-kind/custom control, large-matrix/large-case performance, macOS/Linux native builds/runs and independent QA remain pending. The native script samples key fields rather than exhaustively exercising every catalog field. The frontend's roughly 612 kB minified chunk produces a Vite size warning; it does not fail the build. Audit execution, server integrations and Advanced modeling remain outside this phase.

To reproduce: build in app/, then run scripts/check_phase3_native.ps1 from the repository root. This first authors the full twelve-step case, then exercises Phase 3 and writes evidence. For manual use: configure Property choices, author a matrix under Risk matrices, select a top event/consequence to rate it, edit a barrier's Effectiveness, and create/edit actions through Action register. Save, close and reopen the case to inspect persistence.

## Ambiguities and review boundary

AR45 is **resolved-by-correction by the human**. AR46–AR54 record the unchanged-schema hiding representation, typed field/link storage, custom definitions, matrix-specific rank/context semantics, resize/clear behavior, action status/register derivation, shared hazard/module metadata boundaries, effectiveness metadata and acceptance evidence limits, with alternatives and spec citations. They remain boundary-review candidates; no shared approved memory was promoted.

Review the implemented panels, matrix/category editing, risk display/order, effectiveness annotation, register behavior, AR46–AR54 and the pending manual/platform evidence. Explicit approval is required before Phase 4 work. No merge, push, Phase 4, deployment or release was performed.
