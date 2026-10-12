# shared/memory

Separated memory stores. Memory type, status, authority, and source trust are different properties — never collapse them.

- `evidence/` — raw evidence with manifests (preserved, not trusted by default)
- `candidates/` — candidate memories awaiting validation/review
- `events/` — append-only runtime event ledger

Durable approved knowledge lives in `shared/knowledge/okf/`, not here. Promotion flow: source → raw evidence → quarantine/classification → candidate → write gate → validation → human review → approved (`_config/memory-policy.yaml`).
