# 00-triage — Stage Contract

**Purpose:** classify the incoming task before any work happens. No implementation in this stage.

## Must determine
- task (one sentence)
- risk class (routine / consequential / destructive)
- target (repo paths / systems touched)
- mode (read-only / modifying)
- required recon
- next stage

## Rules
- Verify current `PROJECT_STATUS.md` stage and holds first.
- If an execution hold covers the requested work, report it; do not proceed.
- Output a triage note to `stages/00-triage/output/` and stop.

## Exit criteria
Triage note written; next stage identified; holds confirmed or escalated.
