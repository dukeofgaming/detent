# ADR-8: Organize tests as slice-local fractals

## Status

Accepted

## Date

2026-05-07

## Context

Integration tests were previously placed in project-root `tests/` directories
with `#[path]` harness files (e.g., `tests/compiler.rs`,
`tests/graph_validation.rs`). This created a disconnect between code and its
tests: the files lived far from the implementation they tested, and the
`#[path]` pattern added boilerplate.

Tests were then collapsed into single `{module}/tests.rs` files next to the
implementation. That improved locality, but it flattened the test layout too
much for feature slices:

- slice-specific fixtures still wanted their own directory structure
- larger slices lost a natural place to split test modules
- the layout no longer mirrored the project-level `tests/assets/` pattern

For a feature-sliced codebase, each vertical slice should look like a smaller
version of the project: test code in `tests/`, fixtures in `tests/assets/`, and
ownership kept inside the slice.

## Decision

Organize tests inside each feature slice under a `tests/` subtree.

Examples:

- `src/features/convert_bpmn_to_mdx/tests/mod.rs`
- `src/features/convert_bpmn_to_mdx/tests/assets/`
- `src/features/graph_validation/tests/mod.rs`
- `src/features/graph_validation/tests/assets/`

Rules:

- test modules for a slice live under `src/features/<feature>/tests/`
- slice-owned fixtures live under `src/features/<feature>/tests/assets/`
- the project root `tests/assets/` may remain for truly cross-slice or
  whole-project fixtures
- slice-specific fixtures should not live at the project root

Each feature root declares the test module with:

```rust
#[cfg(test)]
mod tests;
```

where Rust resolves `mod tests;` to `tests/mod.rs`.

## Consequences

**Positive:**
- Tests sit next to the code they exercise
- No project-root `#[path]` harness boilerplate needed
- Each slice owns both its test code and its fixture assets
- The test layout mirrors the project's top-level `tests/assets/` pattern
- Private module internals remain accessible via `crate::`
- Test suites can grow by splitting into focused files under `tests/`

**Negative:**
- Test code is compiled as part of the crate (but `#[cfg(test)]` strips it from
  release builds)
- More files and folders per slice, which adds some navigation overhead for
  very small features

## Related

- [[ADR-7]]: Domain-layer graph validation
- Supersedes the previous project-root `tests/compiler.rs` and
  `tests/graph_validation.rs` harness layout
- Supersedes the intermediate single-file `{module}/tests.rs` convention
