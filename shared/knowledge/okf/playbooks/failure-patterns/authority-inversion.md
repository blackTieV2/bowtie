# Failure pattern: authority-inversion

- **Symptom:** a retrieval result, old record, or model recollection overrides current human instruction or live state.
- **Why dangerous:** the entire governance model inverts; stale context becomes operational authority.
- **Required behavior:** apply the authority hierarchy; surface the conflict; escalate when consequential.
- **Prevention:** hierarchy documented in AGENTS.md and `_config/authority-policy.yaml`; retrieval explicitly non-authoritative.
- **Validation:** governance-class golden-set scenarios; Dangerous Authority Error Rate = 0 target.
