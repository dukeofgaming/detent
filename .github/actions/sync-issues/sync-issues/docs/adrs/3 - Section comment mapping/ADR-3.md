---
type: adr
title: ADR-3 - Section-to-Comment Mapping
date: 2026-04-07
status: accepted
supersedes: 
---

## Context

The canonical template in `docs/issues/AGENTS.md` defines these sections: **Description** (the main problem/goal description) which maps to the GitHub issue body, **Tasks** (task list with checkboxes) which maps to a GitHub comment, and **Journal** (progress notes) which maps to a GitHub comment. Additional sections may exist (Notes, Experiments Log, etc.) and should also map to comments. This ADR is related to ADR-004 for idempotency.

## Decision

Mapping strategy:

| Section | GitHub Target | Notes |
|---------|---------------|-------|
| Description | Issue body | First section before Tasks |
| Tasks | Comment | Must map to tracked comment ID |
| Journal | Comment | Must map to tracked comment ID |
| Any other `##` | Comment | Created as new comments |

Comment IDs are tracked in frontmatter:

```yaml
issue:
    sections:
        description: {comment_id}
        tasks: {comment_id}
        journal: {comment_id}
```

### Options

- **Single-body mapping**: Put all sections into the GitHub issue body. Simple but makes the issue body bloated and loses the ability to update sections independently.
- **Fixed two-comment structure**: Map Description to the body, then concatenate all other sections into a single comment. Reduces comment count but prevents independent updates per section.
- **Per-section comment mapping with tracked IDs (chosen)**: Each `##` section maps to its own GitHub comment, with comment IDs stored in frontmatter for idempotent updates.

### Rationale

Mapping each section to its own comment keeps the GitHub issue body clean (Description only) while preserving the structured nature of the markdown source. Tracking comment IDs in frontmatter enables precise, idempotent updates — the sync script can update only the comments whose content has changed, without needing to search by content. This also means sections like Tasks and Journal can evolve independently across sync runs.

## Consequences

### Positive

1. Clear separation of concerns — each section has a well-defined target in GitHub.
2. Journal and Tasks are preserved as independent, updateable comments.
3. Extensible to arbitrary additional sections (any `##` heading becomes a comment).

### Negative

1. Comment IDs must be tracked in frontmatter for idempotency, adding metadata management overhead.
2. More GitHub comments per issue compared to a single-body approach.
3. Requires bidirectional write-back to update frontmatter with new comment IDs after creation.
