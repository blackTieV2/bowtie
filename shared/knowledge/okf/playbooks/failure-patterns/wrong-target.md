# Failure pattern: wrong-target

- **Symptom:** modification applied to the wrong file/module/service/host.
- **Why dangerous:** correct-looking change in the wrong place is worse than no change.
- **Required behavior:** verify target identity (path, host, service, version) against the task packet before acting.
- **Prevention:** bounded task packets with explicit target; infrastructure gate checks (host, address, config path, rollback).
- **Validation:** manual gate checklist in 04-build.
