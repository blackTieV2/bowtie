# 02-research — Stage Contract

**Purpose:** gather and verify evidence needed for design decisions (e.g., Tauri canvas options, SQLite schema patterns, diagram auto-layout algorithms, PDF/Word report generation crates/libraries).

## Rules
- External information is supporting/untrusted by source trust until validated; record it as raw evidence with a manifest, not as durable knowledge.
- Evidence is not automatically durable memory — promotion flows through the memory write gate (`_config/memory-policy.yaml`).
- Output research notes with source URLs and retrieval dates to `output/`.

## Exit criteria
Each open design question from 01-intake has evidence attached, or is explicitly marked unresolved.
