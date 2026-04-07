---
type: adr
date: 2026-04-07
status: accepted
---
# ADR-002: Title Resolution Order

## Context

Issue titles can come from multiple sources:
1. Frontmatter `title` field
2. Filename (e.g., `#3.md` → "3")
3. Parent folder name (e.g., `3-convert-bpmn-to-mdx/#3.md` → "Convert BPMN to MDX")

The canonical template in `docs/issues/AGENTS.md` specifies title in frontmatter, but we need sensible fallbacks.

## Decision

Title resolution order:

1. **Primary**: `issue.title` from frontmatter
2. **Secondary**: Extract from first H1 heading (`# Title`) in the markdown content
3. **Tertiary**: Parse from folder name:
   - `3-feature-name/` → "Feature: feature-name" (capitalize, replace hyphens)
   - `bugfix/1-foo/` → "Fix: foo"
4. **Final**: Use filename without extension (least preferred)

## Consequences

### Positive
- Works with minimal frontmatter
- Parent folder provides semantic context

### Negative
- Need to handle various folder naming conventions

## References

- Template: [[docs/issues/AGENTS.md]]
- Related: [[ADR-001]]