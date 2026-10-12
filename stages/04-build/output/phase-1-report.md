# Phase 1 review report — Barrier Atlas runtime

Run `002-build-phase1`; branch `feature/phase-1-data-model`. The human approved schema-design.md, initial SQL, AR01–AR33 and the scaffold boundary, selected **Barrier Atlas**, and authorized the remaining case IPC/SQLite tree runtime. `TARGET GATE: PASS` was reported before continuation writes. **The authorized runtime work is implemented and tested on Windows; it now awaits review. Phase 2 has not started.**

## Result

- Application title, HTML, npm/Cargo names, Rust library and Tauri identifier now use Barrier Atlas.
- Native Tauri IPC exposes case create/get/open/save/save-as/close and group/diagram creation through rusqlite. No raw SQL or generic filesystem API is exposed.
- The explorer reads the SQLite `case_tree` projection, renders nested selectable/expandable entries and preserves stable identities across saves/reopening. Group creation and hazard/diagram/top-event creation populate the model; no diagram canvas, dragging/linking/layout or Phase 2 authoring tools were added.
- New/editable cases live in memory until explicit Save. Open uses a read-only connection to validate/copy a supported original case. Failed open/create operations preserve the current session. Save writes through SQLite backup to a validated, synchronized same-directory temporary file, closes its database connection and replaces the destination. Failed saves preserve the existing file, session, path and dirty state; unrelated SQLite targets are protected.
- Every connection enables foreign keys, including readers, scratch validators and save destinations. Runtime IDs are opaque UUID v4 values. Diagram/top-event insertion is transactional; failure injection proves rollback of all new records.
- Unsaved changes are marked and prompt Save/Discard/Cancel before switching/closing cases or closing the native window. Native file dialogs choose `.sqlite` files. No recovery timer or undo system was added in this narrowly authorized continuation.
- Cargo.lock now records native dependencies; generated Tauri `gen/` files are ignored. Original schema SQL and source corpus remain unchanged.

## Starting state and authority

Continuation started on clean `feature/phase-1-data-model` at `2379bef4fca31170d8f01c035ec1c2f73b4b609f`, confirmed sole root `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie` and expected origin. PROJECT_STATUS rev 4 records stage 04-build, build approval and no execution hold. The task packet records the explicit architecture approval and remaining runtime scope.

Earlier authorized cleanup remains committed: `890d09f` restored eight secondary references under provenance/; `cdfd005` recorded owner acceptance of LF-normalized local bytes and updated SPEC-013–023 while retaining superseded hashes. Primary scope and audit/incident authority remain unchanged. The first schema/scaffold review was delivered in `2379bef`; its previously missing Rust/Cargo environment is historical, not the current build result.

## Validation actually performed

| Command / check | Result |
|---|---|
| Root/status/branch/HEAD/remotes | PASS; clean expected continuation target |
| rustc --version; cargo --version after shell PATH refresh | Rust 1.99.0 / Cargo 1.99.0 available; owner installation already present. No installation/configuration workaround |
| npm run tauri build -- --no-bundle, before runtime edits | PASS; Windows x64 scaffold executable built in 3m16s |
| npm run build | PASS; TypeScript and Vite. Runtime bundle ~242 kB / 75 kB gzip; earlier Konva scaffold chunk warning eliminated by removing the decorative canvas import |
| cargo fmt; cargo test --lib --release | PASS; **8 tests**: runtime save/reopen/shared hazard/UUIDs, all connection FK settings, diagram transaction rollback, failed-open/create preservation, failed-save preservation and foreign target protection, schema/version rejection, incomplete-diagram rejection, LF/CRLF schema-signature compatibility |
| npm run tauri build -- --no-bundle, final runtime | **PASS**, exit 0; final build 49.70s. Executable: `app/src-tauri/target/release/barrier-atlas.exe` |
| node scripts/check_phase1_native.mjs against the final built app | PASS: real Windows WebView2/native IPC create/group/diagram, five-entity SQLite tree rendered, save/close/reopen preserves IDs/hierarchy, dirty Close case prompt and Cancel preserve session |
| python scripts/check_phase1_schema.py | PASS; approved schema unchanged: all 12 schema steps, 16 entities save/reopen, 18 negative integrity cases |
| Branding scan / diff checks / source hashes | PASS; application labels use selected name, no legacy product names in code/config, source corpus and approved initial SQL unchanged; release remote-network restrictions retained |

The native smoke uses an ephemeral local WebView2 diagnostic port for this project's executable, a hidden process launch, original fixture data and `.runtime/` staging; it closes the owned test process afterward. This diagnostic harness is not an application network dependency. Native file-dialog chooser/cancellation UX was not automated; direct native case IPC and the unsaved-change modal were exercised.

Every native build attempted during this continuation passed. No native build failure was bypassed or worked around. The environment PATH was refreshed from existing machine/user settings before the first attempt because the agent process predated the owner’s Rust installation.

## Interpretation and traceability updates

The approved AR01–AR33 remain recorded. AR28 naming and AR31 first-boundary stop are satisfied by the current human approval. AR34–AR36 log implementation choices and alternatives for this review: in-memory edit session, validated atomic saves, and original format/version/schema/hierarchy validation with CRLF-normalized schema signatures. These new choices are candidates; no automatic canonical memory promotion occurred.

SPEC-013 §§6.2.2, 6.4.1, 6.4.3 and 7.2 define file/unsaved/tree behavior. SPEC-014 §§3.4–3.5 and the explicit continuation constraints support transactional model creation. Root SPEC-TRACEABILITY.md maps these implementations separately from schema-only future behaviors. The earlier 51-row Standard inventory remains a scope inventory, not a claim that every feature is implemented.

## Changed files in this continuation

- `app/package.json`, `app/package-lock.json`, `app/index.html`, `app/README.md`: name and local-dialog dependency/runtime instructions.
- `app/src-tauri/Cargo.toml`, new `Cargo.lock`, `.gitignore`, `src/{lib.rs,main.rs,commands.rs,case.rs}`, `build.rs`, `tauri.conf.json`, `capabilities/default.json`: native service, command registration, selected name, dependency locks/capabilities and formatting.
- `app/src/{App.tsx,style.css}`: case controls, creation forms, tree selection/expansion and unsaved-change flow; no canvas.
- `scripts/check_phase1_native.mjs`: reproducible built-app IPC/tree smoke client.
- Root `SPEC-TRACEABILITY.md`, build-stage inventory, ambiguity log, name record, schema-design continuation and this report.
- `runs/002-build-phase1/`: current packet/state, approval/result/context records and refreshed artifact hashes.

Generated executables/dependency caches, WebView staging and fixture SQLite files remain ignored. No application database, secrets, release, deployment, push or feature-branch merge is committed.

## Remaining limits and review boundary

Windows native build/runtime IPC/tree smoke and producer tests passed. **macOS/Linux builds/runs, independent QA, native chooser UX and the full twelve-step GUI authoring walkthrough remain unverified.** The twelve-step script is schema coverage; Phase 1 GUI supports the authorized case/group/diagram foundation, not later threat/barrier/canvas editing. Large-case performance and recovery/undo remain later scoped work.

The run stops awaiting review of the remaining Phase 1 runtime and AR34–AR36. No Phase 2 canvas work or build-branch merge is authorized by this implementation. Schema/scaffold approval is already recorded and does not need to be requested again.
