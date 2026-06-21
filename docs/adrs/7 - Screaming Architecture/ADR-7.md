---
type: adr
title: ADR-7 - Screaming Architecture
date: 2026-06-06
status: accepted
supersedes:
---

## Context

Naming discipline started with **BPMN types only**. Early types grouped
`StartEvent` + `EndEvent` in `events.rs`, multiple tasks in `tasks.rs`, etc.
That was refactored (commit `3def0f6`) to **one type per file** under category
folders — `start_event.rs` → `StartEvent`, `exclusive_gateway.rs` →
`ExclusiveGateway` — with thin re-export modules (`events.rs`, `tasks.rs`).

That rule applied initially under flat `src/bpmn/types/`. As the repo moved to
**feature slices** (`ba873c1`, `48f5413`), types relocated to paths like
`src/features/convert_bpmn_to_mdx/adapters/bpmn/types/events/start_event.rs`
without changing the one-type-per-file rule.

By mid-2025 the same discoverability problems appeared elsewhere:

- flat `steps.rs` hiding Given/When/Then
- slice tests without strategy-visible directory names
- generic `types/` folders that do not scream domain concern

Commit `7486af4` extended screaming architecture **project-wide** (superseding
the type-only scope). Test restructuring followed: `bdd/scenarios/`,
`steps/{given,when,then,and}.rs`, `integration/` vs `unit/` ([[ADR-6]],
[[ADR-8]]).

Screaming architecture applies anywhere a developer asks "what lives here?":

- `tests/unit/` — pure unit tests for this slice
- `tests/bdd/world.rs` — Cucumber World entrypoint
- `tests/assets/hello_world/` — hello-world scenario fixtures
- `use_cases/compile.rs` — MDX → Definitions use case

## Decision

**Every file and directory must express its single primary concern in its name.**

### Type files (carried forward from 2026-01 one-type-per-file work)

- One logical type per file; filename matches type (`start_event.rs` → `StartEvent`)
- Category re-export files (`events.rs`, `gateways.rs`) are thin barrels only
- Prefer domain folders (`events/`, `tasks/`) over generic `types/` where practical

Current example:

```
src/features/convert_bpmn_to_mdx/adapters/bpmn/types/
├── events/start_event.rs
├── tasks/manual_task.rs
├── gateways/exclusive_gateway.rs
└── sequence_flow.rs
```

### Tests, fixtures, and tooling

| Avoid | Replace with |
|-------|--------------|
| `steps.rs` | `steps/given.rs`, `steps/when.rs`, `steps/then.rs` |
| flat `tests/*.rs` mixing strategies | `tests/bdd/`, `tests/unit/`, `tests/integration/` |
| `tests/fixtures/` (ambiguous) | `tests/assets/<scenario>/` |
| `utils/`, `common/`, `helpers/` | name the actual concern |

BDD layout detail: [[ADR-8]].

Build/discovery entrypoints use descriptive names:
`tests/feature_slices.rs`, `tests/feature_slices_cucumber.rs`, `build.rs`.

### Options

1. **Type-only screaming** (2026-01 scope) — insufficient once slices and BDD grew
2. **Project-wide screaming** — **chosen**
3. **Conventional flat Rust layout** — poor fit for multi-slice + BDD

### Rationale

Layer folders (`domain/`, `use_cases/`, `adapters/`, `infrastructure/`) already
signal Clean Architecture boundaries ([[ADR-5]] proposal). Test and adapter
trees should be equally glanceable. One-type-per-file reduces merge conflicts and
makes IDE navigation ("go to file") map directly to domain concepts.

Generic `MdxFile::parse_as<T>()` complements typed parsers without violating
one-concern-per-file for BPMN element structs.

## Consequences

### Positive

1. Directory tree documents architecture without opening files
2. `rg` and fuzzy-find map directly to concepts
3. Scales as BPMN coverage and slice count grow
4. Consistent with [[ADR-4]] `folder.rs` layer modules and feature `mod.rs` roots

### Negative

1. More files and nesting vs monolithic modules
2. Residual generic names (e.g. `adapters/bpmn/types/`) remain — rename deferred
3. Re-export barrels add small maintenance overhead

## Related

- [[ADR-4]] — `folder.rs` for layers; `mod.rs` only at feature roots
- [[ADR-6]] — screaming test directory strategy
- [[ADR-8]] — BDD file naming inside `bdd/`
