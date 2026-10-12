# Codex CLI adapter

Integration notes only — no unique project truth here. Canonical rules: `AGENTS.md`.

## Boot sequence
1. `codex` in the repo root.
2. First instruction: "Read AGENTS.md, PROJECT_STATUS.md, CONTEXT.md. Report current stage, holds, and next safe action. Do not modify anything yet."
3. Codex reads `AGENTS.md` natively; it will inherit the constitution from there.

## Notes
- Keep approvals on: run with default sandbox/approval settings so destructive commands require confirmation.
- Use one session per stage; start each session with a fresh read of PROJECT_STATUS.md (do not rely on session memory).
- Phase gates live in `stages/04-build/CONTEXT.md` and the run packet — Codex must stop at each phase boundary.
