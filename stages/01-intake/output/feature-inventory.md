# Standard-edition feature inventory

Run `001-intake`; draft for human review, 2026-10-13 Asia/Singapore. This is a documentation inventory, not an implementation or approval record. All descriptions below are requirements paraphrases. Source titles/row labels identify provenance only.

## Source keys

| Key | Exact spec file | Role and locator convention |
|---|---|---|
| FM | [BowTieXP_Feature_Matrix_2012.md](../references/spec/BowTieXP_Feature_Matrix_2012.md) | Edition matrix: category + named row; no section numbering. |
| M6 | [BowTieXP_Software_Manual_-_For_Release_6.0.md](../references/spec/BowTieXP_Software_Manual_-_For_Release_6.0.md) | Indexed authoritative feature reference; extracted-text integrity unresolved. |
| CMP | [02 Compare BowTieXP Standard & Advanced.md](../references/spec/02%20Compare%20BowTieXP%20Standard%20&%20Advanced.md) | Additional edition table; headings GENERAL/NAVIGATION/SPECIAL ADVANCED ONLY FUNCTIONALITY. |
| M8 | [08 BowTieXP Software Manual Rev 31 for Ver 8.2.md](../references/spec/08%20BowTieXP%20Software%20Manual%20Rev%2031%20for%20Ver%208.2.md) | Additional manual; chapter/section numbering differs from M6. |
| M8-copy | [Software User Manual Rev 31 for Ver 8.2 & Up.md](../references/spec/Software%20User%20Manual%20Rev%2031%20for%20Ver%208.2%20&%20Up.md) | Near-duplicate supplemental manual; two text differences. |
| QS3 | [BowTieXP_3.6_and_up_Quick_Start_Manual.md](../references/spec/BowTieXP_3.6_and_up_Quick_Start_Manual.md) | Indexed acceptance workflows. |
| QS8 | [07 BowTieXP 8.0 and up Quick Start Manual.md](../references/spec/07%20BowTieXP%208.0%20and%20up%20Quick%20Start%20Manual.md) | Additional acceptance workflows; broken references present. |
| STEPS | [BowTieXP_in_12_steps.md](../references/spec/BowTieXP_in_12_steps.md) | Primary acceptance walkthrough; steps 1–12 on original PDF pages 1–4. |
| ADV | [BowTieXP_Advanced_Brochure_[small].md](../references/spec/BowTieXP_Advanced_Brochure_[small].md) | Named/numbered Advanced features; boundary evidence. |
| MGMT | [BowtieXP_Management_document.md](../references/spec/BowtieXP_Management_document.md) | Standard Features / Advanced / add-ons; supporting scope evidence. |

## Indexed matrix: every Standard-marked row

Each F row represents one row with a checkmark in the Standard column of FM. Navigator/Advanced/add-on marks do not authorize Standard scope. Rows with a conflict remain pending human disposition.

| ID | Feature / observable capability | Spec file key + section / row | Qualification / ambiguity |
|---|---|---|---|
| F01 | Import entities — basic | FM DATA MANAGEMENT; M6 §11.2.1; M8 §12.2.1 | Disputed links/configuration coverage; A05/A14 |
| F02 | Move entities while retaining links | FM DATA MANAGEMENT (Mark for Move); M6 §7.7 | Relationship semantics A22 |
| F03 | Text find and navigate to results | FM DATA MANAGEMENT; M6 §7.12; M8 §9.4 | Contextual search/replacement conflict A08 |
| F04 | Barrier colors by metadata | FM MODELING; M6 §§7.4.2, 8.2.2, 8.3 | Effectiveness, type, criticality; A10 |
| F05 | Barrier criticality indicator | FM MODELING; M6 §§3.11.2, 8.1–8.3 | User-defined categories/colors |
| F06 | Extra information below shapes | FM MODELING; M6 §§8.1, 8.2.1 | Short/long/off; metadata and linked items |
| F07 | Display inherent/residual risk | FM MODELING; M6 §§7.11, 8.2.1 | Top-event/consequence assessments; A04 |
| F08 | New-case wizard | FM MODELING; M6 §§3.3, 6.4.1 | New diagram versus new file; template selection |
| F09 | Actions on elements | FM MODELING; M6 §§3.12.4–3.12.6 | One-off actions, priority/status, ownership/sign-off |
| F10 | Selective visibility | FM MODELING; M6 §7.4.4.1 | Off, edit/show, on, reset, filtered snapshot |
| F11 | Reference filter | FM MODELING; M6 §7.4.4.2 | Referenced entities/null values, retained ancestor context |
| F12 | Usage references/backlinks | FM NAVIGATION; M6 §§3.11.4, 7.8.2 | Reverse lookup of links and lookup values |
| F13 | Navigate to entity definition | FM NAVIGATION; M6 §§3.11.3.1, 7.4, 9.1 | Synchronize tree/diagram selection |
| F14 | Navigation back/forward | FM NAVIGATION; M6 §6.9.1 | Selection history separate from undo |
| F15 | Personal window layout | FM USER INTERFACE; M6 §6.7 | Dock/float/resize/autohide, save/load/reset |
| F16 | Two switchable layouts | FM USER INTERFACE; M6 §6.7 | Layout A/B |
| F17 | Fonts | FM USER INTERFACE; M6 §§7.4.3, 8.2.3 | Global defaults and individual shapes |
| F18 | Color legend | FM USER INTERFACE; M6 §12.7 | Show and copy legend |
| F19 | Expand/collapse or widen treeview | FM USER INTERFACE; M6 §§7.2, 6.9.1 | Tree expansion and width are distinct controls |
| F20 | Configure risk matrices | FM CONFIGURING; M6 §7.11; M8 §7.12 | Axes, cells, categories, codes, colors; six/eight conflict A04 |
| F21 | Configure lookup tables | FM CONFIGURING; M6 §3.11.2; M8 §7.3 | Add/edit/delete/order values; replacement when used; A10/A22 |
| F22 | Reports | FM REPORTING; M6 §§12.2.1–12.2.2; M8 §13.2 | Word-compatible/spreadsheet outputs; catalog incomplete A11/A15 |
| F23 | Competency report | FM REPORTING; M6 §12.2.2 (Competencies and Lists) | Ordinary list versus Advanced operation report A11 |
| F24 | Export tree/list/quality data to spreadsheets | FM REPORTING; M6 §§12.3–12.6 | Case-overview export also documented; A15 |
| F25 | Save diagram as image | FM DIAGRAM; M6 §7.4.5; M8 §13.4 | PNG/TIFF/JPEG/BMP in matrix; exact target formats A15 |
| F26 | Copy diagram to clipboard | FM DIAGRAM; M6 §§7.4.5, 12.7; M8 §13.4 | Vector clipboard in later docs; A15 |
| F27 | Export diagram for external diagram editor | FM DIAGRAM (Export to Visio); M8 §13.4 | Host-dependent export/limitations; A15 |
| F28 | Print diagram | FM DIAGRAM; M8 §§13.6.1–13.6.2 | Direct printing and clipboard route; A16 |
| F29 | Order poster | FM DIAGRAM | Listed without detailed contract; offline/service conflict A16 |
| F30 | Diagram headers/footers | FM DIAGRAM; M6 §25.3.4 (layout-transfer checklist) | Creation/printing listed but details missing A16 |
| F31 | Expand/collapse diagram | FM DIAGRAM; M6 §7.4.1 | Individual nodes and global levels |
| F32 | Diagram overview window | FM DIAGRAM; M6 §7.5 | Navigate miniature diagram viewport |
| F33 | Spellcheck | FM OTHER; CMP GENERAL (Spelling check) | Preset/custom dictionaries; no complete behavior A25 |
| F34 | Import incident fact list | FM OTHER (Import of BlackBox Incident investigation fact list) | Listed Standard; file evidence/boundary missing A14 |
| F35 | Case overview | FM OTHER; M6 §§9.1.1, 9.1.4, 9.1.7–9.1.12 | Classic, job titles, documents, barrier/threat/factor/consequence registers, actions |
| F36 | Enhanced management-system overview | FM OTHER; M6 §§5.2.7.1, 9.1.2 | Matrix Standard versus manual Advanced A09 |
| F37 | Quality checks | FM OTHER; M6 §§7.13, 9.2 | Select check, inspect/refresh/export; catalog missing A12 |
| F38 | Document links | FM OTHER; M6 §§3.11.3.4, 10.1–10.2 | Central definitions, assignment, open, drag/drop deduplication; A13 |
| F39 | Offline document links | FM OTHER; M6 §10.3; ADV features 16/19 | Relative paths versus collection/indexing boundary A13 |
| F40 | Case file properties/information | FM OTHER; CMP GENERAL (Case File Properties) | Purpose/history/author/opening message; exact fields A10 |
| F41 | Mail to case author | FM OTHER; M6 §6.9.1 | External mail action; offline boundary A25 |
| F42 | Copy/paste items and subtrees | FM OTHER; M6 §7.7 | Within-case copy/cut/paste/paste-special; A22 |
| F43 | Drag/drop links and entities | FM OTHER; M6 §§3.8, 3.11.3.1, 7.9 | Reorder and assign links; case boundaries A22 |
| F44 | Undo/redo | FM OTHER; M6 §6.5 | Unlimited history stated; persistence/granularity A21 |

## Manual and supplemental Standard behaviors

S rows retain requirements not separately enumerated by FM, plus later-version/unmarked candidates needed to avoid silently losing scope. Server-oriented or edition-conflicting rows are candidates, not settled Standard requirements. SC criteria in the intake brief cover the accepted subset.

| ID | Feature / observable capability | Spec file key + section | Qualification / ambiguity |
|---|---|---|---|
| S01 | Create/edit complete bow-tie structure | M6 §§3.3–3.10; QS3 §§3.3–3.9; M8 §§3.3–3.10 | Hazard/top event, threats, consequences, both barrier sides, escalation factors/barriers; A18/A23 |
| S02 | Groups/locations and hierarchy | M6 §§3.4–3.5, 7.2; M8 §§3.4–3.5; CMP GENERAL | Newer grouping versus locations; A18 |
| S03 | UI language selection | M6 §6.3; CMP GENERAL | Listed eleven UI languages; original translations A17 |
| S04 | Local file lifecycle and unsaved changes | M6 §§6.2.2, 6.4.1, 6.4.3 | New/open/save/save-as/close and save/discard prompt |
| S05 | Automatic backup/recovery | M6 §6.4.4 | Separate backup every ten minutes; A21 |
| S06 | Templates | M6 §§13.1–13.7; CMP GENERAL (Create Template) | Original local case templates; remote template configuration A25 |
| S07 | Activity hierarchy and barrier links | M6 §§3.11.3, 7.2; QS3 §3.10.3 | Tasks, frequency, responsibility, documents, objectives and related metadata; A10/A19 |
| S08 | Systems, competencies, inputs/outputs | M6 §§3.11.3.4, 7.2; QS3 §3.10.3.4 | Linkable management records; version-dependent products A10 |
| S09 | Popup/docked editing and link assignment | M6 §§7.3, 7.3.2, 3.11.3.2 | Commit/revert; selection commits docked edit; hierarchical versus flat links |
| S10 | Lists and multiple selection | M6 §§7.8–7.8.2 | Child lists, ctrl/shift selection, sort/filter assignment lists, export |
| S11 | Zoom/pan and synchronized navigation | M6 §§7.4, 7.5; CMP NAVIGATION | Fit/100%, scrolling and viewport; input/platform tolerance A26 |
| S12 | Diagram sizing/alignment/compaction | M6 §8.2.4; M8 §8.2.4 | Fixed/dynamic sizing, connectors, alignment; exact layout A23 |
| S13 | Unassigned lookup colors | M6 §8.4 | Per-property null colors |
| S14 | Preset terminology | CMP GENERAL (Apply Basic Terminology); M6 §17.2 | Use supplied terminology; editing is Advanced; original wording A17 |
| S15 | Data-entry suggestions | CMP GENERAL (Auto detect data entry) | Prior matching entries suggested; no complete matching contract A25 |
| S16 | Acceptance-criteria text and met flags | CMP GENERAL (Acceptance Criteria); M8 §7.13 | Group-level text/per-element manual flags and quality checks; A12 |
| S17 | Diagram revision information | CMP GENERAL (Revision Info); M8 §7.16 | Hazard revision block alignment, text, image inclusion |
| S18 | Check for updates | CMP GENERAL (Check for Updates); M6 §§25.2.17–25.2.18 | Offline/vendor boundary A25 |
| S19 | Import from another legacy risk tool | CMP GENERAL; M6 §6.4.2 | Versions 3/4; incomplete import caveat; clean-room boundary A14 |
| S20 | Select existing display profile | CMP NAVIGATION (Choose a display profile); M6 §7.4; M8 §17.1 | Standard versus Advanced conflict A07 |
| S21 | Treeview filter profiles | CMP GENERAL (Treeview Filtering); M6 §§5.2.5, 7.10 | Creation/use edition conflict A06 |
| S22 | Custom user fields | M6 §7.3.1; M8 §15.2; FM MODELING | Generic user data versus add-on field scope A24 |
| S23 | Contextual/wildcard search | M6 §9.3; FM BOWTIE VISTA; ADV feature 14 | Edition conflict A08 |
| S24 | Replace search results | M8 §9.4.1; M6 §7.12.1; FM DATA MANAGEMENT | Later manual unmarked, older sources Advanced-only; A08 |
| S25 | Organizational units and plot plans | M8 §§23.2–23.3 | Standalone capability/edition not settled; A27 |
| S26 | Local barrier performance data | M8 §23.5 | Per-unit actual effectiveness; A27 |
| S27 | Formatted descriptions and wiki pages | M8 §§23.6–23.7; QS8 §3.11.3.4 | Standalone/server ambiguity; attachments not fully specified A27 |
| S28 | Advanced reports marked Standard in comparison | CMP GENERAL (Reports (Advanced)); M6 §5.2.7.2 | Contradictory row; retain for review A11 |
| S29 | Threat/consequence/escalation-factor colors | CMP GENERAL (Colour settings for bowtie elements); M6 §§3.11.2, 8.1–8.3 | Color by taxonomy/property, distinct from barrier color |

## Later comparison: every Standard-marked row crosswalk

CMP has 51 Standard-marked rows. Duplicate capabilities map to F/S entries; conflicting rows remain included as disputed evidence. Its extraction embeds “Show Extra Information” inside the screen-layout row (mapped to F06), and splits spreadsheet export between the personal-layout description and the following row (mapped to F24). The split labels below identify the extraction; they do not imply extra features.

| ID | CMP section + row label | Inventory mapping | Manual/spec locator |
|---|---|---|---|
| C01 | CMP GENERAL: Create and Edit Bowtie Diagrams | S01 | M6 §§3.3–3.10; QS3 §§3.3–3.9; M8 §§3.3–3.10 |
| C02 | CMP GENERAL: Bowtie Groups | S02 | M6 §§3.4–3.5, 7.2; M8 §§3.4–3.5; CMP GENERAL |
| C03 | CMP GENERAL: Change Language | S03 | M6 §6.3; CMP GENERAL |
| C04 | CMP GENERAL: Risk Matrices (Create & Edit) | F20 | FM CONFIGURING; M6 §7.11; M8 §7.12 |
| C05 | CMP GENERAL: Action Register | F09 | FM MODELING; M6 §§3.12.4–3.12.6 |
| C06 | CMP GENERAL: Lookup Tables (Basic) | F21 | FM CONFIGURING; M6 §3.11.2; M8 §7.3 |
| C07 | CMP GENERAL: Import Entities (Basic) | F01 | FM DATA MANAGEMENT; M6 §11.2.1; M8 §12.2.1 |
| C08 | CMP GENERAL: Copy, Cut, Paste & Paste Special | F42 | FM OTHER; M6 §7.7 |
| C09 | CMP GENERAL: Colour Settings for Barriers/Controls | F04 | FM MODELING; M6 §§7.4.2, 8.2.2, 8.3 |
| C10 | CMP GENERAL: (Colour settings for bowtie elements) | S29 | CMP GENERAL (Colour settings for bowtie elements); M6 §§3.11.2, 8.1–8.3 |
| C11 | CMP GENERAL: Screen layout (user defined) | F15/F06 | FM USER INTERFACE; M6 §6.7; FM MODELING; M6 §§8.1, 8.2.1 |
| C12 | CMP GENERAL: Saving diagrams as Images | F25 | FM DIAGRAM; M6 §7.4.5; M8 §13.4 |
| C13 | CMP GENERAL: Copy to clipboard | F26 | FM DIAGRAM; M6 §§7.4.5, 12.7; M8 §13.4 |
| C14 | CMP GENERAL: Export to Visio | F27 | FM DIAGRAM (Export to Visio); M8 §13.4 |
| C15 | CMP GENERAL: Printing  Diagrams | F28 | FM DIAGRAM; M8 §§13.6.1–13.6.2 |
| C16 | CMP GENERAL: Changing fonts | F17 | FM USER INTERFACE; M6 §§7.4.3, 8.2.3 |
| C17 | CMP GENERAL: Colour legend | F18 | FM USER INTERFACE; M6 §12.7 |
| C18 | CMP GENERAL: Personal layout | F15 | FM USER INTERFACE; M6 §6.7 |
| C19 | CMP GENERAL: checks etc., to Excel | F24 | FM REPORTING; M6 §§12.3–12.6 |
| C20 | CMP GENERAL: Create Template | S06 | M6 §§13.1–13.7; CMP GENERAL (Create Template) |
| C21 | CMP GENERAL: Treeview | F19/S02 | FM USER INTERFACE; M6 §§7.2, 6.9.1; M6 §§3.4–3.5, 7.2; M8 §§3.4–3.5; CMP GENERAL |
| C22 | CMP GENERAL: Treeview Filtering | S21 | CMP GENERAL (Treeview Filtering); M6 §§5.2.5, 7.10 |
| C23 | CMP GENERAL: Selective Visibility Filters | F10 | FM MODELING; M6 §7.4.4.1 |
| C24 | CMP GENERAL: Advanced Filters (1 Criteria) | F11 | FM MODELING; M6 §7.4.4.2 |
| C25 | CMP GENERAL: Apply Basic Terminology | S14 | CMP GENERAL (Apply Basic Terminology); M6 §17.2 |
| C26 | CMP GENERAL: Spelling check | F33 | FM OTHER; CMP GENERAL (Spelling check) |
| C27 | CMP GENERAL: Search & Find | F03 | FM DATA MANAGEMENT; M6 §7.12; M8 §9.4 |
| C28 | CMP GENERAL: Import data from Thesis (legacy import) | S19 | CMP GENERAL; M6 §6.4.2 |
| C29 | CMP GENERAL: Quality Checks | F37 | FM OTHER; M6 §§7.13, 9.2 |
| C30 | CMP GENERAL: Document Links | F38 | FM OTHER; M6 §§3.11.3.4, 10.1–10.2 |
| C31 | CMP GENERAL: Export Treeview data & Diagram data | F24 | FM REPORTING; M6 §§12.3–12.6 |
| C32 | CMP GENERAL: Reports (Basic) | F22 | FM REPORTING; M6 §§12.2.1–12.2.2; M8 §13.2 |
| C33 | CMP GENERAL: Reports (Advanced) | S28 | CMP GENERAL (Reports (Advanced)); M6 §5.2.7.2 |
| C34 | CMP GENERAL: Auto detect data entry | S15 | CMP GENERAL (Auto detect data entry) |
| C35 | CMP GENERAL: Search for data in the file | F03 | FM DATA MANAGEMENT; M6 §7.12; M8 §9.4 |
| C36 | CMP GENERAL: Case File Properties | F40 | FM OTHER; CMP GENERAL (Case File Properties) |
| C37 | CMP GENERAL: Acceptance Criteria | S16 | CMP GENERAL (Acceptance Criteria); M8 §7.13 |
| C38 | CMP GENERAL: Actions | F09 | FM MODELING; M6 §§3.12.4–3.12.6 |
| C39 | CMP GENERAL: Revision Info | S17 | CMP GENERAL (Revision Info); M8 §7.16 |
| C40 | CMP GENERAL: Check for Updates | S18 | CMP GENERAL (Check for Updates); M6 §§25.2.17–25.2.18 |
| C41 | CMP NAVIGATION: Backlinks | F12 | FM NAVIGATION; M6 §§3.11.4, 7.8.2 |
| C42 | CMP NAVIGATION: Go To Definition | F13 | FM NAVIGATION; M6 §§3.11.3.1, 7.4, 9.1 |
| C43 | CMP NAVIGATION: Undo and Redo | F44 | FM OTHER; M6 §6.5 |
| C44 | CMP NAVIGATION: Back and Forward | F14 | FM NAVIGATION; M6 §6.9.1 |
| C45 | CMP NAVIGATION: Zoom in/out | S11 | M6 §§7.4, 7.5; CMP NAVIGATION |
| C46 | CMP NAVIGATION: Stage diagrams (Expand & Collapse) | F31 | FM DIAGRAM; M6 §7.4.1 |
| C47 | CMP NAVIGATION: Display Risk Assessments | F07 | FM MODELING; M6 §§7.11, 8.2.1 |
| C48 | CMP NAVIGATION: Choose a display profile | S20 | CMP NAVIGATION (Choose a display profile); M6 §7.4; M8 §17.1 |
| C49 | CMP NAVIGATION: Drag and Drop | F43 | FM OTHER; M6 §§3.8, 3.11.3.1, 7.9 |
| C50 | CMP NAVIGATION: Change display options  (the eye icon) | F06 | FM MODELING; M6 §§8.1, 8.2.1 |
| C51 | CMP NAVIGATION: Maximise / Minimise Treeview | F19 | FM USER INTERFACE; M6 §§7.2, 6.9.1 |

## Documented R6 Standard report entries

These are the entries not marked Advanced/add-on in M6 §12.2.2; the manual explicitly says this catalog is incomplete. Every entry cites M6 §12.2.2 and its subsection heading. Exact original report titles below are source locators; shipped wording/layout must be original. A11 governs final catalog/edition selection, and A15 governs output formats. Later action-report additions are listed separately below.

| ID | Report source locator | Spec file + section |
|---|---|---|
| R01 | Actions by Action party | M6 §12.2.2 — Actions |
| R02 | Actions by Priority and Action party | M6 §12.2.2 — Actions |
| R03 | Actions of a specific Action party | M6 §12.2.2 — Actions |
| R04 | All Actions | M6 §12.2.2 — Actions |
| R05 | Activities by single Activity Responsible | M6 §12.2.2 — Activities |
| R06 | Activities grouped by Activities.Responsible of a single Hazard | M6 §12.2.2 — Activities |
| R07 | Activities grouped by Activities.Responsible of a single Hazard (Alt Format) | M6 §12.2.2 — Activities |
| R08 | Activities grouped by Activity Responsible | M6 §12.2.2 — Activities |
| R09 | Activities grouped by Hazard and Activities.Responsible | M6 §12.2.2 — Activities |
| R10 | Activities grouped by Hazard and Activities.Responsible (Alt Format) | M6 §12.2.2 — Activities |
| R11 | Activities linked to Barriers ordered by Activity tree order. | M6 §12.2.2 — Activities |
| R12 | Activities Responsible context overview | M6 §12.2.2 — Activities |
| R13 | All Activities | M6 §12.2.2 — Activities |
| R14 | Barrier Type count of a Location | M6 §12.2.2 — Barrier Types |
| R15 | Barrier Accountable context overview | M6 §12.2.2 — Barriers |
| R16 | Barrier Accountable context overview of a specific Hazard | M6 §12.2.2 — Barriers |
| R17 | Barrier register | M6 §12.2.2 — Barriers |
| R18 | Traditional Hazard register showing Document Links | M6 §12.2.2 — Document Links |
| R19 | Escalation Factors sorted by Barrier Type | M6 §12.2.2 — Escalation Factors |
| R20 | Escalation Factors with Barriers of type... | M6 §12.2.2 — Escalation Factors |
| R21 | Escalation Factors without Barriers | M6 §12.2.2 — Escalation Factors |
| R22 | Report of a specific Hazard | M6 §12.2.2 — Hazards |
| R23 | Report of a specific Hazard, long version | M6 §12.2.2 — Hazards |
| R24 | Report of all Hazards | M6 §12.2.2 — Hazards |
| R25 | Short summary list of Hazards | M6 §12.2.2 — Hazards |
| R26 | Traditional Hazard register | M6 §12.2.2 — Hazards |
| R27 | Activities | M6 §12.2.2 — Lists |
| R28 | Competencies | M6 §12.2.2 — Lists |
| R29 | Barriers | M6 §12.2.2 — Lists |
| R30 | Document Links | M6 §12.2.2 — Lists |
| R31 | Inputs and Outputs | M6 §12.2.2 — Lists |
| R32 | Job Titles | M6 §12.2.2 — Lists |
| R33 | Systems | M6 §12.2.2 — Lists |
| R34 | Threats and Consequences | M6 §12.2.2 — Lists |
| R35 | Contents of Case File overview | M6 §12.2.2 — Other |
| R36 | Report on Barrier BRF Codes | M6 §12.2.2 — Other |
| R37 | Consequences ordered by Risk Assessment | M6 §12.2.2 — Risk Assessment |
| R38 | Report Risk Assessment Matrices | M6 §12.2.2 — Risk Assessment |
| R39 | Barrier Systems Register | M6 §12.2.2 — Systems |
| R40 | Systems linked to Barriers Ordered by System lookup table order | M6 §12.2.2 — Systems |

| Additional documented action-report variant | Spec file + section |
|---|---|
| Flat action list in spreadsheet output | QS8 §3.12.6; M8 §3.12.6 |
| Actions with bow-tie context and empty completion lines | QS8 §3.12.6; M8 §3.12.6 |
| Condensed actions in bow-tie context | QS8 §3.12.6; M8 §3.12.6 |
| Actions for selected hazards | QS8 §3.12.6; M8 §3.12.6 |

## Explicit boundaries

- Advanced-only: full reference-resolving import, external spreadsheet import, scrapbook, cross-file clipboard transfer, compare, group editing, custom type/property authoring, multi-criterion filters, relationship/chained diagrams, SOOB/operational-risk modeling, live equipment states, and barrier bundling. Evidence: M6 §5.2, M8 §5.3, CMP SPECIAL ADVANCED ONLY FUNCTIONALITY, ADV numbered features. Conflicting profile/filter/search/report rows remain questions.
- Spreadsheet formula/user-data integration is an add-on in FM MODELING and M6 chapters 21–22; generic user fields remain disputed (S22/A24).
- Incident analysis/timeline/evidence mappings, surveys/audits, server/SharePoint/portal functions, and incident-management portals are deferred or excluded under current v1 scope. Evidence: README roles, M6 §§5.3–5.5 and chapters 19–23; M8 chapters 23–29; module brochures/manuals. A27 distinguishes local server-oriented candidates from excluded services.
- Diagram screenshots and promotional claims do not specify hidden layout algorithms, complete data schemas, file encodings, exhaustive report/check catalogs, or automatic safety judgments. Missing details remain A questions.

## Coverage statement and limits

This inventory covers 44 Standard-marked FM rows, all 51 Standard-marked CMP rows, 29 supplementary/manual behavior rows, and 40 non-Advanced/non-incident report entries explicitly enumerated in M6 §12.2.2. It also captures four later action-report variants. The twelve-step acceptance workflow is mapped in the intake brief. This is exhaustive against the enumerated edition rows, not a claim of exhaustive software behavior: FM says not all features are shown, the report catalog is incomplete, extracts omit images and some references fail. Source verification, version precedence, final Standard scope, and missing details require human dispositions before approval.
