---
type: adr
title: ADR-10 - Use a stable root harness for slice-owned integration tests
date: 2026-06-01
status: accepted
supersedes: "8, 9"
---

## Context

Slice-owned tests went through three wiring models before this ADR. The **goal
stayed constant** throughout: tests and fixtures live inside the owning feature
slice; project-root `tests/` is only a Cargo discovery shim.

### Evolution (from branch history)

1. **Project-root harness** (early): `tests/compiler.rs`-style `#[path]` files
   far from implementation — rejected for slice work.

2. **Slice-local `#[cfg(test)] mod tests`** (2025-05, issue #3 journal): tests under
   `src/features/<feature>/tests/` with fixtures in `tests/assets/`. Co-located
   but compiled into the **library test target**, so CLI integration tests
   (`assert_cmd::Command::cargo_bin("detent")`) ran under `cargo test --lib`.

3. **Explicit `[[test]]` per slice** (intermediate): each slice registered
   `src/features/<feature>/tests/integration.rs` in `Cargo.toml`. Fixed the lib
   vs integration target split, but every new slice root required a TOML edit.

4. **Feature-slice split** (2025): `graph_validation` separated from
   `convert_bpmn_to_mdx`; slice-local assets duplicated per slice by design.

5. **Root harness + `build.rs`** (commits `5180fb4`, `177db01`, `65fdd3c`): stable
   `tests/feature_slices.rs` includes generated modules from slice paths — no
   per-slice `Cargo.toml` registration.

6. **Screaming test layout** (2025-06, [[ADR-11]] / [[ADR-12]]): within each
   slice, tests split into `integration/mod.rs`, optional `unit/mod.rs`, and
   `bdd/world.rs` with `scenarios/`, `steps/`, and `assets/hello_world/` etc.

Current discovery (`build.rs`):

- `src/features/*/tests/integration/mod.rs`
- `src/features/*/tests/unit/mod.rs` (when present)
- `src/features/*/tests/bdd/world.rs`

Root entrypoints:

- `tests/feature_slices.rs` → integration + unit modules
- `tests/feature_slices_cucumber.rs` → BDD worlds

## Decision

Keep slice test **files and fixtures** under `src/features/<feature>/tests/`.
Expose them to Cargo through **generated root harness** files — not
`#[cfg(test)] mod tests` in library code, and not per-slice `[[test]]` TOML entries.

```rust
// tests/feature_slices.rs
include!(concat!(env!("OUT_DIR"), "/feature_slices.rs"));
```

`build.rs` scans slice directories and emits `#[path = "..."] mod` wiring.

Rules:

- Slice tests: `src/features/<feature>/tests/{integration,unit,bdd}/`
- Slice fixtures: `src/features/<feature>/tests/assets/<scenario>/`
- No `#[cfg(test)] mod tests` in production slice code unless testing private
  internals with no better seam ([[ADR-12]] BDD/integration cover public behavior)
- Slices duplicate fixtures where needed — no shared cross-slice test modules

### Options

| Approach | Outcome |
|----------|---------|
| `#[cfg(test)]` in feature root | Wrong target for CLI tests — **rejected** |
| Per-slice `[[test]]` in Cargo.toml | Correct targets, TOML churn — **superseded** |
| Root harness + `build.rs` | Stable discovery, slice ownership — **chosen** |

### Rationale

Combines slice locality (from the 2025 co-location work) with correct Cargo
targets (from the explicit-`[[test]]` experiment) without ongoing TOML
maintenance. BDD and unit roots discovered the same way as integration.

## Consequences

### Positive

1. Slice owns tests and fixtures; root `tests/` is a thin stable harness
2. `cargo test --lib` stays limited to actual library unit tests
3. New tests in existing slice files need no manifest changes
4. New slice needs only `tests/integration/mod.rs` (and optional unit/BDD roots)
5. Layout aligns with [[ADR-11]] (`integration/`, `unit/`, `bdd/`) and [[ADR-12]]

### Negative

1. `build.rs` indirection — slightly harder to trace than explicit `mod` trees
2. Harness tests primarily use public slice APIs
3. Per-slice fixture duplication (intentional — slices stay independent)

## Related

- [[ADR-11]] — screaming names for test directories
- [[ADR-12]] — BDD file layout inside `bdd/`
