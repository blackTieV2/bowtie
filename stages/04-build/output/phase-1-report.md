# Phase 1 review report — schema/scaffold boundary

Run `002-build-phase1`; branch `feature/phase-1-data-model`; `TARGET GATE: PASS` reported before modifications. **Rule-5/D-006 architecture-review artifacts are prepared. Full Phase 1 application behavior is not claimed complete.** No Phase 2 work started.

## What was prepared

- Current-baseline inventory: all 51 Standard-marked comparison rows plus 16 manual/module-reference rows, mapped to files/sections and proposed phases.
- Original SQLite schema: identity-bearing entity hierarchy, reusable management records, typed links/properties/lookups, eight configurable risk matrices, explicit assessments, recursive tree projection, and reserved audit/incident reference contracts. See schema-design.md and `app/src-tauri/schema/001-initial.sql`.
- Tauri 2 + React/TypeScript + Konva scaffold under `app/`, with bundled SQLite binding declared in Rust. Frontend components define an empty local workspace and expandable empty categories; browser/native rendering has not been smoke-tested. No case create/open/save IPC, diagram editing, native SQLite application service, audit execution, or incident execution is exposed.
- Original desktop placeholder icons generated from `app/assets/window-mark.svg`; no copied assets/branding. Final naming remains unselected; proposals: Barrier Atlas, Risk Weave, Control Grove.
- AR01–AR33 ambiguity dispositions with alternatives and citations. These include all intake A01–A30 and three architecture questions. D-006 authorizes choices for review, not automatic durable approval.
- Root SPEC-TRACEABILITY.md and schema-level twelve-step acceptance script. No legacy binaries/decompilers/file formats or vendor code/assets were used.

## Starting state and cleanup evidence

Confirmed sole root `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie`, clean `main` at `f641af0` before this instruction; expected origin. PROJECT_STATUS rev 4: stage 04-build, build approved, no execution hold.

Human-authorized cleanup commits on main:

- `890d09f` — restored eight texts from `f641af0^` into provenance/; documented secondary-only precedence and unchanged audit/incident authority.
- `cdfd005` — accepted LF-normalized local bytes, updated SPEC-013–023, retained previous conversion hashes, added human baseline-accepted event and regenerated extraction checksums.

Baseline main for this feature branch is `cdfd005`. SPEC-013/014/015 govern core; SPEC-021 ch.24 + SPEC-023 govern audit; SPEC-002 10.4 is incident version authority with SPEC-022 structure evidence. Existing PDF manifest hashes remain PDF hashes. The accepted event hashes all 25 files in spec/, including index/checksum/provenance files; the checksum list covers all Markdown files. No spec source content was changed to force a hash match.

## Important interpretations

- Multiple diagrams may reference one hazard; one top event per diagram. SQL rejects invalid parents, cycles, identity/kind changes and dangling references.
- Recurring activities and one-off actions remain distinct. Steps 9–11 record task ownership, instructions and verification output; step 12 records human improvement reasoning.
- Deletion is RESTRICT with explicit later detach/reassign; audit/incident evidence is protected from implicit cascades.
- Eight matrices; user-defined cells/categories, no copied default risk matrix or undocumented probability/ALARP formula.
- Audit targets are barrier/activity/document identities. Incident links join separate incident barriers to core barriers. Module scoring/timeline/history behavior is not implemented; future schema evolution remains possible without redefining core IDs.
- Primary text wins over provenance; secondary Quick Start is used only to explain input/output roles where primary §7.2 names them without detail.
- Standard scope follows the newer comparison table. The master separately schedules compare/find-replace as later owner scope; this does not reclassify them as Standard or authorize their implementation now.
- Rule 5/D-006 requires schema/scaffold review before application feature code. The broader Phase 1 summary also calls for persistence/tree behavior: those runtime features remain after this first review, before Phase 2. This boundary interpretation is AR31.

## Commands actually run and results

| Check | Result / practical limit |
|---|---|
| Root/status/branch/HEAD/remotes recon | PASS; expected target, clean start, authorized branch |
| LF-normalized SHA-256 against SHA256SUMS and SPEC-013–023 | PASS; previous hashes preserved; human acceptance recorded |
| npm install --ignore-scripts --no-audit --no-fund | PASS; development dependencies installed and locked |
| npm ci --offline --ignore-scripts --no-audit --no-fund | PASS using populated cache; demonstrates cached development install, not GUI network isolation |
| npm run build | PASS; TypeScript check and Vite production output. Main JS chunk ~548 kB / 168 kB gzip; Vite emitted a chunk-size warning |
| python scripts/check_phase1_schema.py | PASS on SQLite 3.49.1: all 12 schema steps, 16 entities saved/reopened, 18 negative cases, FK/integrity checks, audit/incident reference persistence |
| npm run tauri info | CLI/config inspection ran; Windows x64/WebView2/MSVC present; Rust/Cargo absent |
| npm run tauri build -- --no-bundle | FAILED: cargo metadata could not run because Cargo is not installed. No native binary produced |
| npm run tauri icon with original SVG | PASS; desktop PNG/ICO/ICNS retained. Font warning concerned a system font; the SVG contains no text |
| Branding/CSP/capability/inventory/decision checks | PASS: no legacy names in application code/config; release connect-src none; core window capability only; 51 rows/33 decisions |

The first schema check found a trigger CASE terminator syntax error; it was corrected, and the acceptance script then passed. Connection cleanup was strengthened so failed checks close handles on Windows. Long documentation/record shell calls were rejected before execution by tool policy; the same work was completed through scoped patches and shorter calls. No permission bypass was attempted.

Schema checks exercise SQLite directly, not the GUI or a Rust service. The frontend build does not establish native runtime correctness. Independent producer-separated QA has not run. macOS/Linux builds/runs, Windows native build/run, actual GUI twelve-step acceptance, and runtime offline isolation are **unverified**. No Rust Cargo.lock exists yet; resolving and locking native dependencies is required with a Rust toolchain. The native all-platform requirement therefore remains unsatisfied; do not approve Phase 2 as complete based on these checks.

## Files changed on the feature branch

- `app/package.json`, `app/package-lock.json`, `app/tsconfig.json`, `app/vite.config.ts`, `app/index.html`, `app/README.md`.
- `app/src/{main.tsx,App.tsx,style.css}` — original empty shell.
- `app/src-tauri/{Cargo.toml,build.rs,tauri.conf.json}`, `app/src-tauri/src/{main.rs,lib.rs}`, `app/src-tauri/capabilities/default.json`.
- `app/src-tauri/schema/001-initial.sql`, `app/assets/window-mark.svg`, `app/src-tauri/icons/{icon.png,icon.ico,icon.icns}`.
- `scripts/check_phase1_schema.py`, root `SPEC-TRACEABILITY.md`.
- `stages/04-build/output/{feature-inventory.md,AMBIGUITY-RESOLUTIONS.md,schema-design.md,name-proposals.md,phase-1-report.md}`.
- `runs/002-build-phase1/` — copied/filled task packet, state, context manifest, retrieval feedback, promotion review, validation evidence and output hash index.

Generated node_modules/dist and icon staging under .runtime are ignored. No application database, secret, release, deployment, push or build-branch merge is included.

## Review requested before continuing

Approve or revise the schema relationships, reserved module references, scope interpretations, AR01–AR33 and preferred name/identity. Confirm the rule-5 review boundary and authorize the remaining Phase 1 runtime persistence/tree implementation separately before Phase 2. Complete native dependency locking and build/run verification on Windows/macOS/Linux, followed by independent requirements validation and GUI acceptance. This run stops awaiting that review; it does not start Phase 2 or automatically mark requirements approved.
