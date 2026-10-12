# AR45 — risk assessment target review

Current Phase 3 instruction §3b requests inherent/residual assessments on **threats/consequences**. SPEC-013 §7.12, the approved master model, and the approved v1 schema target **top events/consequences**. The schema has both insert and update triggers enforcing those kinds. The manual does not strictly require threat assessments.

## Option A — spec-aligned scope correction (recommended)

Explicitly authorize inherent/residual assessment assignment to **top events and consequences**, replacing "threats" in §3b. Keep the approved SQL and format version unchanged. Implement the remainder of Phase 3 as requested. Threat frequency remains a typed threat property per SPEC-013's Fields on Threats subsection after §15.4; it is not a substitute risk_assessment row.

## Option B — explicitly authorized behavioral extension and migration

Retain threat assessments as a human-directed extension, while keeping existing top-event and consequence cases valid. The required schema delta is confined initially to both trigger predicates:

```diff
- kind IN ('top_event','consequence')
+ kind IN ('top_event','consequence','threat')
```

Both `risk_target_insert` and `risk_target_update` would need replacement in a versioned migration; changing only IPC checks cannot satisfy authoritative triggers. Existing risk table columns/composite references and the eight-matrix trigger can remain unchanged. No automatic ALARP computation is proposed.

Compatibility work also requires review: the current loader accepts only user_version=1 and compares the entire SQL schema, including trigger bodies. Editing the initial SQL alone would make previously saved Phase 1/2 cases fail that comparison. A safe implementation proposal would preserve the v1 signature, validate old cases before conversion, apply a transactional v1→v2 upgrade to the in-memory working copy, use matching version/format metadata and v2 schema validation, and write upgraded bytes only on explicit Save. Failed upgrades must leave the original file/session intact; old data must remain readable by the new loader. The Phase 2 app would not read new v2 files. Destination validation must recognize supported old/new formats without overwriting unrelated files.

Required migration evidence, if approved: v1 create/save/open and v2 round-trip fixtures; existing assessment preservation; threat insert/update acceptance; rejection of all other entity kinds; failure-injected migration rollback; exact schema/version validation and unchanged eight-matrix/FK enforcement. This is a proposal only: no SQL, loader or runtime change has been made.

The current instruction does not authorize this extension automatically: it allows migration only where the spec strictly requires one, and explicitly requires migration proposals to stop for review. Human must choose Option A or explicitly approve Option B and its compatibility scope before implementation resumes.
