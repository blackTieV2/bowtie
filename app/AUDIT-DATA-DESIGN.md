# Original audit case-data contract

Phase 6 uses the approved Phase 1 schema without a migration. SPEC-021 chapter 24 supplies behavior; SPEC-023 supplies the prepare, conduct, review workflow.

| Data | Approved storage | Validation |
|---|---|---|
| Audit identity/title | audit | Nonempty title, unique ID |
| Question hierarchy/title | audit_question | Same-audit parent, FK/cycle triggers; whole-bundle checks |
| Ordered choices | audit_question.answers_json | Distinct nonempty choices, valid center marker/keying |
| Criteria/keying/contextual/most-negative | audit_question.metadata_json | Typed question metadata |
| Barrier/activity/document assignments | audit_question_target | Target-kind trigger/FK and runtime existence checks |
| Survey title/flat ordered subset | audit_survey, audit_survey_question | Same-audit membership and unique contiguous order |
| Survey code/description/fillouts | audit.metadata_json.surveys map | Exact correspondence to relational survey IDs; typed identity/date/respondent/unit/completion |
| Answers/context/remarks | Fillout answers array in that map | Exactly one slot per question/context; known context/choice; completed fillouts fully answered |
| Saved review/diagram filters | audit.metadata_json.filters | Valid survey/question/fillout/target references and date range |

Audit metadata version is 1. Approved relational tables remain authoritative for relationships. Metadata holds audit-owned records. Case open/save/snapshot/import validate these records. Invalid or unsupported metadata is rejected. Entity deletion reports audit references including saved profiles. Changing response scales/keying/contextual mode after answers exist is rejected to preserve meaning.

CaseStore validates the requested bundle and expected current baseline, then replaces its owned subrecords in one transaction. Foreign keys/triggers stay enabled. Failures preserve session/saved bytes. Normal Save persists all data in the .sqlite case. Multi-workbook import parses everything before one CaseStore apply.

Results count ordinal answers, retain details/unanswered counts and select the worst ordinal response when requested. Default center is the ordinal midpoint; @Choice explicitly centers a choice and standalone @ centers a boundary. Keying controls direction. No arithmetic weighting, risk reduction, ALARP score or automatic corrective judgment is calculated. Supporting activity/document links expose relevant questions in barrier context.

## Original respondent workbook

The first worksheet, Audit form, has twelve exact headers:

`survey_id,question_id,context_id,question,criteria,context,answer_options,answer,date,respondent,unit,remark`

Each row is one shared/contextual slot. Hidden original choices and named ranges supply dropdowns. Answer/date cells use text formatting; enter dates as YYYY-MM-DD. Import also recognizes ISO date cells and numeric cells explicitly styled as calendar dates, including 1900/1904 date systems. Fictitious 1900-02-29 is rejected. General-format numbers are not inferred as dates.

Import requires the complete unchanged slot set, question/criteria/choice definitions, one respondent/date/unit across rows, and exact choice labels. Missing answers produce drafts; fully answered forms produce completed fillouts. Remarks are local evidence/notes. No attachments, network retrieval, external formula evaluation, historical workbook layout or server coupling is introduced. Existing 20 MiB expanded-workbook limits apply.

Core case-table exchange includes full audit bundles, preserving definitions, responses and profiles. DOCX review reuses the Phase 4 original report generator; general case reports include audit sections and audit-only reports reflect selected filters. Interactive canvas audit overlays do not alter the established core scene exported by Phase 4 image exports.

Phase 7 compatibility update (human-resolved AR78): audit tables and data envelopes are unchanged. Case schema v2 changes only the incident link insert/update triggers; v1 case opens migrate the working copy and explicit Save persists v2. All audit runtime checks and equality tests remain unchanged. See schema/002-rca-event-links.sql and run008 migration evidence.
