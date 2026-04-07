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

### One File Per Type (Screaming Architecture)

Each file contains exactly one type or class. File names match the exact casing of the entity they contain:
- `IssueFile.ts` contains `IssueFile` type
- `GitHubIssue.ts` contains `GitHubIssue` type  
- `SyncIssueUseCase.ts` contains `SyncIssueUseCase` class
- `GhCliAdapter.ts` contains `GhCliAdapter` class

This maximizes discoverability and minimizes merge conflicts—changes to one type touch only its file.

### TypeScript Convention Override

The common TypeScript convention of `types.ts` (or `types/index.ts`) is intentionally overridden here. While `types.ts` groups all types in one file, screaming architecture demands one type per file. This creates more files but delivers superior discoverability and isolation.

**Convention used:** `{TypeName}.ts` (PascalCase matches the entity exactly)
- `GitHubIssue.ts` → contains `GitHubIssue`
- `SyncOptions.ts` → contains `SyncOptions`
- `NodeFileAdapter.ts` → contains `NodeFileAdapter`

This overrides the common `types.ts` pattern because screaming architecture prioritizes discoverability over file count.

### Directory Structure

```
.github/actions/sync-issues/sync-issues/
├── src/
│   └── features/gh-push/         # Feature module (screaming: "gh-push" feature)
│       ├── domain/               # Core business logic (no external dependencies)
│       │   ├── types/
│       │   │   ├── index.ts          # Re-exports all types
│       │   │   ├── IssueFile.ts      # IssueFile, IssueFrontmatter
│       │   │   ├── GitHubIssue.ts    # GitHubIssue
│       │   │   ├── GitHubComment.ts  # GitHubComment
│       │   │   ├── SyncResult.ts     # SyncResult
│       │   │   └── SyncOptions.ts    # SyncOptions
│       │   └── services/
│       │   │   ├── ParseFrontmatter.ts     # parseFrontmatter
│       │   │   ├── ExtractSections.ts      # extractSections
│       │   │   ├── ExtractIssueId.ts       # extractIssueIdFromFilename
│       │   │   ├── DeriveTitle.ts          # deriveTitleFromFolder
│       │   │   └── ParseIssueFile.ts       # parseIssueFile (composed)
│       ├── application/
│       │   └── usecases/
│       │       └── SyncIssueUseCase.ts     # SyncIssueUseCase
│       ├── adapter/
│       │   ├── index.ts                # Re-exports all ports
│       │   ├── IssueAdapterPort.ts     # IssueAdapterPort interface
│       │   └── FileAdapterPort.ts      # FileAdapterPort interface
│       └── infrastructure/
│           └── gh-cli/
│               ├── RunGh.ts                 # runGh helper
│               ├── NodeIdToNumericId.ts     # nodeIdToNumericId helper
│               ├── GhCliAdapter.ts           # GhCliAdapter
│               └── NodeFileAdapter.ts       # NodeFileAdapter
├── docs/
│   ├── plan.md
│   └── adrs/
│       └── ...
└── index.ts                  # CLI entry point (thin, orchestrates layers)
```

**Note:** Each `.ts` file contains one type or class. File names match the exact PascalCase of the entity they contain.

### Layer Dependencies

- Domain: No dependencies on other layers (pure TypeScript)
- Application: Depends only on Domain (uses types, knows nothing about infra)
- Adapter: Depends on Domain (defines interfaces using Domain types)
- Infrastructure: Implements Adapter interfaces (depends on Adapter + Domain)

### Screaming Architecture Benefits

1. **Discoverability**: File names immediately reveal contents
   - `ParseFrontmatter.ts` contains parsing logic, `GhCliAdapter.ts` is GitHub implementation
2. **Single responsibility**: Each file has one type/class
3. **Minimal merge conflicts**: Changes isolated to single-purpose files
4. **Easy refactoring**: Move a type by moving its file
5. **TDD-friendly**: Each type can be unit tested in isolation

## Consequences

- Positive: Testable - each layer can be mocked
- Positive: Screaming architecture - file names reveal contents
- Positive: One file per type maximizes discoverability and minimizes merge conflicts
- Positive: Easy to spinoff as separate project
- Need to maintain discipline about layer boundaries

## References

- Related: [[ADR-001]], [[ADR-002]], [[ADR-003]], [[ADR-004]]
