---
type: adr
date: 2026-06-06
status: accepted
---
# ADR-11: Screaming architecture for directories and files

## Context

Our initial screaming architecture work (ADR-5) established a one-type-per-file
convention for BPMN domain types, but the principle was applied only to the
`src/bpmn/types/` tree. As the project grew, other parts of the codebase —
feature slices, their internal layers, and especially test structures — silently
adhered to or violated the same idea: that a file or directory should "scream"
its contents without needing to be opened.

Screaming architecture doesn't end at types. It applies anywhere a developer
needs to answer "what lives here?" at a glance:

- `src/features/graph_validation/tests/unit/` should scream "these are unit
  tests for graph_validation"
- `tests/bdd/world.rs` should scream "this is the Cucumber World"
- `tests/bdd/steps/given.rs` should scream "these are Given step definitions"
- `tests/assets/hello_world/` should scream "this is the hello-world fixture set"

When directory and file names lack a one-to-one mapping to their contents,
developers scan, guess, or open files just to orient themselves.

## Decision

**Adopt screaming architecture across the entire project**, not just in type
definitions. Every file and every directory must express its single, primary
concern in its name.

### Rules

1. **One physical concern per file.** A file should contain exactly one logical
   concept. Exceptions: thin re-export modules (`mod.rs`), trivial helper
   closures shared only within a single parent.

2. **One type per file** (carried forward from ADR-5). A file named
   `start_event.rs` defines `StartEvent` and nothing else. A file named
   `task.rs` defines `Task` and nothing else.

3. **Directories name domain slices, not technical roles.** A directory
   `exclusive_gateways/` screams "this is about exclusive gateways", while
   `events/` screams "this is about events". Avoid `types/`, `utils/`,
   `helpers/`, `common/`, `shared/`.

4. **Test directories scream strategy.** `bdd/` screams "these are BDD/Cucumber
   tests", `unit/` screams "these are pure unit tests", `integration/` screams
   "these test multi-component interactions." A single `tests/` with flat files
   would not reveal which approach each file follows.

5. **Step definition files scream step type.** A file `given.rs` screams "I
   contain Given step definitions", `when.rs` screams "I contain When step
   definitions", etc.

6. **Fixture directories scream scenario.** A directory
   `tests/assets/hello_world/` screams "these fixtures belong to the
   hello-world scenario".

7. **Build entrypoints scream purpose.** `build.rs` screams "I am a Cargo build
   script", `tests/feature_slices.rs` screams "I discover feature-slice tests",
   `tests/feature_slices_cucumber.rs` screams "I discover BDD test worlds".

### Counter-examples to avoid

| Avoid | Screams | Replace with |
|-------|---------|--------------|
| `steps.rs` | nothing specific | `steps/given.rs`, `steps/when.rs`, `steps/then.rs` |
| `tests/tests.rs` | nothing specific | `tests/bdd/`, `tests/unit/`, `tests/integration/` |
| `bpmn/types/mod.rs` | "here be types" | BPMN domain folders (`events/`, `tasks/`) |

## Rationale

### Discoverability without opening files

A developer navigating the test tree sees:

```
tests/
├── bdd/
│   ├── world.rs             # Cucumber World
│   ├── slice.feature         # Gherkin features for this slice
│   ├── scenarios/            # one .rs file per scenario
│   └── steps/
│       ├── given.rs          # Given step defs
│       ├── when.rs           # When step defs
│       ├── then.rs           # Then step defs
│       └── and.rs            # And step defs
├── unit/                     # pure unit tests
└── integration/              # multi-component tests
```

Every name conveys intent. A new contributor can guess the purpose of each
file without opening it.

### Contrast with conventional Rust layout

The standard Rust test convention places a single `tests/` directory at root
with flat `*.rs` files. That layout does not scale to a multi-slice project:
slices blur together, and there is no way to distinguish BDD from unit from
integration tests at the filesystem level.

### Consistent with Clean Architecture folders

Feature slices already use layer folders: `domain/`, `use_cases/`,
`adapters/`, `infrastructure/`. Each folder name screams which Clean
Architecture layer it holds. The test structure mirrors this precision.

## Consequences

### Positive
- **Glanceability.** File browsability becomes a reliable substitute for
  deep understanding.
- **Searchability.** `rg "Given" --glob "given.rs"` finds step definitions
  instantly.
- **Self-documenting.** The directory tree doubles as architecture
  documentation.
- **Onboarding.** New contributors navigate by intuition rather than
  tribal knowledge.

### Negative
- **More files.** Refactoring `steps.rs` into `steps/given.rs`,
  `steps/when.rs`, etc. increased file count.
- **Deeper nesting.** `skipping/space/bar/` in editors and diff tools can be
  cumbersome.

### Mitigations
- IDE file navigation and fuzzy-finder (`Ctrl-P`, `Cmd-P`) make file count
  irrelevant.
- Modern editors collapse directory trees or provide flat-file search.
- `mod.rs` files remain thin (2–4 lines) and never accumulate logic.

## Related

- Supersedes ADR-5: Screaming Architecture with One Type Per File
- Referenced by [[ADR-12]](ADR-12.md): BDD Test Layout
- Robert C. Martin, "Screaming Architecture" (Clean Architecture, Chapter 21)
