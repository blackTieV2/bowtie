# Spec traceability — Phase 1 runtime

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

Full inventory and later sequencing are separate. Eight Rust runtime tests and the native Windows smoke cover this continuation; the twelve-step script remains schema-level coverage for later entities. Native file-dialog chooser UX, full twelve-step GUI authoring, independent QA and macOS/Linux builds/runs remain pending. Phase 2 has not started.
