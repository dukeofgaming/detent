---
type: adr
title: ADR-6 - Slice Test Harness
date: 2026-06-01
status: accepted
supersedes:
---

## Context

Slice-owned tests went through several wiring models before this ADR. The **goal
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

6. **Screaming test layout** (2025-06, [[ADR-7]] / [[ADR-8]]): within each
   slice, tests split into `integration/mod.rs`, optional `unit/mod.rs`, and
   `bdd/world.rs` with `scenarios/`, `steps/`, and `assets/hello_world/` etc.

7. **BDD at every test level** (2026-06): all slice tests are Cucumber BDD
   scenarios grouped by test level (`unit`, `functional`, `integration`, `e2e`).
   Plain `#[test]` modules and the separate `feature_slices_cucumber` harness
   were removed.

Current discovery (`build.rs`):

- `src/features/*/tests/world.rs`

Root entrypoint:

- `tests/feature_slices.rs` → one async Cucumber runner per slice

## Decision

Keep slice test **files and fixtures** under `src/features/<feature>/tests/`.
Expose them to Cargo through a **single generated root harness** — not
`#[cfg(test)] mod tests` in library code, and not per-slice `[[test]]` TOML entries.

```rust
// tests/feature_slices.rs
include!(concat!(env!("OUT_DIR"), "/feature_slices.rs"));
```

`build.rs` scans slice directories for `tests/world.rs` and emits `#[path = "..."] mod` wiring plus one `#[test]` function per slice that calls `run()`.

Rules:

- Slice tests: `src/features/<feature>/tests/{unit,functional,integration,e2e}/` (see [[ADR-8]])
- Slice entrypoint: `src/features/<feature>/tests/world.rs`
- Slice fixtures: `src/features/<feature>/tests/assets/<scenario>/`
- No `#[cfg(test)] mod tests` in production slice code unless testing private
  internals with no better seam
- Slices duplicate fixtures where needed — no shared cross-slice test modules

### Options

| Approach | Outcome |
|----------|---------|
| `#[cfg(test)]` in feature root | Wrong target for CLI tests — **rejected** |
| Per-slice `[[test]]` in Cargo.toml | Correct targets, TOML churn — **superseded** |
| Root harness + `build.rs` | Stable discovery, slice ownership — **chosen** |
| Separate plain-test and Cucumber harnesses | Two binaries, duplicated discovery — **superseded** |

### Rationale

Combines slice locality with correct Cargo targets without ongoing TOML
maintenance. One harness keeps `cargo test` simple — no extra `--test` flags.

## Consequences

### Positive

1. Slice owns tests and fixtures; root `tests/` is a thin stable harness
2. `cargo test --lib` stays limited to actual library unit tests
3. New scenarios in existing slice feature files need no manifest changes
4. New slice needs only `tests/world.rs` (and level directories per [[ADR-8]])
5. Layout aligns with [[ADR-7]] (level directories scream test type) and [[ADR-8]]

### Negative

1. `build.rs` indirection — slightly harder to trace than explicit `mod` trees
2. Harness tests primarily use public slice APIs
3. Per-slice fixture duplication (intentional — slices stay independent)

## Related

- [[ADR-7]] — screaming names for test directories
- [[ADR-8]] — BDD file layout by test level
- [[ADR-9]] — slice as the unit that owns tests and fixtures
