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

Current discovery (explicit harness):

- `tests/feature_slices.rs` declares each slice world module via `#[path]` and
  one `#[test] fn` per slice that calls `run()`.

Root entrypoint:

- `tests/feature_slices.rs` → one async Cucumber runner per slice
- No `build.rs` generation — harness is a regular source file visible to
  all tools at edit time.

### automod research

`automod` (a proc-macro crate that scans directories at compile time to
auto-generate `mod` declarations) was evaluated and rejected for test
wiring. It adds a new dependency, introduces proc-macro expansion
complexity, and does not integrate well with the `folder.rs` convention
when scanning subdirectories of a module file. Standard `mod` declarations
in a `{level}.rs` file are simpler and more explicit — one line per
scenario module, zero tooling surprises.

## Decision

Keep slice test **files and fixtures** under `src/features/<feature>/tests/`.
Expose them to Cargo through a **single explicit root harness** — not
`#[cfg(test)] mod tests` in library code, not per-slice `[[test]]` TOML
entries, and not a build-script-generated file.

```rust
// tests/feature_slices.rs
#[path = "../src/features/convert_bpmn_to_mdx/tests/world.rs"]
mod convert_bpmn_to_mdx;

#[test]
fn convert_bpmn_to_mdx() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    assert!(!rt.block_on(convert_bpmn_to_mdx::run()));
}
```

`tests/feature_slices.rs` declares each slice module with one `#[path]`
attribute pointing at the slice's `world.rs`. Inside the slice, all module
resolution uses standard Rust `mod` declarations ([[ADR-4]] `folder.rs`
pattern) — zero `#[path]` attributes inside slice-owned test code.

Rules:

- Slice tests: `src/features/<feature>/tests/{unit,functional,integration,e2e}/` (see [[ADR-8]])
- Slice entrypoint: `src/features/<feature>/tests/world.rs`
- Level modules: `{level}.rs` at `tests/` declares child scenario modules and `mod steps;` via standard Rust resolution
- Scenario files live directly in `{level}/`, not in a `scenarios/` subdirectory ([[ADR-8]])
- Slice fixtures: `src/features/<feature>/tests/assets/<scenario>/`
- No `#[cfg(test)] mod tests` in production slice code unless testing private
  internals with no better seam
- No `#[path]` attributes inside slice-owned test code — all module resolution
  uses standard `mod` ([[ADR-4]] `folder.rs` pattern)
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
