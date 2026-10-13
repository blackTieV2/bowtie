# Task Packet — 007-build-phase6

Stage: 04-build. Task: Phase 6 ONLY, audit module.

TARGET GATE: PASS before modifications, 2026-10-13. Sole repository root `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie`; clean `feature/phase-5-case-tools` at `927b259d94354ca23104ed604b9b5370b5a81e77`; expected origin `https://github.com/blackTieV2/bowtie.git`; build approved, no execution hold. `feature/phase-6-audit-module` created from that tip. Sole producer owns this mutable tree; validator is read-only.

Current human approval: Phase 5, its validator process/defect dispositions and AR64–AR69 accepted. Phase 6 alone authorized. Historical bootstrap status pointers and run 006 pending-review text are superseded by explicit current human approval; accepted run 006 records/hashes are preserved rather than rewritten.

## Current human authorization — preserved verbatim

Human review complete: Phase 5 is APPROVED (compare, find/replace, CSV/XLSX exchange, lookup tooling; validator process and defect dispositions accepted; AR64AR69 accepted).

You are authorized to execute Phase 6 ONLY: the AUDIT MODULE  audit surveys/questions derived from bow-tie barriers, audit execution workflow, scoring, and review & report (per REBUILD-MASTER-PROMPT.md Phase 6).

1. TARGET GATE: PASS before any modification. Confirm sole repository root, clean tree, branch feature/phase-5-case-tools at 927b259.
2. Create branch feature/phase-6-audit-module from that tip. Open run packet runs/007-build-phase6/.
3. Authority: SPEC-021 chapter 24 (AuditXP add-on) is authoritative; SPEC-023 (AuditXP Training Slides) defines workflows/acceptance. Cite every feature in SPEC-TRACEABILITY.md. Where they conflict, flag via AR and stop.
4. Scope:
   a. Audit definition: create audits with question hierarchies per spec; derive audit questions/surveys from case barriers (and activities/documents per the audit_target trigger) exactly as the spec describes.
   b. Survey execution: the documented workflow for conducting a survey against a case  answering, evidence/notes, completion  using the approved audit tables (audit, audit_question, audit_question_target, audit_survey, audit_survey_question). Responses are case data and must persist in the .sqlite case.
   c. Scoring/results: implement ONLY the scoring/result model the spec documents. Do not invent scoring formulas, weightings, or traffic-light logic not in the text  same discipline as AR20 for ALARP.
   d. Review & report: the spec-documented audit review outputs, integrated with the Phase 4 export/report machinery where the spec calls for reports.
   e. SCHEMA GUARD: the Phase 1 audit tables were designed for this phase. If any spec-required behavior genuinely does not fit the approved schema, STOP with an AR and a migration proposal  no silent migration, no trigger bypass.
5. Standing rules unchanged: no decompilers, no legacy binaries/names/branding, offline-only, parameterized SQL, triggers authoritative, producer  validator.
6. Validation bar: new cargo tests (question-hierarchy cycles, target-kind trigger enforcement, survey CRUD, scoring correctness against worked spec examples, save/reopen equality of all audit data). Extend the native GUI script to reproduce the audit walkthrough from SPEC-023 against the liquid-transfer case. Validation sub-agent is authorized again under the same five constraints as Phase 5 (read-only, fresh context, separate evidence, defects returned to producer, no phase-gate replacement). Evidence to runs/007-build-phase6/.
7. Ambiguities: per D-006, append from AR70.
8. STOP at the boundary with stages/04-build/output/phase-6-report.md. PUSH feature/phase-6-audit-module to origin when done. No merge, no Phase 7.

## Initial relevant evidence

SPEC-021 §§24.3–24.4: barrier/management-system/general question levels, workflow and all audit data in the case file. §§24.6.1–24.6.2: ordered ordinal choices, single response, grouping-only hierarchy, keying/center marker, most-negative/contextual options. §§24.8–24.9.1: flat survey subsets, code/name/description, respondent/date/unit, answers/remarks and fill-out workbook exchange. §§24.10–24.10.2: qualitative counts/details and survey/unit/date/respondent/results filtering. SPEC-023 describes matching preparation/conduct/review workflow. Server-only long-format surveys are outside offline scope.

Approved schema checked directly: audit and question metadata JSON, question answer array, target associations, surveys and ordered question membership; exact schema and foreign-key/cycle/target triggers remain authoritative. Storage-fit assessment precedes feature implementation. No scoring formulas or schema changes inferred.
