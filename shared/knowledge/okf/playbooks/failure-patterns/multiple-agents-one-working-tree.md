# Failure pattern: multiple-agents-one-working-tree

- **Symptom:** two modifying agents operate on the same tree simultaneously.
- **Why dangerous:** interleaved writes, lost work, unexplainable diffs.
- **Required behavior:** single-writer rule; other agents read-only or isolated in separate branches/worktrees.
- **Prevention:** ownership recorded in the task packet; collision = stop condition.
- **Validation:** ownership field present in every run packet.
