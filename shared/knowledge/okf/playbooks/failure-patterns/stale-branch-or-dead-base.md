# Failure pattern: stale-branch-or-dead-base

- **Symptom:** agent builds on a branch that no longer exists or is far behind, because a handoff said so.
- **Why dangerous:** work lands on dead history; merges become destructive or impossible.
- **Required behavior:** verify live branches/HEAD before any work; treat handoffs as historical context.
- **Prevention:** recon-before-change; TARGET GATE; handoff format includes branch/HEAD with timestamp.
- **Validation:** golden-set scenario `stale-branch`.
