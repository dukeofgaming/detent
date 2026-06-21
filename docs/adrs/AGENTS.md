## ADR Format

All new ADRs must use the same base structure as the established ADR files.

```markdown
---
type: adr
title: ADR-N - {title}
date: YYYY-MM-DD
status: draft|proposed|accepted
supersedes: 
---

## Context
<!-- Describe the problem or decision context here. -->

## Decision
<!-- Describe the decision made here. -->

### Options
<!-- List the considered options here. -->

### Rationale
<!-- Describe the rationale for the decision here. -->

## Consequences
<!-- Describe the consequences of the decision here. -->

### Positive

1. ...

### Negative

1. ...
```

## Rules

- Keep `status` in the frontmatter instead of adding separate `## Status` or `## Date` sections.
- Use lowercase status values in frontmatter.
- When an ADR is superseded, update its frontmatter status and add the superseding ADR under `## Related`.
- Proposed and superseded ADRs are informational; only accepted ADRs are active architecture rules.
- Do not create ADRs without a number in the title.
- ADR titles should be short and conversational (e.g. "Screaming Architecture", "Clean Architecture").

## File Structure

Each ADR lives in its own directory under `docs/adrs/`:

```
docs/adrs/<N> - <short-name>/
  ADR-N.md
```

- The directory name starts with the number and a short (one-line) slug matching the title.
- The file inside is always `ADR-N.md` matching the ADR number.
- This ensures each ADR has a stable address for wikilinks.
- Cross-reference other ADRs by number in `## Related` sections.
- ADRs are numbered sequentially starting at 1 with no gaps.

## Index

| ADR | Title | Status |
|-----|-------|--------|
| **ADR-1** | Bidirectional Compiler | accepted |
| **ADR-2** | Rust Stack | accepted |
| **ADR-3** | Handcrafted BPMN Types | accepted |
| **ADR-4** | folder.rs Modules | accepted |
| **ADR-5** | Clean Architecture | proposed |
| **ADR-6** | Slice Test Harness | accepted |
| **ADR-7** | Screaming Architecture | accepted |
| **ADR-8** | BDD Test Layout | accepted |
