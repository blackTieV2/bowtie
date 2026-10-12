# 03-design — Stage Contract

**Purpose:** produce architecture, implementation plan, interfaces, risks, rollback, and acceptance criteria.

## Must produce (in `output/`)
- `architecture.md` — Tauri/Rust + React/Konva + SQLite structure (or a justified alternative), module boundaries, data model schema.
- `implementation-plan.md` — phase-gated plan matching REBUILD-MASTER-PROMPT.md phases 1–6.
- `acceptance-criteria.md` — derived from the "12 steps" walkthrough; one verifiable criterion per feature.
- Risk register + rollback approach.

## Rules
- **Design does not authorize build.** `build_approved` flips only by explicit human instruction recorded in PROJECT_STATUS.md.
- Consequential design judgments → decision-record candidates (`shared/templates/memory/decision-record.template.md`).
- Every designed feature must trace to a spec section in `references/spec/`.
