# Phase 5 independent validator report

Validator: `/root/phase5_validator`  
Date: 2026-10-13  
Verdict: **PASS for tested Phase 5 Windows scope. Final human phase-gate approval remains required.**

## Independence and target

Read-only validator context loaded, in order:

1. `AGENTS.md`
2. `PROJECT_STATUS.md`
3. `CONTEXT.md`
4. `stages/05-qa/CONTEXT.md`
5. Only `Consolidated current human authorization — preserved verbatim` from `runs/006-build-phase5/task-packet.md`
6. Relevant SPEC-013 §§7.3, 9.4–9.4.1, 12.3, 19.1–19.3 and SPEC-015 feature matrix
7. Changed implementation, format documentation, test assertions and native diagnostic scripts

Producer reports and narrative were not used as validation conclusions. Producer messages identified artifact readiness and corrections; validation conclusions below come from direct review and independently executed checks.

Repository: `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie`  
Branch: `feature/phase-5-case-tools`  
HEAD during validation: `ad08c71c025cd8733e5d3a475fed855b6e717d86`

Recon verified root, branch, HEAD, remotes and working-tree state. Dirty files were expected ongoing producer work. Validator made no tracked-file edits, commits, schema changes or test changes. Generated evidence is separate under `.runtime/phase5-validator/`; producer evidence was not overwritten.

`PROJECT_STATUS.md` retains bootstrap-era phase pointers. The explicit consolidated current human authorization supplies Phase 5 scope and overrides those stale pointers. No execution hold was found.

## Checks actually executed

- `C:/Users/BlackTie/.cargo/bin/cargo.exe test --locked --offline` from `app/src-tauri`, including a rerun after final Rust changes: **37 passed, 0 failed**.
- `npm run build` from `app`: **PASS**. Vite emitted a bundle-size advisory.
- `npm run check` after final frontend guard/wording changes: **PASS**.
- `./scripts/check_phase5_native.ps1 -EvidenceRoot '.runtime/phase5-validator/evidence' -RuntimeRoot '.runtime/phase5-validator/runtime'`: **PASS, exit 0**.
- `python scripts/check_phase5_artifacts.py .runtime/phase5-validator/evidence`: **PASS**.
- Additional validator-authored Python, executed through stdin using standard-library `sqlite3`, `csv`, `json`, `hashlib`:
  - Opened `imported.sqlite` and `spreadsheet-imported.sqlite` with SQLite URI `mode=ro`.
  - Checked `PRAGMA integrity_check` and `PRAGMA foreign_key_check`.
  - Compared every entity’s normalized root identity, parent, kind, title, description, order, type key, metadata, effectiveness identity and full property map against independently parsed CSV.
  - Checked link, matrix, assessment and lookup counts against CSV.
  - Confirmed both saved files remained byte-identical during inspection.
  - **PASS**.
- Direct code review of read-only opening, canonical schema validation, transactional services, preview signatures, SQL parameter binding, comparison report protection, CSV parsing and XLSX conversion.
- Direct inspection of validator-generated find/replace and import-preview screenshots.
- `git diff --name-only -- '*schema*'`: no schema changes.

## Requirement verdicts

| Requirement | Verdict | Independent evidence |
|---|---|---|
| Complete find/replace preview before application, including entity paths and before/after values | PASS | Static review; native preview row count/path assertions; screenshot inspection |
| Individual/all application in one transaction, post-apply count, stale-preview rejection and rollback | PASS | Passing `case_tools` tests; native apply/count checks; transaction and signature review |
| Documented search behavior; no unauthorized regex or switches | PASS | Extracted spec documents find/results/individual/all replacement but no regex or case/scope toggle; implementation uses literal case-sensitive matching |
| Validated second case opened read-only and never written | PASS | Read-only SQLite flags and pinned read transaction; compare tests; native saved-byte equality and active-session equality |
| Structured entities/properties/positions, links, matrices/assessments and actions comparison plus report | PASS | Canonical-table comparison review; passing comparison tests; native report generation; independent report parsing |
| Compare only; no merge/copy/apply direction | PASS | No such direction found in reviewed comparison spec sections; implementation exposes comparison/report only |
| Original documented CSV/XLSX layout exporting required records | PASS | `app/CASE-TABLE-FORMAT.md`; independent CSV/XML parsing: 21 entities, 7 links, 1 matrix, 4 assessments, 1 lookup, including actions and saved positions |
| Import validates before application and reports malformed input with line diagnostics | PASS | Parse/preflight/scratch-transaction review; tests explicitly exercise unknown kind, bad parent, cycle, duplicate identity, malformed CSV and invalid typed property |
| Dry-run creation summary and single transactional application through shared typed services with authoritative triggers | PASS | Static review; native dry-run/application; dedicated relationship rows; transaction and shared-service review |
| Rejected import preserves saved bytes and session state | PASS | Cargo rejection tests and native malformed-import byte/state assertions |
| Export → import → export structural equality | PASS | Cargo equality test; native CSV byte equality; independent CSV/XLSX structural equality; independent SQLite/CSV value comparison |
| Save/reopen equality after find/replace, compare and CSV exchange | PASS | Independently rerun native acceptance assertions |
| Standard lookup tooling completion | PASS | Create/edit/delete/color/order controls reviewed; native sparse-order append, reorder, preserved assignments and save/reopen checks |
| Offline operation, parameterized SQL, unchanged schema and original application branding | PASS within reviewed scope | Offline Cargo run; local native execution; static review; unchanged schema; Barrier Atlas UI inspection |
| New Cargo coverage and extended native workflow | PASS | 37 passing tests; full native Phase 5 run plus twelve-step and Phase 3 regression |

## Defects reported and disposition

1. **Extra case rows and malformed case payloads could be silently ignored.**  
   Reported independently during static review. Producer corrected total case-row cardinality and validates canonical columns/payload regardless of destination state. Direct re-review and passing regression test verified correction.

2. **Imported entity titles could lose edge whitespace, breaking structural equality.**  
   Reported independently during static review. Shared insertion now preserves persisted import titles; normal editor normalization occurs before calling it. Replacement previews show normalized title results matching application. Regression coverage also verifies persisted entity, lookup and matrix title whitespace round-trips. Verified by direct review and executed tests.

3. **New lookup values could appear before existing values with sparse order numbers.**  
   Reported independently against SPEC-013 §7.3. Producer changed initial order to maximum existing order plus one. Direct review and independent native sparse-order append check verified correction.

**Outstanding blocking defects: none found.**

## Final artifact hashes

SHA-256:

| Artifact | Hash |
|---|---|
| `app/src-tauri/target/release/barrier-atlas.exe` | `30ad0a0cf62b8a8e119a8776e4fa253a9de8ea1f20b68dfa8c4b8ab67273f0ed` |
| `app/src-tauri/src/case_tools.rs` | `bc4302404d4d0ee910b2c421aac19e15c3d0cf289007fe4a3de117be94940e46` |
| `app/src-tauri/src/compare.rs` | `0fb17795d23ecadd3e55dbbfaeb974407b780ba4e7d21a6b68e1f2a20a6c9291` |
| `app/src-tauri/src/exchange.rs` | `6526dd1401908313bbd2494207a87a303a255e8adb780c4af8ff0615f30d0b5c` |
| `app/src/FindReplace.tsx` | `5998f00505925cdea198e36589db5a4b6cfa33f832c9e150bba0a8931f34d083` |
| `app/src/CaseCompare.tsx` | `dead057bf75dc5b0cb42e16d7385b0a97a992db794464ef04b32b95387f07b56` |
| `app/src/CaseExchange.tsx` | `6f09f5b7ebbdff13be16a9e74909cc7cc0c5df75a7476092d2398f17083cb68f7` |
| `app/src/LookupManager.tsx` | `3d2c9988edd3683228fdb675799cd8c0db4660b4000220fc3ed01ac3470a49d7` |
| `app/src/tableWorkbook.ts` | `172031d163d84c70c8e2f0497610abbd721e4eeef437763a939c27084685d53b` |
| `app/CASE-TABLE-FORMAT.md` | `c8b7b594160930fbbbbe017de6bdeadbda2cb27db48116afddd3699e210395ed` |

Validator-generated artifact SHA-256:

- CSV and reexport: `2c141e34a3e0efbe1bead710e5bcca335e0381737ae5264c9e853161a825e365`
- XLSX: `f41908ec181d7d3f2756c1d7c3ab1f26a0a9dacb841140db6e13bf13571b4c09`
- Full diff report: `fef3a50d601f2ebb4c5069e768f4a5cde45028f99bfebfb35723c86a648d2e84`
- CSV-imported SQLite: `559fcbbd19892a14f539c337402aaa7194641fb0871e4757292a8176a5fadb27`
- XLSX-imported SQLite: `40f20aba82d89f051c927ca5e9fee746c1b1710a18b2bc46dab2fa67ea48ab27`

## Limitations and phase boundary

- Native execution was Windows-only. macOS/Linux builds and GUI behavior remain unverified.
- Native file chooser interaction was not exercised; diagnostics supplied local paths.
- XLSX export/import and independent XML parsing passed, but opening files in Microsoft Excel or LibreOffice was not tested.
- No large-case performance or exhaustive hostile-input campaign was performed.
- Static review supports offline operation and original branding; it does not prove code provenance or network isolation under every possible environment.
- Human methodology and final phase-gate acceptance remain human responsibilities.

This validation authorizes no merge, release, deployment, memory promotion or Phase 6 work.
