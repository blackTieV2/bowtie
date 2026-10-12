# Bootstrap Report — bowtie

- **Project:** bowtie (clean-room rebuild of an EOL bow-tie risk-analysis desktop application)
- **Repository:** https://github.com/blackTieV2/bowtie
- **Mode confirmed:** FRESH REPOSITORY — PASS (recon found only `.gitignore` + placeholder `README.md` on `main`)
- **Starting branch:** `main`
- **Starting HEAD:** `261a1ceb75fba8e931a525a657c46b87771bc9fd`
- **Bootstrap branch:** `feature/project-governance-bootstrap`
- **Governance version:** 1.0.0 (human-supplied constitution, applied 2026-10-12)
- **Operator:** Kimi (GUI agent) via GitHub MCP, under explicit human instruction

## Created

- Root canon: `AGENTS.md`, `CONTEXT.md`, `PROJECT_STATUS.md`, `README.md`, `workspace.manifest.yaml`
- Policies: `_config/{authority,memory,retrieval,tool-profiles}-policy.yaml`
- Stage contracts: `stages/00-triage` … `stages/06-handoff` (7 × `CONTEXT.md`)
- OKF knowledge structure: `shared/knowledge/okf/` (index, log, domains, systems, datasets, decisions, episodes, relationships, playbooks, references)
- Memory stores: `shared/memory/{evidence,candidates,events}` with READMEs
- Templates: `shared/templates/memory/` (memory record, decision record, evidence manifest, event, retrieval feedback)
- Runs: `runs/README.md` + `runs/_template/` (run-state, context-manifest, task-packet, retrieval-feedback, promotion-review)
- Evaluation: `evaluations/governed-memory/` (README, golden-set with 11 scenarios, scoring; Dangerous Authority Error Rate target = 0)
- Failure-pattern library: 13 patterns under `shared/knowledge/okf/playbooks/failure-patterns/`
- Misc: `scripts/`, `model-adapters/`, `skills/` READMEs; `setup/README.md`
- Project payload: `stages/01-intake/output/intake-brief.md`, `stages/01-intake/references/spec/README.md`, `stages/04-build/references/REBUILD-MASTER-PROMPT.md`, 12 evidence manifests (SPEC-001..012) with SHA-256 of the owner's source PDFs

## Updated

- `.gitignore` — replaced placeholder C++ template with exclusions matching the target stack (Node/Rust) plus governance runtime exclusions (`.runtime/`, `.env*`, caches, vector indexes). The C++ entries can be restored from git history if ever needed.

## Skipped (with reason)

- Bulk spec text (~700 KB extracted from 12 owner-supplied PDFs) NOT pushed to the remote: the manuals carry a restrictive training-use-only license. Delivered out-of-band to the owner as a local package; owner drops the `.md` files into `stages/01-intake/references/spec/` after cloning. Integrity anchored by SHA-256 manifests.
- No application code, no vector infrastructure, no SaaS memory — per constitution non-goals §54.

## Validation

- structural: all canonical files, 7 stages, OKF, memory stores, templates, run templates, evaluation scaffold, failure patterns present — PASS (verified via commit history on the bootstrap branch)
- parse/schema: YAML/JSON templates follow the constitution schemas — PASS by construction (not machine-validated; noted as limitation)
- secrets: none introduced; secret-like content excluded by policy and .gitignore — PASS
- model neutrality: no vendor/model/tool dependencies in canonical files; adapters isolated to `model-adapters/` — PASS
- Git cleanliness: only intended files on the bootstrap branch; `main` untouched — PASS

## Holds and approvals

```yaml
execution_hold: true
build_approved: false
qa_approved: false
deployment_approved: false
release_approved: false
```

Repository governance installation does not authorize application implementation.

## Unresolved variables

- TIMEZONE: unresolved (owner to set in PROJECT_STATUS.md)
- LOCAL_REPOSITORY_PATH: unresolved (set at first clone)
- Final product name: to be proposed by the build agent in Phase 1 (3 options), chosen by human

## Known risks

- Legacy manuals' license is training-use-only — keep repo private, work personal/internal.
- Legacy product line continues under its acquirer; no distribution of any clone; no legacy name/branding.
- Diagram auto-layout is the expected hardest engineering problem (assess in 02-research/03-design).

## Next safe action

1. Human reviews and merges PR `feature/project-governance-bootstrap` → `main`.
2. Owner clones locally, drops the 12 extracted spec `.md` files into `stages/01-intake/references/spec/`, commits.
3. Human starts the intake agent on `stages/01-intake/` with a run packet from `runs/_template/`.
4. When design is approved, human sets `build_approved: true`, clears `execution_hold`, and hands the build agent `stages/04-build/references/REBUILD-MASTER-PROMPT.md` (Phase 1 only).
