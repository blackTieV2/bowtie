# Failure pattern: dirty-tree-collision

- **Symptom:** unexpected uncommitted changes appear in the working tree mid-task.
- **Why dangerous:** may be another agent's or the human's in-flight work; overwriting it destroys evidence/work.
- **Required behavior:** STOP MODIFYING, preserve state, read-only recon, report the collision.
- **Prevention:** single-writer rule; recon before change; git status in every task packet.
- **Validation:** golden-set scenario `dirty-tree-collision`.
