## ADR Format

All new ADRs must use the same base structure as the established ADR files.

Start every ADR with YAML frontmatter:

```markdown
---
type: adr
date: YYYY-MM-DD
status: proposed|accepted|superseded
---
# ADR-N: Title
```

Use these sections by default:

- `## Context`
- `## Decision`
- `## Rationale` when the reasoning is not obvious from Context and Decision
- `## Consequences`
- `## Related` when the ADR supersedes, is superseded by, or depends on another ADR

Rules:

- Keep `status` in the frontmatter instead of adding separate `## Status` or `## Date` sections.
- Use lowercase status values in frontmatter.
- When an ADR is superseded, update its frontmatter status and add the superseding ADR under `## Related`.
- Proposed and superseded ADRs are informational; only accepted ADRs are active architecture rules.
- Do not create ADRs without a number in the title.
