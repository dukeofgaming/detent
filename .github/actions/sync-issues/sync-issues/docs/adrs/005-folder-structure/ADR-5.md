# ADR-005: Folder Structure (Vertical Slice + Clean Architecture)

## Status

Accepted

## Context

The sync-issues script is becoming complex and needs proper organization to:
1. Support TDD approach with testable layers
2. Enable future spinoff as standalone project
3. Follow Clean Architecture principles
4. Use screaming architecture for discoverability

## Decision

### Directory Structure

```
.github/actions/sync-issues/sync-issues/
├── domain/                    # Core business logic (no external dependencies)
│   ├── types/                 # Issue, Journal, Section, etc.
│   │   └── issue.ts          # IssueFile, GitHubIssue types
│   └── services/              # Pure functions
│       └── parser.ts         # parseIssueFile, extractSections
├── application/              # Use cases & orchestration
│   └── usecases/
│       └── sync-issue.ts     # SyncIssueUseCase
├── adapter/                   # Interfaces (ports)
│   ├── github/
│   │   └── issue-adapter.ts  # IssueAdapter interface
│   └── filesystem/
│       └── file-adapter.ts   # FileAdapter interface
├── infrastructure/            # External dependencies (concrete implementations)
│   └── gh-cli/
│       └── gh-adapter.ts     # Concrete gh CLI implementation
├── docs/
│   ├── plan.md
│   └── adrs/
│       ├── ADR-001/
│       └── ...
└── index.mjs                  # Entry point & CLI
```

### Layer Dependencies

- Domain: No dependencies on other layers
- Application: Depends only on Domain
- Adapter: Depends on Domain (interfaces)
- Infrastructure: Implements Adapter interfaces

## Consequences

- Positive: Testable - each layer can be mocked
- Positive: Screaming architecture - file names reveal contents
- Positive: Easy to spinoff as separate project
- Need to maintain discipline about layer boundaries

## References

- Related: [[ADR-001]], [[ADR-002]], [[ADR-003]], [[ADR-004]]
