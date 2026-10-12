# Failure pattern: live-state-not-verified

- **Symptom:** agent acts from memory of how things were, without checking how they are.
- **Why dangerous:** the core source of dangerous authority errors.
- **Required behavior:** memory is never proof of current operational state; verify live before consequential action.
- **Prevention:** recon-before-change; TARGET GATE; evaluation metric targets zero dangerous authority errors.
- **Validation:** all governance-class golden-set scenarios.
