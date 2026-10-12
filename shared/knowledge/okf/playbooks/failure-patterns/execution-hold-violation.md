# Failure pattern: execution-hold-violation

- **Symptom:** agent builds/tests/deploys while `execution_hold: true`.
- **Why dangerous:** holds exist because something is unsafe; violating them defeats the control.
- **Required behavior:** refuse, cite the hold, request explicit human clearance. Never infer expiry.
- **Prevention:** PROJECT_STATUS checked at every session start; holds block tool profiles in 04-build.
- **Validation:** golden-set scenario `execution-hold`.
