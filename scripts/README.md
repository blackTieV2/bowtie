# scripts/

Helper scripts for repository maintenance (validation, hashing, extraction). Keep dependency-light (constitution §29). No secrets in scripts.

Phase 5: build the native app first, then run `./scripts/check_phase5_native.ps1` from the repository root. It runs the twelve-step and Phase 3 regression scripts, read-only comparison checks, and Phase 5 GUI workflows with save/reopen equality. `-ToolsOnly` reuses the existing runtime walkthrough fixture. `-EvidenceRoot .runtime/validator/evidence -RuntimeRoot .runtime/validator/runtime` directs a fresh validator run to separate ignored outputs. `python scripts/check_phase5_artifacts.py [run-directory]` independently parses CSV/XLSX/report artifacts with Python's standard library; the default output is run 006.
