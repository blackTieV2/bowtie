# Phase 4 review report — Barrier Atlas

Phase 4 implements offline PNG/SVG/PDF diagram exports and an original Word-compatible DOCX case report. Run `005-build-phase4`; branch `feature/phase-4-reporting`, based on approved Phase 3 tip `ac20e1d4d4992316d34ac724cb4734f81bd62e9a`. Stop before Phase 5. Boundary approval and AR55–AR62 review remain human decisions.

## Authority and target

The human approved Phase 3, AR46–AR54 and the AR45 correction, then explicitly authorized Phase 4 and its feature-branch push. Recon confirmed the sole repository root `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie`, clean `feature/phase-3-properties-risk` at `ac20e1d`, and origin `https://github.com/blackTieV2/bowtie.git`. **TARGET GATE: PASS** before branch/packet creation or code changes. The current authorization supersedes the status profile's historical Phase 1 next action; no execution hold is active.

The requested core report content is available in existing entity/property/link, lookup, matrix/assessment and action snapshots. No missing-schema content was identified for the scoped report. No schema migration or new case persistence was introduced.

## What was built

The Export dialog offers PNG, SVG and PDF for the selected diagram, plus a DOCX report for the open case. Users can choose a destination with Browse or enter an absolute path. PNG supports the documented 1–4 size multiplier and defaults to 4×. All exports cover full diagram bounds and preserve stored positions, colors, path geometry, text wrapping and supporting/action annotations; viewport pan/zoom and selection adornments are excluded. Shared scene code feeds the interactive canvas and output renderers.

PNG is rasterized locally through Konva. SVG contains vector shapes, paths and text with an embedded font, not a bitmap wrapper. PDF uses vector shapes and selectable/extractable embedded-font text. Its single custom page is sized to the diagram at 0.9 points per diagram unit, keeping body labels about 11.7 pt and secondary annotations about 9 pt at native page size. Large diagrams therefore produce poster-sized pages rather than an automatically unreadable A4 reduction. PNG's pixel budget limits pathological allocations; users can lower its scale or choose vector output.

The DOCX uses original black headings, readable body/table text, explicit light-gray table borders, repeating headers, expandable rows and a self-authored OOXML layout. It includes case/organization/group counts and context, hazards and diagrams, threat/consequence/barrier/escalation structure and properties, visible risk matrices/axes/cells/categories/rank-ordered assessments, barrier effectiveness/definitions, action details/status/party/sign-off, and linked supporting items and relationships. Complete values appear in report tables even where canvas labels are ellipsized. Hidden matrices remain omitted from report sections per §7.12. Rationale and user-entered dates remain manual recorded data, without automatic ALARP conclusions.

Read-only CaseStore export context returns a current snapshot and SHA-256 signature. Before writing, CaseStore checks that signature against live state, validates format/absolute extension/size, and atomically replaces the destination through a synchronized temporary file. Export never changes case rows, dirty status or saved case bytes. SQL remains in the existing parameterized snapshot services; schema triggers remain untouched.

Noto Sans is bundled from its official open-source repository with OFL license and hash. The application loads local font bytes; SVG/PDF/DOCX embed them. Runtime export needs no network, font service, Python, Word or LibreOffice. PDF-lib/fontkit/fflate are pinned frontend dependencies; sha2 is a pinned Rust dependency for snapshot signatures. Release CSP adds data fonts while retaining connect-src none.

## Files changed

| Files | Purpose |
|---|---|
| `app/src/diagramScene.ts`, `fontAssets.ts`, `diagramExports.ts`, `exportXml.ts` | Shared layout/text, local font, PNG raster and vector SVG/PDF serializers |
| `app/src/caseReport.ts`, `ExportDialog.tsx` | Original deterministic OOXML case report and export workflow |
| `app/src/DiagramCanvas.tsx`, `App.tsx`, `main.tsx` | Shared-scene canvas, Export control and local font readiness |
| `app/src/assets/` | Unmodified Noto Sans, OFL license and provenance/hash |
| `app/src-tauri/src/export.rs`, `commands.rs`, `lib.rs` | Signed read-only context, export-only atomic writer, IPC and three Rust tests |
| Frontend/Rust manifests and locks; `tauri.conf.json` | Pinned export dependencies and offline data-font policy |
| `scripts/check_phase4_native.*`, `check_phase4_artifacts.py`, `render_phase4_*` | Native GUI/two-run tests, independent format parsers and Word/PDF visual QA |
| `scripts/check_phase3_native.mjs` | Optional run-output override, preserving approved run 004 during regression |
| `SPEC-TRACEABILITY.md`, ambiguity log, this report, `runs/005-build-phase4/` | Spec mapping, review candidates, evidence/artifacts/hashes |

Approved SQL, source corpus, governance and accepted run 003/004 evidence are unchanged.

## Validation evidence

| Check | Observed result |
|---|---|
| TypeScript/Vite build | PASS; export code split into local bundles; size warning documented |
| `cargo test --offline` | PASS: 25 tests, zero failures; stable signature, stale rejection, payload/path checks, atomic replacement and unchanged case state |
| `npm run tauri build -- --no-bundle` | PASS: final Windows native build completed in 1m 13s; executable hash in build-evidence.json |
| `./scripts/check_phase4_native.ps1` | PASS: full twelve-step and Phase 3 native regression, then all four formats exported through GUI twice |
| `./scripts/check_phase4_native.ps1 -ExportOnly` after report refinements | PASS: final fresh-frontend exports from the saved walkthrough; gui-validation-results.json |
| `python scripts/check_phase4_artifacts.py` | PASS: PNG 6680×2112, vector SVG XML, one PDF page with known extractable text, valid DOCX ZIP/XML and known case/group/action/effectiveness strings |
| Repeated output hashes | Two-run byte equality for all four formats; same input case, fresh frontend and different viewport zoom |
| Case/position checks | Full snapshot unchanged by exports; SVG's dragged barrier coordinates equal the stored position |
| PDF/DOCX visual rendering | PASS: all 1 diagram and 7 DOCX-render pages inspected; render-results.json records renderer and visual checks |
| Hygiene and hashes | Schema/accepted evidence unchanged; no legacy product names in application/new export source; whitespace and artifact hashes checked |

Artifacts for both runs live in `runs/005-build-phase4/artifacts/run-1/` and `run-2/`; their hashes and parser checks are recorded separately from the app renderer. The final output examples cover the original 21-entity walkthrough case, one 2×2 matrix, four ratings, effectiveness and two actions. Test diagnostics use only the newly built original app on an isolated local port/profile. Native Browse selection remains manual; automated tests use the visible destination field and Export file button, not direct writer calls.

The packaged render_docx.py was attempted and failed because LibreOffice soffice.exe is absent on this Windows host. An owned hidden Microsoft Word instance opened only the generated report read-only and exported a QA PDF; PyMuPDF rasterized that PDF and the app's diagram PDF for inspection. These are QA-only dependencies/intermediates, not application requirements. Initial visual inspection found split type/header text and orphan table headings; columns, header retention and spacing were corrected and outputs regenerated. Export-only testing also exposed a missing startup wait in the diagnostic harness, corrected before its final pass.

## Determinism and practical limits

Stable snapshot ordering, shared measured wrapping and fixed font bytes remove ordinary content/font variability. PDF generation disables automatic creation/modification metadata. DOCX has no generated-date properties or current-time body content; ZIP entries carry the unavoidable fixed DOS epoch of 1980-01-01. User-entered action/sign-off dates remain report content. Two-run hashes verify deterministic artifact bytes on the tested build/host, not every future renderer/version/OS.

Browser PNG rasterization and locale collation may differ across platforms or versions. Word/LibreOffice pagination and QA PDF timestamps are separate from the deterministic DOCX package; the temporary Word-rendered PDF is not the user-exported diagram PDF. Font coverage is that of bundled Noto Sans; broad script coverage and unusual/very large cases are not certified. The frontend's roughly 2.1 MB entry bundle (including font/fontkit/canvas) triggers a size warning; native build succeeds. Large PNGs can exceed the 100-million-pixel or 100 MiB export guard and should use a lower multiplier or vectors.

## Manual/pending checks and review boundary

Native Browse save/overwrite/cancellation ergonomics, physical poster printing, large diagrams/long reports, broad character coverage, macOS/Linux native execution and cross-platform raster/Office-reader fidelity remain pending. Independent QA and human approval remain distinct from the producer's parser/visual checks.

AR55–AR62 document consolidated report scope, vector replacements/full bounds, shared font/wrapping, print sizing/resource limits, deterministic containers/platform qualification, signed live snapshots, hidden-matrix/report treatment and validation boundaries, with alternatives and citations. Review the exports, report layout/content, limits and these candidates. No legacy templates/assets/binaries, decompilers or external runtime services were used.

Feature-branch push is explicitly authorized and is recorded with the completion evidence. No merge/main push, Phase 5, deployment or release is authorized/performed. Stop for Phase 4 review.

Implementation commit: `88c3867054ccf8dd12d258a8de691f5d1079609a`. Initial feature-branch push succeeded (push-evidence.json); the completion evidence/report commit is pushed before stopping.
