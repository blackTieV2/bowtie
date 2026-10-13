# Task Packet — 004-build-phase3

- **Project:** bowtie
- **Stage:** 04-build
- **Task:** Phase 3 ONLY: per-kind typed properties, configurable risk matrices/assessments, barrier effectiveness and action register.
- **Starting state:** Sole root C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie; clean feature/phase-2-diagram-canvas at 1fc5d75a91d25ac031cc47e644b1c9b4766461d7; origin https://github.com/blackTieV2/bowtie.git. TARGET GATE: PASS. Created feature/phase-3-properties-risk from that exact tip.
- **Tool profile:** build
- **Scope (approved):** Human accepted Phase 2 and AR37–AR44. Phase 3 authorizes typed per-kind properties, maximum eight user-defined risk matrices, inherent/residual assessments for TOP EVENTS/CONSEQUENCES (AR45 human correction), effectiveness annotations and a sortable/filterable editable action register over tree entities. Include soft-hide, confirmed global clearing, custom axes/XY-YX, ranked categories/cell editing. Preserve parameterized SQL, unchanged schema/triggers, offline runtime and manual ALARP rationale. Current instruction supersedes profile's older Phase 1 next action.
- **Stop conditions:** Unexpected dirty work, wrong target/ownership, secrets, migration proposal requiring review, any work beyond Phase 3. Human explicitly requires any migration proposal to be logged as an AR and stopped for review. No legacy binaries/decompilers/names/assets, merge/push or Phase 4.
- **Evidence requirement:** Target and authority, spec-property mapping, Rust matrix/assessment/register tests, native GUI authoring and save/reopen equality, build/results/hashes/screenshots, phase report. Do not label unavailable validation as passed.

## Optimization criteria

```yaml
priorities:
  correctness: critical
  preservation_of_existing_work: critical
  reversibility: high
  maintainability: high
  security: critical
  speed: secondary
  token_cost: secondary
```

## Dissent check (for high-risk work)
- Evidence against the risk target request: SPEC-013 §7.12 and both approved schema triggers permit top events/consequences, not threats; an in-memory probe confirms rejection.
- Least secure assumption: that "threats" was intended to mean "top events". Do not silently substitute targets or bypass triggers.
- Gate reached: meeting the literal threat requirement needs a trigger/schema-version compatibility change, which the manual does not require and the current instruction gates for review.
- Safest reversible option: preserve all application code/schema; submit AR45 with a scope correction option and a concrete migration alternative. Stop implementation pending human resolution.

## Outcome

Stopped at migration-review gate before application changes. Phase 3 is NOT complete. See stages/04-build/output/phase-3-report.md and risk-target-review.md in this run. No Phase 3 build or GUI acceptance has been claimed.

## Human resolution and continuation

Human resolved AR45 by correction: assessments target TOP EVENTS and CONSEQUENCES. No migration is authorized or needed. Resume Phase 3 on the existing clean branch at 7e8e2d1; renewed TARGET GATE PASS. Additional §7.12 scope: soft-hide matrices by clearing displayed name while retaining assessments, confirmed Case-level Clear all risk assessments, XY/YX/custom axis labels, vertical severity/horizontal likelihood, ordered low-to-high categories and rank-based sorting, category-selection/cell-click editing. Stop before Phase 4. This continuation supersedes the historical stop/outcome above.

## Final outcome

Phase 3 implemented with unchanged SQL and AR45 resolved-by-correction. Final Windows build passed (53.05s), 22 Rust tests passed, native twelve-step regression plus Phase 3 GUI authoring passed, and tree/links/matrices/assessments/lookups/actions/custom definitions survived Save/Open equality. AR46–AR54 await boundary review; screenshots and hashes accompany the report. Stop before Phase 4. Native chooser/platform/independent QA and large-case checks remain pending.

Implementation commit: 834071c. Documentation/evidence are a separate commit. No push or merge performed.
