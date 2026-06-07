---
type: adr
title: ADR-9 - Use explicit slice integration test targets
date: 2026-06-01
status: accepted
supersedes: 8
---

## Context

[[ADR-8]] established that each feature slice owns its own `tests/` subtree and
`tests/assets/` fixtures. In practice, the initial implementation of that ADR
used `#[cfg(test)] mod tests;` inside the slice root, which made Rust compile
those slice tests as part of the library test target.

That arrangement caused two problems:

- `cargo test --lib` executed slice tests that were not really library-unit
  tests
- CLI-facing slice tests depended on the binary target being available during
  the library test run

For example, `assert_cmd::Command::cargo_bin("detent")` is appropriate for
binary integration coverage, but it is a poor fit when those tests are wired
into the library test target through `#[cfg(test)]` modules.

We still want the ownership and locality introduced by [[ADR-8]]:

- tests should remain inside the owning feature slice
- fixtures should remain inside the owning feature slice
- project-root harness files should remain unnecessary for slice-specific tests

What needs to change is how those slice tests are compiled and discovered.
This ADR superseded [[ADR-8]] for how slice tests are compiled and registered,
and has been superseded by [[ADR-10]].

## Decision

Keep slice tests under `src/features/<feature>/tests/`, but register them as
explicit integration test targets in `Cargo.toml` instead of wiring them into
library modules with `#[cfg(test)] mod tests;`.

### Options

1. **Keep `#[cfg(test)]` wiring** (from ADR-8): Simple but forced bin tests into lib target
2. **Explicit `[[test]]` targets in Cargo.toml**: Clean separation but requires per-slice TOML entries — chosen
3. **Root `tests/` harness per slice**: Stable discovery but requires root boilerplate

### Rationale

Slice integration test roots live under `src/features/<feature>/tests/` and
slice-owned fixtures under `src/features/<feature>/tests/assets/`. Slice tests
are registered as explicit `[[test]]` targets in `Cargo.toml` rather than wired
into library modules with `#[cfg(test)] mod tests;`. The project root
`tests/assets/` may remain for truly cross-slice or whole-project fixtures, but
slice-specific fixtures should not live at the project root.

Examples:

- `src/features/convert_bpmn_to_mdx/tests/integration.rs`
- `src/features/convert_bpmn_to_mdx/tests/assets/`
- `src/features/graph_validation/tests/integration.rs`
- `src/features/graph_validation/tests/assets/`

```toml
[[test]]
name = "convert_bpmn_to_mdx_slice_tests"
path = "src/features/convert_bpmn_to_mdx/tests/integration.rs"
```

## Consequences

### Positive

1. Tests remain owned by the feature slice they exercise
2. No project-root `#[path]` harness boilerplate is needed
3. `cargo test --lib` remains limited to actual library tests
4. CLI-oriented slice tests can use binary integration patterns without being forced into the library test target
5. Each slice still owns both its test code and its fixture assets

### Negative

1. `Cargo.toml` must explicitly list nonstandard slice test targets
2. Tests exercise the public slice surface instead of private internals unless a narrower seam is added
3. There is a small amount of extra target registration maintenance
