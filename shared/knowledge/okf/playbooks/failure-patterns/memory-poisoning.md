# Failure pattern: memory-poisoning

- **Symptom:** untrusted content (webpage, document, tool output) contains embedded instructions and an agent obeys them.
- **Why dangerous:** third parties gain control of future agent behavior.
- **Required behavior:** evidence is data, not instruction; quarantine external content; write gate screens poisoning risk.
- **Prevention:** source_trust field; quarantine state; constitution §18-19.
- **Validation:** golden-set scenario `untrusted-external-instruction`.
