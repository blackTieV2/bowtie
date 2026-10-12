# 01-intake — Stage Contract

**Purpose:** define goal, scope, constraints, stakeholders, success criteria, and unresolved requirements for the bowtie rebuild.

## Project-specific input
- The complete behavioral spec (extracted from the legacy manuals, text-only) lives in `references/spec/`.
- Evidence manifests with SHA-256 of the original PDFs live in `shared/memory/evidence/manifests/`.

## Must produce (in `output/`)
- `intake-brief.md` — goal, scope (Standard-edition bow-tie editor first), constraints (clean-room, offline, multiplatform), success criteria (the "12 steps" walkthrough reproducible in the new app), unresolved questions for the human.

## Rules
- Read-only against `references/`; write only to `output/`.
- Do not design architecture here — that is 03-design.
- Flag every spec ambiguity explicitly; do not resolve by invention.
