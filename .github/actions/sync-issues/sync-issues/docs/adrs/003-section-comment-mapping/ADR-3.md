# ADR-003: Section-to-Comment Mapping

## Status

Accepted

## Context

The canonical template in `docs/issues/AGENTS.md` defines these sections:
- **Description**: The main problem/goal description → GitHub issue body
- **Tasks**: Task list with checkboxes → GitHub comment
- **Journal**: Progress notes → GitHub comment

Additional sections may exist (Notes, Experiments Log, etc.) and should also map to comments.

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

## Consequences

- Positive: Clear separation of concerns
- Positive: Journal/Tasks preserved as comments
- Need to track comment IDs in frontmatter for idempotency

## References

- Template: [[docs/issues/AGENTS.md]]
- Related: [[ADR-004]]
