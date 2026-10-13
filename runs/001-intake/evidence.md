# Intake evidence — 001-intake

Status: producer structural review passed; semantic requirements/source integrity remain unvalidated and awaiting human review.

## Starting target and authorization

Root: `C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie`. Branch `main`, HEAD `f4295ebb87868fc5517eecf090e945ad7a72ca2e`. Starting `git status --short` was empty. Origin fetch/push: `https://github.com/blackTieV2/bowtie.git`. The human explicitly assigned intake and output/run-record writes. No branch, commit, merge, or remote state changed.

Required reads were performed in order: AGENTS, PROJECT_STATUS, root CONTEXT, intake CONTEXT, spec README. Stage/holds/next safe action and TARGET GATE: PASS were reported before modifications. The malformed build flag/stale profile was surfaced; no implementation authority was inferred. No execution hold was active or cleared.

## Actions and changed artifacts

1. Scoped file listing, relevant governance/configuration/template reads, and role-based corpus review, indexed authoritative references first. Exact paths, hashes, review depth and limits appear in context-manifest.yaml. No line-by-line certification of deferred modules is claimed.
2. Compared actual and LF-normalized extract hashes with SHA256SUMS. Nine normalized matches; R6 and indexed training slides remain mismatches. Source PDFs were unavailable; their manifest hashes were not independently verified. Quick Start 3.6 is unlisted. Compared the two R8.2 copies: one blank-line and one release-notes menu-spacing difference.
3. Refined the existing intake brief; added the feature inventory and A01–A30 question list. Requirements are paraphrased candidates, not approved memory. Copied all five files from runs/_template into the previously absent runs/001-intake and filled them; added this evidence record.

Changed files: the existing intake brief, two new intake outputs, and six run records (the five copied templates plus evidence.md). No spec, governance, shared memory, template, or application file changed. Output hashes exclude run-state.yaml itself to avoid a self-referential hash.

## Commands and results

- Recon: `git rev-parse --show-toplevel`, `git status --short`, `git branch --show-current`, `git rev-parse HEAD`, `git remote -v` — expected target/clean start/main/HEAD/origin.
- Reads: PowerShell `Get-Content -LiteralPath`, scoped `rg --files` and `rg -n`, Python role/section/diff/hash inspection. No browsing, binaries, installs, builds, or application execution.
- Initial oversized manual outputs were truncated; relevant section ranges/role scans were read subsequently. A Unicode diff-print failed under the console encoding and was rerun with UTF-8. Neither caused writes. A combined evidence/finalization shell call was rejected by tool policy before execution; it was replaced with a scoped patch and separate finalization call.
- Integrity: Python actual-byte and CRLF-to-LF SHA-256 checks, plus `git ls-files --eol` — all eleven raw-byte hashes differ; nine normalized hashes match and two remain mismatches. No checksum record rewritten.
- Structural assertions — PASS: 44 F rows, 51 C rows, 29 S rows, 40 R rows; crosswalk target IDs and source links resolve; A01–A30 sequential; twelve acceptance steps; filled packet placeholders; YAML parses; 23 corpus role records; recorded input hashes unchanged.
- Scope: `git diff --name-only` — only the existing intake brief tracked modification. `git ls-files --others --exclude-standard` — only requested intake outputs/run directory. No unexpected dirty work encountered.
- `git diff --check` — exit 0, no whitespace errors. `git diff --stat` / `git status --short` — expected scoped artifacts. New outputs also checked for replacement characters and structure.

Application tests: not run (documentation-only intake). Independent semantic validation: not performed. Producer checks do not establish approved requirements, exhaustive feature parity, or 05-qa completion. Artifacts are parsed/structurally checked, not tested/validated/accepted/approved/deployed.

## Remaining risks and next safe action

Thirty human questions remain. Priority risks: source integrity/provenance, version precedence, edition contradictions, incomplete report/check catalogs, proprietary import versus clean-room constraints, offline/multiplatform outputs, and undefined acceptance tolerances. The manuals/matrix disclaim completeness, extraction omits images, and some cross-references fail. Enumerated edition-row coverage is not proof of exhaustive behavior.

Stop here. The owner reviews the drafts and disposes of A01–A30, source questions first. Source verification, governance correction, research, design, build, or memory promotion belongs in a subsequent explicit bounded assignment. No approval for an execution exception is requested.
