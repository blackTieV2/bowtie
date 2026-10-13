# Task Packet â€” 005-build-phase4

- **Project:** bowtie
- **Stage:** 04-build
- **Task:** Phase 4 ONLY: offline PNG/SVG/PDF diagram exports and DOCX case report.
- **Starting state:** Clean feature/phase-3-properties-risk at ac20e1d4d4992316d34ac724cb4734f81bd62e9a; sole root C:/Users/BlackTie/Documents/GitHub/bowtie/bowtie and origin https://github.com/blackTieV2/bowtie.git verified. TARGET GATE PASS before modifications; new feature/phase-4-reporting branch.
- **Tool profile:** build
- **Scope (approved):** Human approved Phase 3 and AR46â€“AR54/AR45 correction. Export the current diagram with saved positions, scaled PNG, vector SVG/PDF and original DOCX layout for case/hazard/diagram summary, hierarchy, risk/matrix context, effectiveness, actions and linked supporting items. Derive from live snapshots only; unchanged schema/no new persistence. Deterministic ordering and metadata, test two runs. Feature branch push authorized; no main push/merge or Phase 5.
- **Stop conditions:** Unexpected dirty changes/target mismatch/secrets; schema-content gap (log AR and stop); any schema migration/persistence extension; scope beyond Phase 4. No legacy binaries/decompilers/templates/assets/branding, network app services/fonts.
- **Evidence requirement:** Rust logic tests, native GUI exports, PNG size/XML/PDF text/pages/DOCX OOXML checks, two-run hashes, rendered visual QA, committed artifacts/hashes, phase report and push output. Producer evidence remains distinct from independent QA.

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
- Missing or altered positions/content, raster-only SVG/PDF, malformed OOXML, or differing bytes for repeated same-input exports invalidate acceptance.
- Font/rendering portability is least secure: bundle a permissively licensed local font and verify artifacts mechanically and visually; qualify cross-platform raster fidelity.
- A requested report field absent from the approved schema would stop implementation; the current scoped content is present in snapshots/properties/links/matrices/actions.
- Freeze a live snapshot, render without case mutations, validate its signature before atomic output writing, and keep all work on the feature branch.

## Completion
Implementation 88c3867054ccf8dd12d258a8de691f5d1079609a; 25 Rust tests and Windows native build PASS. Four exports byte-identical across two fresh frontend runs; format parsers and all final rendered pages PASS. Artifacts/hashes committed in this packet; initial feature-branch push succeeded. Stop for human Phase 4 review, with independent QA/manual limits in the report.
