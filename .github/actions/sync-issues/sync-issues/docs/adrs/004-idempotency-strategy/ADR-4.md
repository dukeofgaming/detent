---
type: adr
date: 2026-04-07
status: accepted
---
# ADR-004: Idempotency Strategy

## Context

The sync script must be idempotent - running it multiple times should produce the same result without duplicating comments or creating spam.

## Decision

Idempotency rules:

1. **Issue matching**: 
   - If `issue.id` in frontmatter → use that GH issue number directly
   - Otherwise match by exact title

2. **Comment matching**:
   - If `issue.sections.<section>.id` exists in frontmatter → update that comment
   - Otherwise find by content comparison (first 100 chars)
   - If no match found → create new comment

3. **Content comparison**:
   - Before updating a comment, compare content
   - Skip update if content is identical
   - This prevents unnecessary API calls and "edited" flags

4. **Order of operations**:
   - Always fetch existing comments first
   - Match by tracked ID > match by content > create new

## Consequences

### Positive
- Safe to run multiple times
- Comments stay in sync with markdown

### Negative
- Need to write back comment IDs to frontmatter (future enhancement)

## References

- Related: [[ADR-001]], [[ADR-003]]