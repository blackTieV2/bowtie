# Intake brief — Standard-edition bow-tie editor

Status: draft requirements candidate, awaiting human review. Run: `001-intake`. Prepared 2026-10-13 Asia/Singapore (2026-10-12 UTC). This document defines requirements; it does not approve design, implementation, memory promotion, or release.

## Goal and stakeholders

Provide the repository owner with a modern 64-bit offline desktop application for creating, maintaining, reviewing, and communicating bow-tie risk analyses on Windows, macOS, and Linux. Reimplement documented Standard-edition behavior with original code and a new visual identity. The owner is the scope decision-maker and acceptance reviewer; analysts create the cases, and reviewers consume diagrams, responsibilities, documents, and reports. Professional use is the intended context; the application records the analyst's assessment rather than establishing its correctness automatically.

The owner's license and the legacy product's unsupported status are recorded project premises from `PROJECT_STATUS.md`, not independently verified legal conclusions.

## Evidence and scope authority

Read order by role: indexed R6 software manual and 2012 edition matrix; additional edition comparison and R8.2 manuals as supplemental feature evidence; twelve-step walkthrough and quick starts as acceptance evidence; management/methodology/training references as supporting evidence; Advanced brochure as edition-boundary evidence; incident, audit, portal, and combined product brochures as deferred-module/boundary evidence. Exact files and review depth are recorded in `runs/001-intake/context-manifest.yaml`.

The README indexes 12 documents, but the live corpus has 23 content documents. The newer material does not silently supersede the indexed baseline. Two R8.2 manual copies differ in a blank line and release-notes menu spacing. Nine of the eleven checksum-listed files match after CRLF-to-LF normalization; R6 and the indexed training slides still differ. The older quick start is absent from that checksum list. PDF manifest hashes refer to PDFs, not these Markdown extracts; the retained PDFs were not available for verification. Source integrity and version precedence remain unresolved (A01–A03 in [spec-ambiguities.md](spec-ambiguities.md)).

## Scope

The v1 scope intent is the Standard-edition bow-tie editor, not merely drawing shapes. [feature-inventory.md](feature-inventory.md) records every feature explicitly marked Standard in the two edition tables, supporting manual behaviors, documented Standard report entries, and additional version-dependent candidates. Inventory membership is evidence of documented behavior, not automatic approval to implement a disputed feature.

The core requirements are:

- Local case creation, open/save/save-as/close, templates, unsaved-change handling, recovery, undo, and redo.
- A navigable hierarchy of diagram groups/locations, hazards/top events, threats, consequences, preventive/recovery barriers, escalation factors, and escalation-factor barriers; creation, editing, deletion, ordering, and relationships.
- Configurable lookup values and colors, responsibility/job-title information, activities, competencies, systems, document references, and one-off improvement actions with completion/sign-off details.
- User-assigned inherent and residual risk assessments on top events and consequences, configurable matrices, and display of those assessments.
- Diagram navigation, zoom, overview, expansion/collapse, selective visibility, Standard reference filtering, fonts, colors, extra information, and configurable display/layout behavior.
- Text search, usage references/backlinks, case overviews/registers, quality checks, diagram/legend export, printing, and documented Standard reports/table exports.

Features requiring a human decision include imports involving proprietary legacy formats, exact reports, online/external integrations, edition-conflicting overview/profile/search functions, and the extra R8.2 features. They are retained in the inventory and ambiguity list rather than silently removed or included.

Explicit v1 exclusions from current project scope: incident investigation (including timelines and mappings onto bow-tie barriers; separately gated future scope), audit/questionnaire modules, portal/web viewing, SharePoint synchronization, enterprise collaboration/server services, and spreadsheet/formula add-ons. Advanced-only functions such as SOOB, chaining, file comparison, scrapbook, bulk/group editing, multi-criterion filtering, live equipment state, and barrier bundling are outside Standard scope unless the human explicitly changes it. Standalone-capable R8.2 server-oriented fields need a scope decision; excluding a service does not prove every associated local field is excluded.

## Constraints

- Use documentation as behavioral evidence. No legacy binary or original source code is an input; do not copy legacy code, wording for product UI, artwork, icons, branding, or proprietary file formats. Spec filenames and section titles here are provenance references only.
- Create original product wording, name, assets, and visual identity. Preserve behavioral relationships and information without requiring pixel-identical presentation.
- Core authoring, saving, review, and agreed exports must work offline. Online links and mail/update/template-download behaviors in the manuals conflict with that constraint and need explicit boundaries.
- Keep work private and personal/internal under the recorded project constraints; no publication, distribution, sale, deployment, or release is authorized by this run. This is a scope constraint, not legal clearance.
- Support 64-bit Windows/macOS/Linux; supported versions, packaging, and acceptance tolerances await decisions. No dependency on a particular model/vendor for project truth.
- Architecture belongs to 03-design. The stack in project status is proposed, not selected here. Intake chooses no storage, canvas, framework, or file format.
- Preserve sources and governance. This run writes only intake output and the explicitly requested run directory. Every ambiguous behavior remains a question, not an inferred requirement.

## Success criteria

These are proposed acceptance criteria to be agreed before design/build; nothing has been implemented or tested in this run.

| ID | Observable outcome | Evidence / dependency |
|---|---|---|
| SC01 | A new local case can be created, edited, saved, reopened, and closed; the accepted model, references, matrices, and actions survive reopening. Unsaved changes prompt the user, and recovery behavior is demonstrated. | R6 §§6.2, 6.4–6.5; A21–A22 |
| SC02 | The Standard portions of the older Quick Start §§3.3–3.10.4 and R6 §§3.3–3.12 can be completed with the same entity relationships and visible results. Later Quick Start §§3.3–3.12 applies if its version is selected. | Acceptance guides; A01, A10, A19 |
| SC03 | All twelve walkthrough steps below can be completed using original examples and retained evidence, including management tasks, ownership/instructions, verification, and improvement planning. | Twelve-step guide pp.1–4; A19–A20 |
| SC04 | Every accepted Standard inventory row has a spec citation and a later acceptance check; disputed/excluded rows have an explicit human disposition. Shipped features appear in `SPEC-TRACEABILITY.md` during 04-build. | AGENTS.md; inventory; human review |
| SC05 | The agreed case/workflows and exports function without network access on each approved 64-bit OS target. No activation service or vendor network infrastructure is required for the new app. | Project constraints; A13–A18, A25–A26 |
| SC06 | Lookup replacement/deletion, move/copy operations, and link assignment preserve the agreed relationship semantics. Filters and collapse change presentation without erasing model data. | R6 §§3.11, 7.7–7.10; A22–A24 |
| SC07 | User-entered matrix assessments, lookup metadata, responsibility records, and action status are reflected in agreed overviews, quality checks, reports, and diagram exports. | R6 §§7.11, 8, 9.1–9.2, 12; A04, A09, A11–A12 |
| SC08 | Human-approved scale/readability and export tolerances are met for original small and large cases; export fidelity is assessed on all target platforms. | R6 §§7.4–7.5, 8.2.4; A15–A16, A23, A26 |

### Twelve-step acceptance coverage

| Step | User outcome (paraphrased) | Software evidence / unresolved boundary |
|---|---|---|
| 1 | Identify and record the hazard. | R6 §3.5; group/location prerequisite §3.4 |
| 2 | Record the loss-of-control top event. | R6 §3.5; multiple-top-event modeling A18 |
| 3 | Record credible threats. | R6 §3.6 |
| 4 | Record potential consequences. | R6 §3.7 |
| 5 | Add preventive barriers to threat paths. | R6 §3.8 |
| 6 | Add recovery/mitigation barriers to consequence paths. | R6 §3.8 |
| 7 | Identify factors that undermine barriers. | R6 §3.9 |
| 8 | Add barriers against those factors. | R6 §3.9 |
| 9 | Define recurring tasks that sustain barriers and link them. | R6 §3.11.3; older Quick Start §3.10.3 |
| 10 | Record task ownership, timing, and instructions. | R6 §§3.11.2–3.11.3, 7.2; older Quick Start §3.10.3.4; A19 |
| 11 | Record how task completion is verified. | Older Quick Start §3.10.3.4 inputs/outputs; A19–A20. Full audit module remains excluded. |
| 12 | Record and review possible improvements and their justification. | R6 §3.12; methodology §§6.2.4–6.2.5; A20. No undocumented automated ALARP calculation is assumed. |

## Open questions and next safe action

The numbered questions in [spec-ambiguities.md](spec-ambiguities.md) are the complete set identified in this role-based intake review, not a guarantee that no further ambiguity will emerge during research/design. Resolve provenance/version precedence and edition/import conflicts first, then acceptance/output/platform decisions. The owner reviews the draft and records explicit dispositions; a subsequent assigned run can enter 02-research to verify missing evidence. No stage advancement, build approval, or governance correction occurs automatically.

Project status still records `00-triage` and `execution_hold: false`. The explicit current assignment authorizes this intake run. The malformed build flag and conflicting approval narrative are recorded as A29; they are not treated as implementation authority. The expected repository is clean at the recorded starting HEAD, already on `main`; no merge or commit is part of this task.
