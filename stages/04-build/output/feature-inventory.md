# Feature inventory — current baseline

Run `002-build-phase1`. Architecture-review candidate under D-006; no application feature is claimed implemented by this inventory. The Standard column in SPEC-015 defines edition scope. Primary manual SPEC-013 defines behavior. Secondary provenance is consulted only if primary evidence is silent. Original application wording is required; table row labels below are source locators.

## Exact source keys

| Key | Spec file | Authority |
|---|---|---|
| SPEC-013 | `08 BowTieXP Software Manual Rev 31 for Ver 8.2.md` | Core behavior |
| SPEC-014 | `07 BowTieXP 8.0 and up Quick Start Manual.md` | Acceptance workflows |
| SPEC-015 | `02 Compare BowTieXP Standard & Advanced.md` | Edition scope |
| SPEC-016 | `06 Methodology Manual V15.md` | Methodology |
| SPEC-019 | `01 GuidingPrinciples(A5 Back) 2017.md` | Methodology quick reference |
| SPEC-004 | `BowTieXP_in_12_steps.md` | Twelve-step acceptance |
| SPEC-021 | `Software User Manual Rev 31 for Ver 8.2 & Up.md` | Audit ch.24 / incident integration ch.29 |
| SPEC-023 | `AuditXP Training Slides_Rev D.md` | Audit workflows |
| SPEC-002 | `IncidentXP_Software_Manual_-_For_Release_10.4.md` | Incident version authority |
| SPEC-022 | `IncidentXP Software Manual - For Release 8.2.md` | Incident structure |

## Every Standard-marked comparison row

The file contains 51 Standard-marked rows. The screen-layout row embeds extra-information display; the personal-layout description and next row split the spreadsheet-export label. Both are accounted for here. Phase values are sequencing proposals, not permission to continue. Schema means representation/integrity proposal; none of the listed GUI workflows is implemented at this first rule-5 gate.

| ID | SPEC-015 section + row | Behavior locator (SPEC-013 unless noted) | Proposed phase / status |
|---|---|---|---|
| F01 | GENERAL: Create and Edit Bowtie Diagrams | 3.3–3.10 | 1 — schema/scaffold proposal; remaining UI later |
| F02 | GENERAL: Bowtie Groups | 3.4, 7.2 | 1 — schema/scaffold proposal; remaining UI later |
| F03 | GENERAL: Change Language | 6.3 | 3 — future implementation |
| F04 | GENERAL: Risk Matrices (Create & Edit) | 7.12 | 1 — schema/scaffold proposal; remaining UI later |
| F05 | GENERAL: Action Register | 3.12 | 1 — schema/scaffold proposal; remaining UI later |
| F06 | GENERAL: Lookup Tables (Basic) | 7.3 | 1 — schema/scaffold proposal; remaining UI later |
| F07 | GENERAL: Import Entities (Basic) | 12.2.1 | 3 — future implementation |
| F08 | GENERAL: Copy, Cut, Paste & Paste Special | 7.8 | 3 — future implementation |
| F09 | GENERAL: Colour Settings for Barriers/Controls | 8.2.2, 8.3 | 3 — future implementation |
| F10 | GENERAL: (Colour settings for bowtie elements) | 8.1–8.3 | 3 — future implementation |
| F11 | GENERAL: Screen layout (user defined) | 6.7, 8.2.1 | 3 — future implementation |
| F12 | GENERAL: Saving diagrams as Images | 13.4 | 4 — future implementation |
| F13 | GENERAL: Copy to clipboard | 13.4 | 4 — future implementation |
| F14 | GENERAL: Export to Visio | 13.4 | 4 — future implementation |
| F15 | GENERAL: Printing  Diagrams | 13.6 | 4 — future implementation |
| F16 | GENERAL: Changing fonts | 7.5.3, 8.2.3 | 3 — future implementation |
| F17 | GENERAL: Colour legend | 13.5 | 4 — future implementation |
| F18 | GENERAL: Personal layout | 6.7 | 3 — future implementation |
| F19 | GENERAL: checks etc., to Excel | 13.3 | 4 — future implementation |
| F20 | GENERAL: Create Template | 14.1–14.7 | 5 — future implementation |
| F21 | GENERAL: Treeview | 7.2 | 1 — schema/scaffold proposal; remaining UI later |
| F22 | GENERAL: Treeview Filtering | 7.11 | 2 — future implementation |
| F23 | GENERAL: Selective Visibility Filters | 7.5.4.1 | 2 — future implementation |
| F24 | GENERAL: Advanced Filters (1 Criteria) | 7.5.4.2 | 2 — future implementation |
| F25 | GENERAL: Apply Basic Terminology | 20.2 | 3 — future implementation |
| F26 | GENERAL: Spelling check | SPEC-015 row only | 5 — future implementation |
| F27 | GENERAL: Search & Find | 9.4 | 5 — future implementation |
| F28 | GENERAL: Import data from Thesis™ | 6.4.2 | 5 — proprietary import excluded; original new-case import only (AR14) |
| F29 | GENERAL: Quality Checks | 7.15, 9.3 | 3 — future implementation |
| F30 | GENERAL: Document Links | 10.1–10.3 | 1 — schema/scaffold proposal; remaining UI later |
| F31 | GENERAL: Export Treeview data & Diagram data | 13.3 | 4 — future implementation |
| F32 | GENERAL: Reports (Basic) | 13.2 | 4 — future implementation |
| F33 | GENERAL: Reports (Advanced) | 13.2; edition labels conflict | 4 — catalog interpretation AR11; not all Advanced functionality included |
| F34 | GENERAL: Auto detect data entry | SPEC-015 row only | 3 — future implementation |
| F35 | GENERAL: Search for data in the file | 9.4 | 5 — future implementation |
| F36 | GENERAL: Case File Properties | 6.4.1; SPEC-015 row | 1 — schema/scaffold proposal; remaining UI later |
| F37 | GENERAL: Acceptance Criteria | 7.13 | 1 — schema/scaffold proposal; remaining UI later |
| F38 | GENERAL: Actions | 3.12 | 1 — schema/scaffold proposal; remaining UI later |
| F39 | GENERAL: Revision Info | 7.16 | 3 — future implementation |
| F40 | GENERAL: Check for Updates | 32.2.19–32.2.20 | Excluded at runtime by offline-only override (AR25) |
| F41 | NAVIGATION: Backlinks | 3.11.4, 7.9.2 | 1 — schema/scaffold proposal; remaining UI later |
| F42 | NAVIGATION: Go To Definition | 7.5, 9.2 | 2 — future implementation |
| F43 | NAVIGATION: Undo and Redo | 6.5 | 5 — future implementation |
| F44 | NAVIGATION: Back and Forward | 6.9.1 | 2 — future implementation |
| F45 | NAVIGATION: Zoom in/out | 7.5 | 2 — future implementation |
| F46 | NAVIGATION: Stage diagrams (Expand & Collapse) | 7.5.1 | 2 — future implementation |
| F47 | NAVIGATION: Display Risk Assessments | 7.12, 8.2.1 | 3 — future implementation |
| F48 | NAVIGATION: Choose a display profile | 17.1; use versus authoring | 2 — future implementation |
| F49 | NAVIGATION: Drag and Drop | 7.10 | 2 — future implementation |
| F50 | NAVIGATION: Change display options  (the eye icon) | 8.1 | 2 — future implementation |
| F51 | NAVIGATION: Maximise / Minimise Treeview | 7.2, 6.9.1 | 2 — future implementation |

## Additional manual and shared-model behaviors

| ID | Behavior | Exact source / section | Gate treatment |
|---|---|---|---|
| M01 | Local file lifecycle, unsaved prompts and recovery | SPEC-013 §§6.2, 6.4.1–6.4.4 | Schema file version/metadata only; runtime after review |
| M02 | Hierarchical reusable activities and responsibility links | SPEC-013 §§3.11.2–3.11.4, 7.2–7.4 | Typed entity/link schema; recurring work separate from actions |
| M03 | Systems, competencies, objectives, inputs/outputs | SPEC-013 §§3.11.2, 7.2, 15.6 | Entities, typed references and JSON properties; no automated scoring |
| M04 | Multiple hazard/top-event combinations within groups | SPEC-014 §§3.4–3.5; SPEC-019 Guiding Principles | Shared hazard has multiple diagram children, one top event per diagram |
| M05 | Escalation factors and their barriers, including nesting | SPEC-013 §3.9; SPEC-014 §3.9 | Typed parents and cycle rejection |
| M06 | Risk cells/categories/coordinates, eight matrices | SPEC-013 §7.12 | Eight limit; explicit cells; no copied default matrix |
| M07 | Custom fields and taxonomy-backed effectiveness | SPEC-013 §§7.4.1, 7.3, 15.2 | Original JSON properties and typed lookup references |
| M08 | Entity deletion replacement versus dangling links | SPEC-013 §§3.11.2, 3.11.3.3 | RESTRICT proposal; explicit detach/reassign workflow later |
| M09 | Short/long/off extra information and null colors | SPEC-013 §§8.1–8.4 | Reserved metadata; no display implementation |
| M10 | Original tree projection and ordering | SPEC-013 §§7.2, 7.9 | SQL recursive tree view; empty React explorer scaffold |
| M11 | Acceptance criteria and revision info | SPEC-013 §§7.13, 7.16 | Properties stored; human flags, no algorithmic safety approval |
| M12 | Improvement justification and management verification | SPEC-004 steps 9–12; SPEC-016 §§6.2.4–6.2.5 | Schema fixture records tasks, ownership, instructions, output, action rationale |
| A01 | Audit question hierarchy, survey subset, targets | SPEC-021 §§24.3, 24.5–24.8; SPEC-023 Prepare audit questions & surveys | Reference tables only; Phase 6 execution/scoring deferred |
| A02 | Ordinal answer scales and contextual answers | SPEC-021 §§24.6.1–24.6.2 | Answers/properties reservation; scoring implementation deferred |
| I01 | Incident records, timeline event/actor structure | SPEC-002 §§3.2–3.4; SPEC-022 §3.3 | Generic incident-node reference contracts; Phase 7 UI deferred |
| I02 | Incident/barrier associations | SPEC-021 §29.2; SPEC-002 ch.10 | Separate incident barriers linked to stable bow-tie barrier IDs |

## Exclusions and limits

Advanced-only rows with an open circle in SPEC-015 are not Standard features. The approved master prompt separately schedules case comparison/find-replace in Phase 5; these are owner-scheduled extensions requiring later phase review, not newly interpreted Standard rows. No such functionality is implemented here. Web/SharePoint/server collaboration services, proprietary legacy file formats, activation, network updates and copied branding are excluded. Secondary references remain preserved; they cannot override primary text.

The manuals do not enumerate every report/quality check, and extracted images are absent. Full parity is not proven by row coverage. AR01–AR33 in `AMBIGUITY-RESOLUTIONS.md` states the chosen interpretations and alternatives. The architecture gate requires review of missing catalog details before the corresponding later phase.
