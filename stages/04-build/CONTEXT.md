# 04-build — Stage Contract

**Purpose:** implement approved scope only.

## Gate (all required before any modification)
```
TARGET GATE: PASS
current_stage == 04-build
execution_hold == false
build_approved == true
human-approved scope documented in runs/<run-id>/task-packet.md
```

## Rules
- Follow `references/REBUILD-MASTER-PROMPT.md` phases strictly; stop for human review at each phase boundary.
- Every feature cites its spec section in `SPEC-TRACEABILITY.md`.
- Original code only; no legacy code, strings, assets, branding, or file formats.
- App must build and run offline on Windows, macOS, Linux at every phase boundary.
- Run the acceptance tests from `stages/03-design/output/acceptance-criteria.md`; report commands actually run and results.

## Completion claims
`implemented ≠ parsed ≠ tested ≠ validated ≠ approved ≠ deployed`. Report each separately.
