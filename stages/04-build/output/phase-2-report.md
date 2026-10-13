# Phase 2 review report — Barrier Atlas

Phase 2 implementation and Windows producer validation are ready for review. Stop boundary: **Phase 3 has not started**. Run `003-build-phase2`; branch `feature/phase-2-diagram-canvas`, created from `feature/phase-1-data-model` at `21fe11430b8a8e02073100f2e942c60d75c8ae20`.

Implementation commit: `b78d18f` — `feat(canvas): implement Phase 2 diagram editing and persistence`. The report, traceability, interpretations and run evidence are a separate documentation commit.

## Authority and target

The human approved Phase 1 completion, its Windows runtime evidence, and AR34–AR36, then authorized Phase 2 only. Before modifications, recon confirmed the sole repository root `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie`, a clean tree, the Phase 1 branch/HEAD, and origin `https://github.com/blackTieV2/bowtie.git`. **TARGET GATE: PASS**. No execution hold is active. The current human instruction supersedes the profile's older Phase 1 next action.

## What was built

An interactive React/Konva canvas renders the hazard above the center top event, threats on the left, consequences on the right, barriers along their connected paths, and escalation factors below the barriers they affect. Connectors update while dragging. The canvas supports selection, double-click editing, persistent drag coordinates, explicit classic Auto-layout, Fit, 100%, zoom buttons, background-drag pan, wheel/Shift-wheel pan, Ctrl-wheel pointer zoom, and arrow-key pan. Canvas dimensions follow the available viewport.

The context toolbar and entity dialogs create/edit/delete threats, consequences, barriers, escalation factors and one-off actions. The existing explorer/group/diagram controls create structural roots. Supporting-item controls create/edit activities, documents, systems, competencies, job titles and products; activities/documents/systems can be nested. The selected item's link editor assigns typed links, shows incoming references, and explicitly unlinks shared items. Canvas annotations show linked items and actions.

Task timing, instructions, responsibility, document references, verification/output records and manually entered improvement/ALARP rationale complete the twelve method steps. These are basic entity/link fields; risk matrices, effectiveness configuration and the action register await their later phase.

All editing extends **CaseStore IPC**. Parameterized rusqlite writes retain the approved schema, identity/parent/link triggers, UUID v4 runtime IDs and `foreign_keys=ON` connection setup. Coordinate batches and edits are transactional. Positions use the existing generic property table; no schema migration occurred. Failed edits/deletes preserve the working case. Save retains Phase 1's validated atomic backup and unsaved-work semantics.

Deletion explains actual blocking child names, incoming/outgoing relationship names and referenced item titles. It also identifies risk/audit/incident reference table counts if present. Shared references never cascade. Owned property rows are explicitly removed when an unreferenced entity is deleted. An empty diagram and its sole top event are deleted together transactionally; deleting the organization or top event alone is blocked to preserve the valid case invariant.

## Files changed

| Files | Purpose |
|---|---|
| `app/src/model.ts`, `app/src/DiagramCanvas.tsx` | Typed snapshots, deterministic layout, Konva shapes/connectors, navigation and drag persistence |
| `app/src/EntityEditor.tsx`, `app/src/App.tsx`, `app/src/style.css` | Entity/link editing, context controls, explorer integration, dialogs and canvas styling |
| `app/src-tauri/src/editing.rs` | Transactional entity/property/link/position service, deletion diagnostics and five new Rust tests |
| `app/src-tauri/src/case.rs`, `commands.rs`, `lib.rs` | Snapshot properties/links, five typed editing commands and registration |
| `scripts/check_phase2_native.mjs`, `scripts/check_phase2_native.ps1` | Reproducible built-app GUI walkthrough, pointer interactions, save/reopen and owned-process cleanup |
| `SPEC-TRACEABILITY.md`, `stages/04-build/output/AMBIGUITY-RESOLUTIONS.md` | Phase 2 feature citations and AR37–AR44 review candidates |
| `stages/04-build/output/phase-2-report.md`, `runs/003-build-phase2/` | Review report, packet/state, checksums, validation/build evidence and canvas screenshot |

The approved `001-initial.sql`, source corpus, dependency locks and canonical governance files are unchanged.

## Validation evidence

| Check | Observed result |
|---|---|
| `npm run check` / `npm run build` in `app/` | PASS: TypeScript and Vite production bundle |
| `cargo test --offline` in `app/src-tauri/` | PASS: 13 tests, zero failures (eight Phase 1 + five Phase 2) |
| `npm run tauri build -- --no-bundle` in `app/` | PASS: Windows optimized native executable; Rust release build reported 1m 05s |
| `./scripts/check_phase2_native.ps1` from repository root | PASS: twelve GUI-authored steps, 20 entities, five typed links, original case save/reopen equality |
| Canvas pointer/geometry assertions | PASS: threats left/consequences right, path barrier placement, escalation factor below its barrier, dragged coordinates persisted, toolbar and Ctrl-wheel zoom, background/wheel pan |
| GUI edit/delete and RESTRICT | PASS: edit persisted, temporary leaf removed, blocked threat/task deletion names real child/incoming activity reference |
| Native save/reopen | PASS: complete tree (including properties/coordinates) and links equal; clean reopened case; GUI Save on established path and Close case |
| Source scan and `git diff --check` | PASS: no legacy product names in runtime/source/new test files, no whitespace errors; approved SQL has no diff |

The test launches only this project's newly built executable with an isolated WebView2 profile and local diagnostic port. This port is a test-run setting; the shipping app retains its offline CSP and has no network service dependency. All entity/property/link authoring uses rendered GUI controls. Test IPC supplies its own Save/Open paths and reads snapshots for assertions. The first test attempt selected the diagram container instead of the identically titled top event; that selector was corrected, and the full script subsequently passed, including the strengthened geometry/wheel checks.

Committed evidence: `runs/003-build-phase2/validation-results.json`, `build-evidence.json`, `input-hashes.json`, `output-hashes.json`, and `evidence/canvas.png`. The generated original case and diagnostic profile are ignored under `.runtime/phase2-walkthrough/` and `.runtime/phase2-webview/`. Tests are producer verification, not independent QA or human acceptance.

## Twelve-step example and manual reproduction

SPEC-004 is a method-question walkthrough without a named example dataset. AR37 uses original liquid-transfer content. Start New case (Transfer safety review / Example workshop), Add group (Transfer station), then Add diagram. Select a shape or explorer item to use its context controls; supporting items and assigned links are edited below the canvas.

| Step | GUI-authored example / resulting structure |
|---|---|
| 1–2 | Hazard Flammable liquid transfer; top event Loss of containment |
| 3 | Threats Hose rupture and Tank overfill during transfer |
| 4 | Consequences Pool fire and Environmental release |
| 5 | Preventive barriers Pre-transfer hose inspection and High-level trip |
| 6 | Recovery barriers Emergency isolation and Bund containment |
| 7–8 | Poor lighting below inspection; its barrier Portable inspection light |
| 9 | Activity Inspect transfer hose linked to inspection barrier |
| 10 | Timing/instructions on activity; responsible Transfer operator; Hose condition checklist document link; operator accountable for inspection barrier |
| 11 | Verification method on activity; output Signed hose inspection record |
| 12 | Improve inspection lighting action under inspection barrier, and practical-improvement rationale on action/hazard |

Click Auto-layout and Fit; drag a barrier; Save; Close case; Open the saved `.sqlite` case. Confirm the example hierarchy, five links and moved barrier coordinates. The script performs equivalent authoring plus edit/deletion/blocker checks and verifies persistence programmatically.

Manual/pending checks: native Open/Save As chooser path selection and cancellation; keyboard/Shift-wheel/touch navigation and double-click editor ergonomics; human assessment of methodology and ALARP rationale; macOS/Linux native builds/runs; independent QA. Vite reported a 583.86 kB minified JavaScript chunk warning after adding Konva; no build failure occurred. Large-diagram performance is not certified by this small walkthrough.

## Ambiguities and next review

AR37–AR44 in `AMBIGUITY-RESOLUTIONS.md` record the chosen original fixture, entity-based position storage (including shared-hazard coordinates), coordinate drag versus sibling ordering, safe deletion/invariants, minimum task/link fields, navigation inputs/bounds, local document references, and the test/manual boundary, each with alternatives and spec evidence. D-006 authorized these interpretations during implementation; human boundary acceptance remains pending.

Review the canvas behavior, deletion policy, AR37–AR44 and documented manual/platform gaps. **Explicit human approval and Phase 3 authorization are required before starting Phase 3.** No merge, push, deployment or release was performed.

Framework API references consulted: [Konva React drag events](https://konvajs.org/docs/react/Drag_And_Drop.html) and [pointer-relative zoom](https://konvajs.org/docs/sandbox/Zooming_Relative_To_Pointer.html). Behavioral authority remains the repository spec sections in `SPEC-TRACEABILITY.md`.
