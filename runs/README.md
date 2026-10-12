# runs/

Runtime execution state, one directory per controlled run: `runs/<run-id>/`.

Each run contains: `run-state.yaml`, `context-manifest.yaml`, `task-packet.md`, `retrieval-feedback.yaml`, `memory-promotion-review.md` (copy from `_template/`).

A normal execution packet contains: one project, one stage, one task, one starting state, one permitted tool profile, one scope, one set of stop conditions, one evidence requirement.

**A checkpoint records state. A checkpoint does NOT grant permission.**

Run directories are transient operational state; they may be pruned after handoff captures their outcome, per human decision.
