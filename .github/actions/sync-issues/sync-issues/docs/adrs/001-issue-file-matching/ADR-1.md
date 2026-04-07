---
type: adr
date: 2026-04-07
status: accepted
---
# ADR-001: Issue File Matching Strategy

## Context

Issue files in `docs/issues/` are named `#<id>.md` where `<id>` should correspond to a GitHub issue number. The current implementation matches issues by title, which causes:
1. New issues to be created instead of updating existing ones
2. Filename not being used to determine the target GH issue

## Decision

We will use the following matching strategy:

1. **Primary**: If `issue.id` is present in frontmatter, use that GH issue number directly
2. **Secondary**: Extract ID from filename pattern `#<number>.md`
3. **Fallback**: Match by title (current behavior, for files without ID)

This ensures that:
- `#3.md` maps to GH issue #3
- `#4.md` maps to GH issue #4 (or creates #4 if it doesn't exist)
- Files without numeric prefix fall back to title matching

## Consequences

### Positive
- Existing issues are updated correctly
- Clear mapping between files and GH issues

### Negative
- Need to ensure filenames match GH issue numbers

## References

- Template: [[docs/issues/AGENTS.md]]