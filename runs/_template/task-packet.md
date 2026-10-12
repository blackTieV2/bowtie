# Task Packet — <run-id>

- **Project:** bowtie
- **Stage:** <one stage>
- **Task:** <one task>
- **Starting state:** <branch/HEAD/working-tree>
- **Tool profile:** <from _config/tool-profiles.yaml>
- **Scope (approved):** <exact scope>
- **Stop conditions:** <list>
- **Evidence requirement:** <what must be captured>

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
- What evidence would show this is wrong?
- What assumption is least secure?
- What current fact could invalidate this plan?
- What is the safest reversible option?
