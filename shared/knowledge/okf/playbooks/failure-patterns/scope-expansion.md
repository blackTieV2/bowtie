# Failure pattern: scope-expansion

- **Symptom:** agent does adjacent "helpful" work beyond the approved scope (e.g., starts Phase 2 during a Phase 1 packet).
- **Why dangerous:** unreviewed changes bypass phase gates and human checkpoints.
- **Required behavior:** stop at scope boundary; propose the expansion to the human instead.
- **Prevention:** bounded packets (one stage, one task, one scope); phase gates in 04-build contract.
- **Validation:** golden-set scenario `scope-expansion`.
