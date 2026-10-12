# shared/tooling

Notes on tooling used around this repository (extraction scripts, local helpers). Tooling notes are operational aids, not project truth.

Current:
- PDF spec extraction performed with `pdftotext -layout` (2026-10-12); outputs in `stages/01-intake/references/spec/`, manifests in `shared/memory/evidence/manifests/`.

Any future retrieval indexing (SQLite FTS prototype, etc.) must follow `_config/retrieval-policy.yaml` — derived, disposable, rebuildable, non-authoritative.
