# Task Packet — 008-build-phase7

Stage: 04-build. Phase 7 ONLY, final incident build phase. Producer /root owns this mutable tree. Fresh read-only validator authorized under the same five Phase 5/6 constraints.

TARGET GATE: PASS, 2026-10-13. Sole repository root C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie, clean feature/phase-6-audit-module at 4167043354510b5a817574d488f1cafc29f4ba95, expected origin https://github.com/blackTieV2/bowtie.git. No hold, build approved. Created feature/phase-7-incident-module from that tip. Bootstrap status pointers are superseded by explicit current human phase authorization; accepted earlier run records are preserved.

## Human authorization — verbatim

Human review complete: Phase 6 is APPROVED (audit module, storage design, qualitative-only results, recorded-answer immutability; AR70AR76 accepted).

You are authorized to execute Phase 7 ONLY: the INCIDENT MODULE  incident diagrams, timelines, BSCAT/event trees, and links back to bow-tie barriers (per REBUILD-MASTER-PROMPT.md Phase 7). This is the final build phase.

1. TARGET GATE: PASS before any modification. Confirm sole repository root, clean tree, branch feature/phase-6-audit-module at 4167043.
2. Create branch feature/phase-7-incident-module from that tip. Open run packet runs/008-build-phase7/.
3. Authority: SPEC-002 (IncidentXP 10.4) is version-authoritative; SPEC-022 (8.2) is the clean-structure reference. Where they conflict, default to 10.4 and flag via AR. Cite every feature in SPEC-TRACEABILITY.md.
4. Scope:
   a. Incident creation with documented analysis methods (method_key exists in the schema  implement only the methods the spec documents; BSCAT/event-tree behavior exactly as specified, no invented taxonomy).
   b. Incident diagram canvas: event/barrier/actor/evidence nodes with edges per spec, reusing the Konva scene machinery where it fits. Layout and interactions original.
   c. Timeline: the documented timeline behavior only.
   d. Barrier linkage: incident barrier nodes link to bow-tie barriers via incident_barrier_link (the trigger enforces barrierbarrier); surface case-barrier context (effectiveness, audit results where the spec connects them) read-only.
   e. Incident outputs: spec-documented reports through the Phase 4 export machinery.
   f. SCHEMA GUARD (same as Phase 6): use the approved incident tables. Any spec-required behavior that genuinely doesn't fit  STOP with an AR and migration proposal. No silent migration, no trigger bypass. Versioned metadata envelopes where metadata_json is used.
5. PLATFORM DEBT  now due: add a GitHub Actions workflow that builds the app on windows-latest, macos-latest, and ubuntu-latest, runs cargo tests on all three, and uploads the binaries as artifacts. Rule 4 of the master prompt (runs on all three OSes) has been deferred since Phase 2; this phase closes it. Document any platform-specific failures as ARs rather than shipping around them.
6. Standing rules unchanged: no decompilers, no legacy binaries/names/branding, offline-only runtime, parameterized SQL, triggers authoritative, producer  validator. Validation sub-agent re-authorized under the same five constraints.
7. Validation bar: new cargo tests (incident node/edge integrity, identity immutability trigger, barrier-link trigger, method behaviors against worked spec examples, save/reopen equality, incident data survives CSV round-trip). Native GUI script reproduces the incident walkthrough from SPEC-002. CI run evidence from all three platforms in runs/008-build-phase7/.
8. Ambiguities: per D-006, append from AR77.
9. STOP at the boundary with stages/04-build/output/phase-7-report.md. PUSH feature/phase-7-incident-module to origin when done. No merge.

## AR78 migration authorization — verbatim

AR78 RESOLVED  constrained migration AUTHORIZED. Human verification of SPEC-002 confirms: RCA/TOP-SET event nodes must link to bow-tie barriers (ch. 10; lines "link RCA events to bowtie barriers", "or an event if you are using the TOP-SET/RCA method"). The link rule is method-conditional and the migration MUST preserve that.

Authorized migration (nothing beyond this):
1. New file app/src-tauri/schema/002-rca-event-links.sql: PRAGMA user_version = 2; DROP and recreate ONLY incident_link_insert and incident_link_update so a link is valid when the incident node is kind='barrier' (any method, as before) OR kind='event' AND its incident's method_key is in the documented RCA-family set (per SPEC-002: TOP-SET/RCA methods). Event links remain REJECTED for BSCAT and basic/timeline methods; actor/evidence links remain rejected for all methods. No other table, trigger, or index changes.
2. case.rs: open accepts user_version 1 or 2; v1 cases migrate transactionally on open under the existing atomic/backup discipline; validate() accepts the v2 schema signature (CRLF-normalized as before). Schema signature constant updated; no other logic changes.
3. Deletion diagnostics already count incident_barrier_link  confirm unchanged.
4. Tests (all required): a real v1 fixture file opens, migrates to 2, and round-trips; barrier-node links still accepted for every method; event-node links accepted for RCA-family methods and rejected for BSCAT/basic; actor/evidence links rejected; existing 43 tests still pass unmodified.
5. Docs: SPEC-TRACEABILITY.md cites SPEC-002 ch. 10 for the link behavior; AR78 recorded resolved-by-migration with this rationale; AUDIT-DATA-DESIGN/schema docs updated.
6. Then continue Phase 7 implementation (including the CI platform debt) under the existing authorization. Validator re-validates the migration specifically.
7. STOP at the boundary with phase-7-report.md and PUSH feature/phase-7-incident-module to origin (include the migration proposal and stop report in the push). No merge.

Naming caution for incident methods: BSCAT and Tripod Beta are third-party registered marks (DNV / Energy Institute)  use methodology names nominatively only (plain text references), no logos, no reproduced DNV/Energy Institute content. If unsure, label generically ("barrier-based systemic cause analysis") and log an AR.
