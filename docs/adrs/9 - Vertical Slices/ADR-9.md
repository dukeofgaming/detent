---
type: adr
title: ADR-9 - Vertical Slices
date: 2026-06-20
status: accepted
supersedes:
---

## Context

detent grew from flat `src/bpmn/` and `src/mdx/` modules into multiple
capabilities — BPMN↔MDX transpilation, graph validation, and future standards
(SWS). We needed an organizing principle that:

1. Keeps each capability shippable and testable on its own branch/issue
2. Co-locates implementation, tests, and fixtures with the feature they exercise
3. Composes with Clean Architecture ([[ADR-5]]) without collapsing into a
   layer-first monolith
4. Scales as new workflow standards and engine features arrive

Vertical slicing was adopted in practice (commits `ba873c1`, `48f5413`; issue
#3/#4 journals) but never captured as a single ADR. Slice rules currently
appear piecemeal in [[ADR-3]] (types per slice), [[ADR-4]] (feature-root
`mod.rs`), [[ADR-5]] (layers inside slices), [[ADR-6]] (slice-owned tests),
[[ADR-7]] (screaming names), and [[ADR-8]] (BDD layout).

## Decision

**Organize feature code as vertical slices under `src/features/<feature>/`.**

Each slice is a self-contained directory that owns one user-visible capability
(or a tightly coupled sub-capability). Slices are the primary unit of
development, testing, and incremental delivery — not top-level `domain/`,
`adapters/`, or `infrastructure/` folders spanning the whole repo.

### Slice layout

```
src/
├── main.rs              # CLI composition only
├── lib.rs               # crate root; re-exports features/
└── features/
    ├── mod.rs           # declares slice modules
    └── <feature>/
        ├── mod.rs       # slice root (exception to [[ADR-4]] folder.rs rule)
        ├── domain/      # optional; standard-neutral semantics
        ├── use_cases/   # orchestration
        ├── adapters/    # format-specific parse/serialize/map
        ├── infrastructure/
        │   └── cli/     # thin command handlers for this slice
        └── tests/       # slice-owned tests + assets ([[ADR-6]], [[ADR-8]])
```

Current slices:

| Slice | Issue | Capability |
|-------|-------|------------|
| `convert_bpmn_to_mdx` | #3 | BPMN↔MDX compile/import |
| `graph_validation` | #4 | Graph invariants on workflow IR |

`src/assets/` holds **runtime** BPMN XSD schemas (optional validation per
[[ADR-3]]), not slice implementation.

### Slice rules

1. **One directory per capability.** Name the folder after what it does
   (`convert_bpmn_to_mdx`, `graph_validation`), not after a layer (`adapters/`).

2. **Layers live inside the slice.** Domain, use cases, adapters, and
   infrastructure are boundaries *within* a slice ([[ADR-5]]). Dependency
   direction still points inward.

3. **Slice owns its tests and fixtures.** Tests live under
   `src/features/<feature>/tests/`; Cargo discovers them via the root harness
   ([[ADR-6]]). Fixtures duplicate across slices when needed — no shared
   cross-slice test modules.

4. **No cross-slice adapter imports.** A slice must not `use` another slice's
   `adapters/` or `infrastructure/` modules. Shared semantics belong in a
   slice's `domain/` (or a future explicitly shared domain module), reached
   through adapter mappers at the boundary — not by reaching into a peer slice's
   BPMN types.

5. **CLI composes slices.** `main.rs` wires subcommands to slice
   `infrastructure/cli/` handlers. Slices may each expose their own CLI surface;
   duplication at the CLI layer is acceptable when slices stay independent.

6. **Feature flags gate optional slices.** Optional capabilities (e.g.
   `graph-validation`) compile as separate modules under `features/` rather
   than `#ifdef`-style clutter in a monolith.

7. **New work starts as a new slice.** Adding SWS support, execution stepping,
   or another standard adapter should add `src/features/<new_capability>/`
   rather than expanding an unrelated slice's adapter tree.

### Options

1. **Layer-first monolith** (`src/domain/`, `src/adapters/`, …): familiar Clean
   Architecture layout — rejected; cross-cutting folders obscure which code
   belongs to which shipped capability.

2. **Vertical slices under `src/features/`** — **chosen**; capability-first
   directories with internal layer boundaries.

3. **Micro-crates per slice** (workspace members): strongest isolation — deferred;
   unnecessary for current crate size and complicates WASM/CLI packaging.

### Rationale

1. **Issue-aligned delivery.** Each GitHub issue maps to one slice directory,
   making branch work and review scope obvious.

2. **Test locality.** Slice-owned tests ([[ADR-6]]) follow naturally from
   slice-owned code; BDD and integration assets stay beside the behavior they
   prove ([[ADR-8]]).

3. **Clean Architecture compatibility.** Vertical slicing and layer boundaries
   solve different problems. Slices answer *which feature*; layers answer
   *which concern* inside that feature. [[ADR-5]] applies inside each slice, not
   instead of slicing.

4. **Multi-standard growth.** Future adapters (SWS, etc.) add slices without
   rewriting the BPMN slice ([[ADR-5]] anti-corruption at adapter boundaries).

5. **Incremental graph validation.** `graph_validation` shipped as an additive
   slice isolated from `convert_bpmn_to_mdx` (#4 journal) — the pattern we want
   to repeat.

## Consequences

### Positive

1. Directory tree screams shipped capabilities before opening files ([[ADR-7]])
2. Slices can evolve, test, and land on separate timelines
3. Layer discipline is enforceable per slice without repo-wide refactors
4. Clear home for new standards and engine features

### Negative

1. Some duplication (fixtures, thin CLI handlers) across slices — intentional
2. Shared domain types require explicit extraction when a third slice needs them
3. `main.rs` must compose multiple slice CLIs as capabilities grow

## Related

- [[ADR-3]] — BPMN types live in the owning slice's adapter layer
- [[ADR-4]] — feature-root `mod.rs` keeps each slice physically self-contained
- [[ADR-5]] — layer boundaries apply inside each slice
- [[ADR-6]] — slice-owned test discovery
- [[ADR-7]] — screaming names within slice trees
- [[ADR-8]] — BDD layout under `tests/bdd/`
