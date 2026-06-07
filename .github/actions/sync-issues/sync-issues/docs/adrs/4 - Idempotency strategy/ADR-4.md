---
type: adr
title: ADR-4 - Idempotency Strategy
date: 2026-04-07
status: accepted
supersedes: 
---

## Context

The sync script must be idempotent — running it multiple times should produce the same result without duplicating comments or creating spam. This ADR is related to ADR-001 (issue matching) and ADR-003 (section-to-comment mapping with tracked IDs).

## Decision

Idempotency rules:

1. **Issue matching**:
   - If `issue.id` in frontmatter → use that GH issue number directly.
   - Otherwise match by exact title.

2. **Comment matching**:
   - If `issue.sections.<section>.id` exists in frontmatter → update that comment.
   - Otherwise find by content comparison (first 100 chars).
   - If no match found → create new comment.

3. **Content comparison**:
   - Before updating a comment, compare content.
   - Skip update if content is identical.
   - This prevents unnecessary API calls and "edited" flags.

4. **Order of operations**:
   - Always fetch existing comments first.
   - Match by tracked ID > match by content > create new.

### Options

- **Always-create**: Create new issues and comments on every run. Simplest implementation but produces duplicates and spam.
- **Title/timestamp matching**: Match by title alone without tracking IDs. Works for most cases but breaks when titles change or multiple issues share similar titles.
- **Tracked-ID idempotency with content-aware fallback (chosen)**: Store comment IDs and issue numbers in frontmatter for precise matching, with content-based fuzzy matching as a fallback when IDs are absent.

### Rationale

Storing tracked IDs in frontmatter is the most reliable foundation for idempotency — it eliminates ambiguity entirely. The content-based fallback (first 100 chars) handles the bootstrap case when frontmatter doesn't yet contain comment IDs. Content comparison before update avoids unnecessary API calls and prevents "edited" badges on comments that haven't actually changed. Fetching existing comments first and matching in priority order (tracked ID → content → create new) ensures the sync is safe to run repeatedly regardless of current state.

## Consequences

### Positive

1. Safe to run multiple times without creating duplicate issues or comments.
2. Comments stay in sync with markdown content — changes are reflected, unchanged content is left alone.
3. Minimal API calls due to content comparison before update.

### Negative

1. Requires write-back of comment IDs to frontmatter (future enhancement), adding metadata management complexity.
2. Content-based fallback matching (first 100 chars) can produce false matches if sections have similar openings.
