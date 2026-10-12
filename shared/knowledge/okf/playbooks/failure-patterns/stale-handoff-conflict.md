# Failure pattern: stale-handoff-conflict

- **Symptom:** handoff claims state that live verification contradicts.
- **Why dangerous:** stale state drives wrong actions; "claimed done" work may not exist.
- **Required behavior:** verify live target state; mark the handoff stale; surface the conflict.
- **Prevention:** handoffs carry timestamps; live state outranks handoffs (authority rank 2 vs 8).
- **Validation:** golden-set scenario `stale-handoff`.
