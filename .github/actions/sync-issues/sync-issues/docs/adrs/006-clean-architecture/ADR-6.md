# ADR-6: Clean Architecture for sync-issues

## Status

Accepted

## Context

This ADR documents the Clean Architecture decisions and implementation details for the sync-issues GitHub Action. It consolidates and expands upon ADR-005's folder structure decisions with specific attention to layer dependencies, TypeScript configuration, and code organization.

## Decision

### Clean Architecture Layers

The sync-issues implementation follows Uncle Bob's Clean Architecture with four distinct layers:

```
.github/actions/sync-issues/sync-issues/
├── src/features/gh-push/
│   ├── domain/               # Entities only (innermost layer)
│   ├── application/          # Use cases + Ports
│   ├── adapters/            # Implementations
│   └── infrastructure/       # External dependencies
└── index.ts                  # Entry point (orchestrates layers)
```

### Layer Definitions

| Layer | Contents | Dependencies |
|-------|----------|--------------|
| **Domain** | Entities (IssueFile, GitHubIssue, etc.), pure functions (ParseFrontmatter, ExtractSections) | None (innermost) |
| **Application** | Use cases (SyncIssueUseCase), Port interfaces (IssueAdapterPort, FileAdapterPort) | Domain only |
| **Adapters** | Implementations (GhCliAdapter, NodeFileAdapter) | Application ports + Infrastructure |
| **Infrastructure** | External dependencies (RunGh, NodeIdToNumericId, node:fs) | None (outmost) |

### Dependency Direction Rule

**Core principle**: All dependencies point inward. Inner layers never know about outer layers.

```
Application ──► Domain ◄── Infrastructure
       │                        ▲
       │                        │
       └──────── Adapters──────┘
              implements
```

**Practical implications**:
- Domain has zero imports from other layers
- Application imports from Domain (types) and Application (ports)
- Adapters import from Application (ports) and Infrastructure (helpers)
- Infrastructure imports only from node:fs, node:child_process

### Port/Adapter Pattern

Following Uncle Bob's "Interface Adapters" layer:

- **Ports** (interfaces) live in the **Application layer**
- **Adapters** (implementations) live in the **Adapters layer**
- This ensures Application owns the interface contracts

```typescript
// Application layer - Port definition
// src/features/gh-push/application/ports/IssueAdapterPort.ts
export interface IssueAdapterPort {
  findIssueByNumber(number: number): Promise<GitHubIssue | null>;
  createIssue(title: string, body: string, labels: string[]): Promise<GitHubIssue>;
  // ...
}

// Adapters layer - Implementation
// src/features/gh-push/adapters/GhCliAdapter.ts
export class GhCliAdapter implements IssueAdapterPort {
  // implements all IssueAdapterPort methods
}
```

### One File Per Type (Screaming Architecture)

Each TypeScript file contains exactly one type or class. File names match the exact PascalCase of the entity:

- `IssueFile.ts` contains `IssueFile` type
- `GhCliAdapter.ts` contains `GhCliAdapter` class
- `SyncIssueUseCase.ts` contains `SyncIssueUseCase` class

**Convention override**: The common `types.ts` pattern is intentionally avoided. Screaming architecture prioritizes discoverability over file count.

### TypeScript Configuration

**Location**: tsconfig files are scoped to the vertical slice:

```
.github/actions/sync-issues/sync-issues/
├── tsconfig.base.json           # Base Deno configuration
└── src/features/gh-push/
    └── tsconfig.json           # Slice-level (extends base)
```

**tsconfig.base.json** (Deno-recommended defaults):
```json
{
  "compilerOptions": {
    "target": "ES2022",
    "lib": ["ES2022", "deno.ns"],
    "strict": true,
    "useUnknownInCatchVariables": true,
    "noImplicitOverride": true
  }
}
```

**tsconfig.json** (slice-level paths):
```json
{
  "extends": "../../tsconfig.base.json",
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "#domain/*": ["domain/*"],
      "#application/*": ["application/*"],
      "#adapters/*": ["adapters/*"],
      "#infrastructure/*": ["infrastructure/*"]
    }
  },
  "include": ["domain/**/*.ts", "application/**/*.ts", "adapters/**/*.ts", "infrastructure/**/*.ts"]
}
```

**Path alias behavior**:
- Paths work for IDE autocomplete and `tsc --project`
- Runtime uses relative paths (e.g., `../../domain/types/index.ts`)
- Deno requires explicit relative paths without import maps

### Vertical Slice Structure

Each feature is a self-contained vertical slice:

```
src/features/gh-push/           # One slice per feature
├── domain/                     # Feature-specific domain
│   ├── types/                 # Entities
│   └── services/             # Pure functions
├── application/               # Use cases + ports
│   ├── ports/                # Interfaces
│   └── usecases/             # Business logic
├── adapters/                 # Implementations
└── infrastructure/           # External deps
```

**Cross-slice imports**: Not allowed. If two slices need shared code, that code should be extracted to a shared location or indicate a missing vertical slice boundary.

## Consequences

- **Positive**: Clear dependency boundaries enforce architectural discipline
- **Positive**: Testable - each layer can be mocked independently
- **Positive**: Screaming architecture - file names immediately reveal contents
- **Positive**: One file per type maximizes discoverability and minimizes merge conflicts
- **Positive**: Easy to spinoff as separate project
- **Negative**: More files than consolidated approaches
- **Negative**: Requires discipline to maintain layer boundaries

## References

- Uncle Bob's Clean Architecture: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
- Deno TypeScript config: https://docs.deno.com/runtime/reference/ts_config_migration/
- Related: [[ADR-001]], [[ADR-002]], [[ADR-003]], [[ADR-004]], [[ADR-005]]