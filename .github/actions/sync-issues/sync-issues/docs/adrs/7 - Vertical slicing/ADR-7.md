---
type: adr
title: ADR-7 - Vertical Slicing Architecture
date: 2026-04-07
status: accepted
supersedes: 
---

## Context

The sync-issues tool needs multiple commands (push, pull, register) that share similar domain logic but have different external interfaces. We evaluated several architectural approaches:

1. **Monolithic single slice** — Everything in one package, imports across "modules"
2. **Shared library** — Common code in a shared package, imported by both slices
3. **Vertical slicing** — Each command is a self-contained slice with its own copy of domain code

References:
- Uncle Bob's Clean Architecture: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
- Vertical Slicing: https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell
- Related: [[ADR-6]] (Clean Architecture)

## Decision

### Options

1. **Monolithic single slice** — Everything in one package with imports across "modules". Simpler initially but leads to tight coupling and cascading changes.
2. **Shared library** — Common domain code extracted into a shared package imported by both slices. Reduces duplication but creates hidden coupling and a "shared" → "abandoned" anti-pattern risk.
3. **Vertical slicing** — Each command family is a completely isolated slice with its own copy of domain code. Duplicates code intentionally to maximize independence and evolution flexibility.

### Rationale

**Vertical slicing architecture:** Each command family is a completely isolated slice:

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

**Clean Architecture Within Each Slice:** Each slice follows Clean Architecture independently:

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

**Code Duplication is Intentional:** The following code is deliberately copied between slices, not shared:
- Domain types (`IssueFile`, `GitHubIssue`, etc.)
- Domain services (`parseFrontmatter`, `ExtractSections`, etc.)
- Port interfaces (`IssueAdapterPort`, `FileAdapterPort`)
- Infrastructure components (`CliParser`, `CommandRegistry`)

**Why duplication is intentional:**

1. **Slice independence**: Each slice can evolve independently without affecting others
2. **No hidden coupling**: No accidental dependencies through shared code
3. **Explicit boundaries**: The architecture enforces separation at the file level
4. **Technology flexibility**: Slices can use different implementations if needed
5. **Debugging clarity**: Issues in one slice do not cascade to others
6. **Onboarding clarity**: Each slice is self-contained, easier to understand

**When to share code:** Only when it represents a genuine shared abstraction that both slices truly share. Even then, prefer duplication over wrong abstraction.

**TypeScript Configuration Enforcement:** Each slice has its own `tsconfig.json` that:
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

**CLI Routing:** The main `index.ts` routes to the appropriate slice based on the first argument:

```typescript
const args = Deno.args;
const primaryCommand = args[0] || "help";

if (primaryCommand === "push") {
  // Use gh-push slice
} else if (primaryCommand === "pull" || primaryCommand === "register") {
  // Use gh-pull slice
}
```

**Slice Responsibilities:**

**gh-push slice:**
- Purpose: Push local markdown issues to GitHub
- Writes to GitHub: Yes (issues, comments)
- Commands: `push`
- Direction: Local → GitHub

**gh-pull slice:**
- Purpose: Read from GitHub, update local markdown files
- Writes to GitHub: No (read-only)
- Commands: `pull`, `register`
- Direction: GitHub → Local

Note: The gh-pull slice NEVER modifies GitHub. All operations are read from GitHub (issues, comments) and write to local files (markdown with references).

## Consequences

### Positive

1. Slices are independently testable
2. No hidden cross-slice dependencies
3. Each slice can evolve independently
4. Clear boundaries prevent feature creep
5. Easier to onboard new developers (one slice at a time)
6. No "shared" become "abandoned" anti-pattern

### Negative

1. More code overall (duplication)
2. Must maintain consistency manually across slices
3. Refactoring domain logic requires changes in multiple slices
4. Need to document that duplication is intentional
5. Mitigation: document architectural decisions in ADRs, regular code review to catch drift, consider scripting to update multiple slices when domain changes
