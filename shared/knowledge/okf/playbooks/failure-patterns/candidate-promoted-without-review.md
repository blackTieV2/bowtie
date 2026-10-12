# Failure pattern: candidate-promoted-without-review

- **Symptom:** a repeated pattern or agent inference silently becomes canonical policy.
- **Why dangerous:** inference hardens into unreviewed authority; errors become permanent.
- **Required behavior:** DETECT → PROPOSE → REVIEW → PROMOTE; candidates always start `status/authority: candidate, canonical: false`.
- **Prevention:** memory-policy promotion table; human review for decisions/procedures/governance.
- **Validation:** audit trail in `shared/knowledge/okf/log.md` for every promotion.
