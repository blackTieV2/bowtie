Phase 5 validator erratum — `/root/phase5_validator`, 2026-10-13

I independently reran `Get-FileHash -Algorithm SHA256 -LiteralPath app/src/CaseExchange.tsx`. The correct SHA-256 is:

`6f09f5b7ebdff13be16a9e74909cc7cc0c5df75a7476092d2398f17083cb68f7`

The original report contains an extra `b` in that hash transcription. This corrects the recorded hash only; the validation verdict remains unchanged. No files were modified.
