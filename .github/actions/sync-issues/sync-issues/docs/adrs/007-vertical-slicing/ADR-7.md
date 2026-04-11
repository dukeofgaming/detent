---
type: adr
date: 2026-04-07
status: accepted
---
# ADR-7: Vertical Slicing Architecture

## Context

The sync-issues tool needs multiple commands (push, pull, register) that share similar domain logic but have different external interfaces. We evaluated several architectural approaches:

1. **Monolithic single slice** - Everything in one package, imports across "modules"
2. **Shared library** - Common code in a shared package, imported by both slices
3. **Vertical slicing** - Each command is a self-contained slice with its own copy of domain code

## Decision

We adopt **vertical slicing** where each command family is a completely isolated slice:

```
.github/actions/sync-issues/sync-issues/
├── src/features/
│   ├── gh-push/           # Push slice - writes to GitHub
│   │   ├── domain/        # Entities, services (copied, not shared)
│   │   ├── application/  # Use cases, ports
│   │   ├── adapters/     # Implementations
│   │   └── infrastructure/ # CLI, commands
│   └── gh-pull/           # Pull slice - reads from GitHub
│       ├── domain/        # COPY of domain logic (no sharing)
│       ├── application/
│       ├── adapters/
│       └── infrastructure/
└── index.ts               # Routes to appropriate slice
```

## Why Vertical Slicing?

### Clean Architecture Within Each Slice

Each slice follows Clean Architecture independently:

| Layer | Contents | Dependency Rule |
|-------|----------|-----------------|
| **Domain** | Entities, value objects, pure services | No dependencies (innermost) |
| **Application** | Use cases, port interfaces | Depends only on Domain |
| **Adapters** | Implementations of ports | Depends on Application + Infrastructure |
| **Infrastructure** | External I/O, CLI framework | Depends on nothing (outermost) |

**Example slice structure:**
```
src/features/gh-push/
├── domain/
│   ├── types/
│   │   ├── IssueFile.ts
│   │   ├── GitHubIssue.ts
│   │   └── ...
│   └── services/
│       ├── ParseFrontmatter.ts
│       ├── ExtractSections.ts
│       └── ...
├── application/
│   ├── ports/
│   │   ├── IssueAdapterPort.ts
│   │   └── FileAdapterPort.ts
│   └── usecases/
│       └── SyncIssueUseCase.ts
├── adapters/
│   ├── GhCliAdapter.ts
│   └── NodeFileAdapter.ts
└── infrastructure/
    ├── CommandRegistry.ts
    ├── CliParser.ts
    └── PushCommand.ts
```

### Code Duplication is Desirable

**Copied code (not shared):**
- Domain types (`IssueFile`, `GitHubIssue`, etc.)
- Domain services (`parseFrontmatter`, `ExtractSections`, etc.)
- Port interfaces (`IssueAdapterPort`, `FileAdapterPort`)
- Infrastructure components (`CliParser`, `CommandRegistry`)

**Why duplication is intentional:**
1. **Slice independence**: Each slice can evolve independently without affecting others
2. **No hidden coupling**: No accidental dependencies through shared code
3. **Explicit boundaries**: The architecture enforces separation at the file level
4. **Technology flexibility**: Slices can use different implementations if needed
5. **Debugging clarity**: Issues in one slice don't cascade to others
6. **Onboarding clarity**: Each slice is self-contained, easier to understand

**When to share code:**
- Only when it represents a genuine shared abstraction that both slices truly share
- Even then, prefer duplication over wrong abstraction

### TypeScript Configuration Enforcement

Each slice has its own `tsconfig.json` that:
1. Extends the base config
2. Sets `baseUrl: "."` (slice-relative)
3. Defines paths only within the slice (`#domain/*`, `#application/*`, etc.)
4. Has `include` that restricts to only the slice's files

This makes **cross-slice imports fail at compile time**:
```typescript
// gh-pull/domain/services/ParseIssueFile.ts - THIS FAILS
import { parseFrontmatter } from "../../gh-push/domain/services/ParseFrontmatter.ts";
// Error: File is not in the "include" of this tsconfig
```

### CLI Routing

The main `index.ts` routes to the appropriate slice based on the first argument:
```typescript
const args = Deno.args;
const primaryCommand = args[0] || "help";

if (primaryCommand === "push") {
  // Use gh-push slice
} else if (primaryCommand === "pull" || primaryCommand === "register") {
  // Use gh-pull slice
}
```

## Slice Responsibilities

### gh-push Slice
- **Purpose**: Push local markdown issues to GitHub
- **Writes to GitHub**: Yes (issues, comments)
- **Commands**: `push`
- **Direction**: Local → GitHub

### gh-pull Slice
- **Purpose**: Read from GitHub, update local markdown files
- **Writes to GitHub**: No (read-only)
- **Commands**: `pull`, `register`
- **Direction**: GitHub → Local

**Note**: The gh-pull slice NEVER modifies GitHub. All operations are:
- Read from GitHub (issues, comments)
- Write to local files (markdown with references)

## Consequences

### Positive
- Slices are independently testable
- No hidden cross-slice dependencies
- Each slice can evolve independently
- Clear boundaries prevent feature creep
- Easier to onboard new developers (one slice at a time)
- No "shared" become "abandoned" anti-pattern

### Negative
- More code overall (duplication)
- Must maintain consistency manually across slices
- Refactoring domain logic requires changes in multiple slices
- Need to document that duplication is intentional

### Mitigation
- Document architectural decisions in ADRs
- Regular code review to catch drift
- Consider scripting to update multiple slices when domain changes

## References

- Uncle Bob's Clean Architecture: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
- Vertical Slicing: https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell
- Related: [[ADR-6]] (Clean Architecture)