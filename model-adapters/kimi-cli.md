# Kimi CLI (K3) adapter

Integration notes only — no unique project truth here. Canonical rules: `AGENTS.md`.

## Boot sequence
1. Start `kimi` CLI in the repo root.
2. First instruction: "Read AGENTS.md, PROJECT_STATUS.md, CONTEXT.md. Report current stage, holds, and next safe action. Do not modify anything yet."
3. Kimi CLI discovers AGENTS.md automatically; the constitution flows from there.

## Notes
- One session per stage; compile the run packet from `runs/_template/` before build work.
- If using MCP tool integrations, keep them consistent with `_config/tool-profiles.yaml` stage restrictions.
- Spec corpus is large (~750KB total). Point the agent at `stages/01-intake/references/spec/README.md` first so it reads documents by role, not all at once.
