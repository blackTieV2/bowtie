# Failure pattern: wrong-repository-root

- **Symptom:** commands run in a similarly named but different repository.
- **Why dangerous:** changes pushed to the wrong project; confidential work misdirected.
- **Required behavior:** verify `git rev-parse --show-toplevel` and remote URL against PROJECT_STATUS before mutation.
- **Prevention:** TARGET GATE includes remote identity check.
- **Validation:** golden-set scenario `wrong-repository-root`.
