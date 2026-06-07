---
type: adr
title: ADR-6 - Clean Architecture for sync-issues
date: 2026-04-07
status: accepted
supersedes: 
---

## Context

This ADR documents the Clean Architecture decisions and implementation details for the sync-issues GitHub Action. It consolidates and expands upon ADR-005's folder structure decisions with specific attention to layer dependencies, TypeScript configuration, and code organization.

References:
- Uncle Bob's Clean Architecture: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
- Deno TypeScript config: https://docs.deno.com/runtime/reference/ts_config_migration/
- Related: [[ADR-001]], [[ADR-002]], [[ADR-003]], [[ADR-004]], [[ADR-005]]

## Decision

### Options

1. **Ad-hoc layer organization** — Layers loosely defined without strict dependency rules or enforcement. Simple but prone to circular dependencies and architectural drift.
2. **Monolithic tsconfig** — A single tsconfig.json for the entire project with shared path aliases across slices. Simpler configuration but allows accidental cross-slice imports.
3. **Clean Architecture with per-slice tsconfig** — Four distinct layers (domain, application, adapters, infrastructure) with per-slice TypeScript configuration that enforces dependency direction and slice isolation at compile time.

### Rationale

**Clean Architecture Layers:** The sync-issues implementation follows Uncle Bob's Clean Architecture with four distinct layers:

```
.github/actions/sync-issues/sync-issues/
├── src/features/gh-push/
│   ├── domain/               # Entities only (innermost layer)
│   ├── application/          # Use cases + Ports
│   ├── adapters/            # Implementations
│   └── infrastructure/       # External dependencies
└── index.ts                  # Entry point (orchestrates layers)
```

**Layer Definitions:**

| Layer | Contents | Dependencies |
|-------|----------|--------------|
| **Domain** | Entities (IssueFile, GitHubIssue, etc.), pure functions (ParseFrontmatter, ExtractSections) | None (innermost) |
| **Application** | Use cases (SyncIssueUseCase), Port interfaces (IssueAdapterPort, FileAdapterPort) | Domain only |
| **Adapters** | Implementations (GhCliAdapter, NodeFileAdapter) | Application ports + Infrastructure |
| **Infrastructure** | External dependencies (RunGh, NodeIdToNumericId, node:fs) | None (outmost) |

**Dependency Direction Rule:** Core principle: all dependencies point inward. Inner layers never know about outer layers.

```
Application ──► Domain ◄── Infrastructure
       │                        ▲
       │                        │
       └──────── Adapters──────┘
              implements
```

**Practical implications:**
- Domain has zero imports from other layers
- Application imports from Domain (types) and Application (ports)
- Adapters import from Application (ports) and Infrastructure (helpers)
- Infrastructure imports only from node:fs, node:child_process

**Port/Adapter Pattern:** Following Uncle Bob's "Interface Adapters" layer, ports (interfaces) live in the Application layer and adapters (implementations) live in the Adapters layer. This ensures Application owns the interface contracts.

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

**One File Per Type (Screaming Architecture):** Each TypeScript file contains exactly one type or class. File names match the exact PascalCase of the entity:

- `IssueFile.ts` contains `IssueFile` type
- `GhCliAdapter.ts` contains `GhCliAdapter` class
- `SyncIssueUseCase.ts` contains `SyncIssueUseCase` class

**Convention override:** The common `types.ts` pattern is intentionally avoided. Screaming architecture prioritizes discoverability over file count.

**TypeScript Configuration:** Structure — base config at action level, per-slice configs extend it:

```
.github/actions/sync-issues/sync-issues/
├── tsconfig.base.json                 # Base configuration (action level)
├── package.json                       # Node.js package with import maps
└── src/features/
    ├── gh-push/
    │   ├── tsconfig.json              # Slice-level (extends base)
    │   └── domain/, application/, ...
    └── another-slice/
        └── tsconfig.json              # New slice copies same pattern
```

**tsconfig.base.json** (Node.js compatible):
```json
{
  "compilerOptions": {
    "target": "ES2022",
    "lib": ["ES2022"],
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "strict": true,
    "useUnknownInCatchVariables": true,
    "noImplicitOverride": true
  }
}
```

**tsconfig.json** (each slice):
```json
{
  "extends": "../../../tsconfig.base.json",
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

**TypeScript configuration design decisions:**

1. **One tsconfig per slice**: Each vertical slice has its own tsconfig.json that extends the base. This ensures slice isolation.

2. **Same path rules for all slices**: All slices use the same layer aliases (`#domain/*`, `#application/*`, etc.). This makes the codebase consistent and predictable.

3. **Cross-slice imports fail**: The `include` directive restricts each slice to its own files. Attempting to import from another slice (e.g., `import { X } from "other-slice/domain/..."`) fails at compile time because it is not in the slice's `include`. This enforces the vertical slice boundary.

4. **Path aliases are slice-relative**: Each slice's `baseUrl: "."` means `#domain/types` resolves to `domain/types/index.ts` within that slice only. There is no way to escape the slice.

5. **Node.js compatible**: Using `lib: ["ES2022"]` (not `deno.ns`) and `moduleResolution: "bundler"` makes it work with both tsc and Node.js.

6. **Runtime requires package.json imports**: For Deno runtime, path aliases need to be defined in `package.json` `imports` field:
```json
{
  "imports": {
    "#domain/types": "./src/features/gh-push/domain/types/index.ts"
  }
}
```

7. **Relative imports need .ts extension**: Within a slice, relative imports must include `.ts` extension (e.g., `from "./ParseFrontmatter.ts"`).

**Creating a new slice:**

```bash
# 1. Create slice directory structure
mkdir -p src/features/new-slice/{domain,application,adapters,infrastructure}/{types,ports,usecases}

# 2. Copy tsconfig from existing slice
cp src/features/gh-push/tsconfig.json src/features/new-slice/tsconfig.json

# 3. Update extends path in new-slice/tsconfig.json:
# Change "../../../tsconfig.base.json" to "../../../tsconfig.base.json"
```

The new slice automatically inherits the same strict settings and path aliases.

**Vertical Slice Structure:** Each feature is a self-contained vertical slice:

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

**Cross-slice imports:** Not allowed. If two slices need shared code, that code should be extracted to a shared location or indicate a missing vertical slice boundary.

## Consequences

### Positive

1. Clear dependency boundaries enforce architectural discipline
2. Testable — each layer can be mocked independently
3. Screaming architecture — file names immediately reveal contents
4. One file per type maximizes discoverability and minimizes merge conflicts
5. Easy to spinoff as separate project

### Negative

1. More files than consolidated approaches
2. Requires discipline to maintain layer boundaries
