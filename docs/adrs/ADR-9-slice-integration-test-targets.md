---
type: adr
date: 2026-06-01
status: superseded
---
# ADR-9: Use explicit slice integration test targets

## Context

ADR-8 established that each feature slice owns its own `tests/` subtree and
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

We still want the ownership and locality introduced by ADR-8:

- tests should remain inside the owning feature slice
- fixtures should remain inside the owning feature slice
- project-root harness files should remain unnecessary for slice-specific tests

What needs to change is how those slice tests are compiled and discovered.

## Decision

Keep slice tests under `src/features/<feature>/tests/`, but register them as
explicit integration test targets in `Cargo.toml` instead of wiring them into
library modules with `#[cfg(test)] mod tests;`.

Examples:

- `src/features/convert_bpmn_to_mdx/tests/integration.rs`
- `src/features/convert_bpmn_to_mdx/tests/assets/`
- `src/features/graph_validation/tests/integration.rs`
- `src/features/graph_validation/tests/assets/`

Rules:

- slice integration test roots live under `src/features/<feature>/tests/`
- slice-owned fixtures live under `src/features/<feature>/tests/assets/`
- slice tests are registered as explicit `[[test]]` targets in `Cargo.toml`
- slice tests should not be wired into library modules with
  `#[cfg(test)] mod tests;`
- the project root `tests/assets/` may remain for truly cross-slice or
  whole-project fixtures
- slice-specific fixtures should not live at the project root

Example registration:

```toml
[[test]]
name = "convert_bpmn_to_mdx_slice_tests"
path = "src/features/convert_bpmn_to_mdx/tests/integration.rs"
```

## Consequences

**Positive:**
- Tests remain owned by the feature slice they exercise
- No project-root `#[path]` harness boilerplate is needed
- `cargo test --lib` remains limited to actual library tests
- CLI-oriented slice tests can use binary integration patterns without being
  forced into the library test target
- Each slice still owns both its test code and its fixture assets

**Negative:**
- `Cargo.toml` must explicitly list nonstandard slice test targets
- Tests exercise the public slice surface instead of private internals unless a
  narrower seam is added
- There is a small amount of extra target registration maintenance

## Related

- ADR-8: Organize tests as slice-local fractals
- Supersedes ADR-8 for how slice tests are compiled and registered
- Superseded by ADR-10
