# SQLite schema proposal — Phase 1 architecture gate

Artifact: `app/src-tauri/schema/001-initial.sql`. Original schema; one case is one `.sqlite` file. No legacy file compatibility. D-006/rule 5 requires review before runtime case commands.

`case_info` holds singleton file metadata; `PRAGMA user_version=1` identifies the revision. Every future connection must enable `PRAGMA foreign_keys=ON`. The initial script applies atomically; it is not an idempotent migration runner. The shell creates no application database.

`entity` provides stable identity, required title, kind, description, type_key, effectiveness lookup, order, and object JSON metadata. Planned runtime IDs are opaque unique IDs. One organization roots the case. Groups represent locations or categories and can nest. A hazard belongs to a group; diagram children represent hazard/top-event combinations. At most one top event per diagram; runtime creation must insert the diagram and top event transactionally. Multiple diagrams can share one hazard. Threat/consequence paths contain barriers; barriers own escalation factors and further barriers. SQL rejects invalid parents and cycles.

Activities, documents, systems, competencies, job titles and products are central definitions. Typed links preserve responsibility, supporting work, document references, system membership, competency, inputs/outputs and action ownership/sign-off. Actions are one-off; activities recur. Optional rich fields use JSON properties. Immutable identities and RESTRICT deletion protect references; runtime deletion must explicitly detach or reassign in a transaction.

Risk matrices have explicit axes, coordinates, ranked categories, cells and inherent/residual assignments. Composite foreign keys reject cross-matrix mistakes. Eight matrices maximum; only top events/consequences can be assessed. No ALARP or effectiveness-to-probability computation is inferred. No legacy default matrix/dictionary is shipped.

Audit tables reserve hierarchy, survey subsets and barrier/activity/document targets. General questions can have no target. Ordered JSON answer lists reserve data; runtime contextual answering/scoring and full answer validation belong to Phase 6. Incident records/nodes/edges preserve separate identities and barrier joins; composite keys reject cross-incident edges. Timeline/causation/history behavior belongs to Phase 7. Basic links can use core barrier IDs without redefining them; no promise of zero future migrations for all module features.

`case_tree` is a recursive data projection, not final GUI layout. The shell has empty expandable categories and is not connected to SQLite. It cannot create/open/save cases at this gate. The persistence fixture exercises SQLite directly.

`scripts/check_phase1_schema.py` maps twelve steps to original fixture data, saves/reopens a temporary file and exercises negative integrity cases. GUI acceptance and independent validation remain pending.

Technical references: [SQLite foreign keys](https://www.sqlite.org/foreignkeys.html), [Tauri capabilities](https://v2.tauri.app/security/capabilities/), [Tauri Vite setup](https://v2.tauri.app/start/frontend/vite/), [rusqlite 0.37](https://docs.rs/rusqlite/0.37.0/rusqlite/). Behavioral citations and decisions appear in the inventory and ambiguity log.

## Approved runtime continuation

The owner approved this design and authorized remaining Phase 1 runtime work. The approved initial SQL is unchanged. `src/case.rs` enables foreign keys on every connection; opaque UUID v4 IDs populate the entity tree. Cases stay in memory until explicit Save. Open validates version, exact schema objects, application format metadata, integrity and references before replacing the session. Save writes through the SQLite backup API to a same-directory temporary file, validates/synchronizes it, closes the destination connection, then replaces the selected `.sqlite` file. Unrelated SQLite databases cannot be overwritten. Diagram/top-event insertion is transactional. The earlier empty-shell paragraphs describe the first approved gate; the runtime service now supersedes that implementation status, not the schema design. Canvas work remains Phase 2 and has not started.

## Human-authorized schema v2 compatibility

AR78 resolved: SPEC-002 ch.10 requires ordinary/categorized RCA event sources to link to bow-tie barriers. 002-rca-event-links.sql sets user_version=2 and replaces only incident_link_insert/update. Existing barrier sources remain valid for every method; event sources require incident.method_key rca/categorized_rca; actor/evidence remain invalid. Foreign keys and immutable identity are unchanged. No additional table/trigger/index changes are authorized. v1 opens migrate a validated in-memory backup transactionally; explicit Save persists through the existing atomic replacement. Audit storage is unchanged.
