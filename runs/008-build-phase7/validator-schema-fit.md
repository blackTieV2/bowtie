Independent Phase 7 schema-fit validation: **STOP / FAIL**.

Starting state verified: expected repository and origin; branch `feature/phase-7-incident-module`; HEAD `4167043354510b5a817574d488f1cafc29f4ba95`. Only the producer’s run008 directory was untracked. No tracked modifications made.

**Blocking defect:** SPEC-002 §§7.1.1, **8.1.2**, and 10.2 require RCA and categorized-RCA **event nodes** to link to bow-tie barriers. Approved `incident_barrier_link` insert/update triggers require an incident **barrier node**. Node-kind immutability prevents conversion. The behavior cannot fit these constraints without migration.

Independent in-memory SQLite checks against `001-initial.sql`:

- Event→bow-tie-barrier insertion rejected: `Links require two barriers`.
- Event→barrier kind conversion rejected by identity trigger.
- Barrier→bow-tie-barrier positive control succeeded.

**Version conflict:** SPEC-022 §7.1.1 prohibits RCA bow-tie linkage; SPEC-002 introduces it from version 9.2. Apply the authoritative 10.4 requirement and record an AR. Omitting linkage, introducing proxy barriers, or storing these relationships in metadata would circumvent the approved relationship contract.

**Migration proposal for human review:** extend both link triggers to accept event nodes belonging to documented RCA methods, retaining barrier target validation, foreign keys, and immutable node identity. Review any corresponding method-key changes and migration compatibility before implementation.

Remaining coverage after approval: incident grouping/multiple diagrams; connector insertion/branching; versioned metadata validation; method-specific causes and category references; timeline columns/actors/cell moves; action/document associations; reports; save/reopen and CSV preservation. Documented methods are timeline, barrier-based systemic analysis, Tripod, BFA, RCA, and categorized RCA. BFA category lists start empty; no proprietary taxonomy should be invented.

No implementation validation, builds, native GUI scripts, or CI ran during the schema stop. The stage-referenced `stages/03-design/output/acceptance-criteria.md` is absent; explicit run008 acceptance requirements remain available.
