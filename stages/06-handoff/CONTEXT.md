# 06-handoff — Stage Contract

**Purpose:** record accepted current state so a fresh agent can continue safely.

## Must record (handoff note in `output/` + PROJECT_STATUS.md update)
- accepted current state, completed work, evidence links
- decisions (with decision-record refs), known issues, active holds
- branch / HEAD / version, working-tree state
- approved vs prohibited actions
- next safe action, exact files to read

## Rules
- Do not copy entire conversations into the handoff.
- A handoff is useful context; it never outranks later verified live state.
- Propose memory candidates for reusable knowledge; promotion follows `_config/memory-policy.yaml`.
