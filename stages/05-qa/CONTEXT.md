# 05-qa — Stage Contract

**Purpose:** validate independently. Producer ≠ final validator.

## Rules
- Validate in a fresh context containing: requirements, accepted design, changed artifacts, acceptance criteria, test evidence.
- Do not silently become an implementation phase; defects are reported, not fixed, unless the human explicitly assigns a fix scope.
- Execute the acceptance criteria from 03-design against the built artifact; record pass/fail with evidence in `output/`.
- Distrust confidence: an artifact is not correct because a strong model produced it or several agents repeated the claim.

## Exit criteria
Validation report with evidence; defects list; explicit verdict per acceptance criterion.
