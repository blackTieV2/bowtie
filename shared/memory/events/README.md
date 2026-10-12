# Runtime Event Ledger

Append-only events: run-created, target-gate-pass, evidence-captured, candidate-created, decision-proposed, human-approved, build-started, file-modified, test-failed, repair-started, validator-pass, memory-promoted, handoff-created.

Schema: `shared/templates/memory/memory-event.template.json`. `runs/<run-id>/run-state.yaml` is a projection of event history — a checkpoint records state but does not grant permission.
