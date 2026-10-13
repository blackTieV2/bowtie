# AR78 — proposed incident linkage migration

Status: proposal only, NOT approved or applied. Phase 7 is stopped before app/CI implementation. Approved SQL, triggers and cases are unchanged.

## Required behavior and incompatibility

SPEC-002 §§7.1.1 and 8.1.2 define RCA and categorized RCA shapes as events and allow them to link to bow-tie barriers; §10.2 documents assignment of those links. SPEC-022 §7.1.1 and chapter 9 exclude RCA linkage. The human explicitly makes 10.4 authoritative; AR77 records that version difference.

Approved incident_node has event/barrier/actor/evidence kinds. incident_barrier_link references an incident node and a core entity. Both incident_link_insert and incident_link_update require an incident barrier source and core barrier target. incident_identity_immutable forbids changing kind. This contract rejects a genuine RCA event source. Versioned metadata is appropriate for owned properties, but putting this join in metadata or introducing a synthetic barrier node would evade the authoritative link table/trigger.

## Proposed minimal change

Keep incident_barrier_link and its foreign keys/composite identity. Change both link triggers to permit:

1. An incident barrier source, as already approved; or
2. A genuine incident event source belonging to a documented RCA analysis diagram (ordinary RCA or categorized RCA).

Targets must remain core barriers. Keep node ID/kind/incident immutability. Actor/evidence nodes, timeline events and events from other analysis methods must remain rejected. The method discriminator must be validated against the owning analysis diagram, not inferred from its appearance/title. Proposed original method keys are rca and categorized_rca; they are design candidates, not existing approved keys.

Before writing migration SQL, settle the versioned incident/diagram metadata contract for multiple analysis diagrams plus timelines, so method membership is authoritative and cannot be changed to invalidate existing links. The existing incident.method_key can govern a single-method incident, but is insufficient by itself when multiple diagrams use distinct methods. If the approved diagram contract stores method membership in versioned metadata, triggers must validate that discriminator and matching diagram reference; do not merely trust a free-form node method label. Any additional schema change needed for this discriminator must be explicit in the reviewed migration.

## Compatibility and acceptance

- Introduce a new schema version and explicit migration path; never rewrite the approved initial schema silently. Canonical case open currently validates exact schema, so version dispatch/open/save validators must recognize the approved migrated form.
- Migrate a validated backup, preserve original case bytes until successful validation and atomic replacement, and preserve all prior identities/data.
- Existing barrier-to-barrier links remain valid. Existing cases without incident data remain structurally unchanged apart from the approved schema version/trigger definitions.
- Test insert/update with both RCA methods, other-method/timeline/actor/evidence rejection, non-barrier target rejection, kind/ownership immutability, method changes with existing links, foreign-key integrity and save/reopen/CSV roundtrip.
- Fresh read-only validator reviews the exact migration and runs these checks. Approval of this proposal does not constitute Phase 7 acceptance.

Human decision needed: authorize this constrained migration design and continued Phase 7 implementation, or grant an explicit scope exception excluding the 10.4 RCA event-link behavior. No migration, trigger bypass, app code or platform workflow has been written.

## Authorized AR78 implementation

The human has replaced the broader proposal with the exact two-trigger migration in task-packet.md. This section records execution of that constrained design; earlier stop/proposal text remains historical evidence.

Original method keys: rca (ordinary RCA) and categorized_rca (categorized RCA). Only those incident.method_key values enable event-to-barrier links. Barrier sources stay accepted for all methods; actor/evidence remain rejected. The proposed extra diagram discriminator/schema changes are NOT implemented. The existing incident.method_key is authoritative exactly as the human instructed.

Opening v1 validates its original schema first, backs it into memory, migrates that working copy transactionally, and validates the exact v2 signature before replacing the session. Explicit Save performs the established atomic backup/replacement; compare stays read-only and can validate both versions without altering the reference file. Existing new-case construction stays unchanged to preserve the 43 tests unmodified; Phase 7's first incident write will apply the same migration within its transaction on a fresh v1 working session.

Deletion diagnostics still count incident_barrier_link by barrier_id; editing.rs was not changed for this migration. Approved 001-initial.sql remains unchanged.
