# ADR-8: Co-locate tests with implementation slices

## Status

Accepted

## Date

2026-05-07

## Context

Integration tests were previously placed in `tests/` directories with `#[path]`
harness files (e.g., `tests/compiler.rs`, `tests/graph_validation.rs`). This
created a disconnect between code and its tests — the files lived far from the
implementation they tested, and the `#[path]` pattern added boilerplate.

## Decision

Co-locate tests in `{module}/tests.rs` files with `#[cfg(test)]` gating:

- `src/features/convert_bpmn_to_mdx/tests.rs` — all `#3` slice tests
  (BPMN parsing, CLI, compile/import logic, MDX round-trip)
- `src/features/graph_validation/tests.rs` — all `#4` slice tests
  (Graph operations, semantic checks, reachability)

Each `{module}/mod.rs` declares the test module with:

```rust
#[cfg(test)]
mod tests;
```

## Consequences

**Positive:**
- Tests sit next to the code they exercise
- No `#[path]` harness boilerplate needed
- Private module internals remain accessible via `crate::`
- Cleaner project root (`tests/` now contains only asset fixtures)

**Negative:**
- Test code is compiled as part of the crate (but `#[cfg(test)]` strips it from
  release builds)
- Larger module files, but each test module is self-contained and clearly named

## Related

- ADR-7: Domain-layer graph validation
- Supersedes the previous `tests/compiler/` and `tests/graph_validation/`
  integration test layout
