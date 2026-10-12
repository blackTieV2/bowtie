# Failure pattern: false-completion-claim

- **Symptom:** "done/fixed/tested" reported without evidence.
- **Why dangerous:** downstream agents and humans trust phantom work.
- **Required behavior:** completion reports list commands/tests actually run, results, files changed, remaining risks. Separate implemented/parsed/tested/validated/approved/deployed.
- **Prevention:** constitution §43 report format enforced in stage contracts.
- **Validation:** golden-set scenario `false-completion-claim`.
