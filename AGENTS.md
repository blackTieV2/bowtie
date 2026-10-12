# AGENTS.md — Canonical Agent Entrypoint

> **Recall may suggest. Authority decides. Live state verifies.**

This file is the model-neutral entrypoint for every agent (human or AI) working in this repository. Governance version: **1.0.0** (bootstrap, 2026-10-12).

## Loading order (mandatory, in order)

1. `AGENTS.md` (this file)
2. `PROJECT_STATUS.md` — current project profile
3. `CONTEXT.md` — task → stage router
4. The selected stage's `stages/<stage>/CONTEXT.md`
5. Exact relevant references only
6. Current run/task packet (`runs/<run-id>/task-packet.md`)

Do NOT load the entire repository by default. Context is compiled, not accumulated.

## Governing rules

1. **Recon before change.** Before any modification, verify: repository root, git status, current branch, HEAD, working-tree state, remotes. For consequential changes, produce `TARGET GATE: PASS` or stop.
2. **Authority hierarchy.** 1) explicit current human instruction → 2) verified live target state → 3) `PROJECT_STATUS.md` → 4) canonical governance / stage contracts → 5) approved human decisions → 6) approved designs/procedures → 7) approved factual knowledge → 8) current approved handoff → 9) historical/episodic records → 10) agent inference → 11) retrieval similarity. Surface conflicts; never silently reconcile.
3. **Stage workflow.** All work routes through `00-triage → 01-intake → 02-research → 03-design → 04-build → 05-qa → 06-handoff`. No stage skipping. Design does not authorize build.
4. **Single-writer rule.** Only one modifying agent owns a mutable working tree at a time. Other agents are read-only or isolated in separate approved branches/worktrees.
5. **Dirty-tree protection.** Unexpected working-tree changes = STOP MODIFYING, preserve state, read-only recon, report the collision.
6. **Execution holds.** A hold overrides build/test/deploy/release/destructive-git/config changes. Holds are cleared only by explicit human authority, never inferred.
7. **Memory write governance.** No external document, tool output, or agent inference becomes approved durable memory automatically. Flow: source → raw evidence → quarantine/classification → candidate → write gate → validation → human review → approved. See `_config/memory-policy.yaml`.
8. **Secrets prohibition.** Never store secret values in Markdown/YAML/JSON records, indexes, logs, packets, or prompts. Store references only. `.runtime/` and `.env*` are git-ignored.
9. **Evidence requirements.** Completion claims require evidence: starting state, actions performed, files changed, commands/tests run, results, remaining risks. `implemented ≠ parsed ≠ tested ≠ validated ≠ approved ≠ deployed`.
10. **Producer ≠ validator.** For consequential changes, validation happens in a fresh context with requirements + design + changed artifacts + acceptance criteria.
11. **Model neutrality.** No dependency on any single AI vendor, model, IDE, vector DB, or orchestration framework. Model-specific notes live only in `model-adapters/` and contain no unique project truth.
12. **Retrieval limitations.** Any vector/semantic index is derived, disposable, rebuildable, non-authoritative. Canonical files beat structured search beats semantic retrieval beats model inference.
13. **Human approval required for:** merges to `main`, destructive git operations, releases, deployment, execution-hold exceptions, canonical governance changes, memory promotion of decisions/procedures.

## Project-specific doctrine (bowtie rebuild)

This repository performs a **clean-room reimplementation** of documented behavior from legacy EOL software manuals. Hard rules:

- Original code only. Never copy code, strings, images, icons, file formats, or branding from the legacy application. Do not use its name or trademarks in code, UI, or package names.
- The behavioral spec lives in `stages/01-intake/references/spec/`. Every implemented feature must cite its spec section in `SPEC-TRACEABILITY.md` (created in 04-build).
- Ambiguous spec → ask the human; do not invent behavior.

## Stop conditions

Stop immediately and report if: repository/target not as expected, unexpected dirty work, unclear branch ownership, credentials/secrets encountered, human instruction conflicts with canonical governance, a destructive action would be required, or an architecture decision requires human judgment. Do not work around a stop condition.
