# ADR-10: Use a stable root harness for slice-owned integration tests

## Status

Accepted

## Date

2026-06-01

## Context

ADR-9 improved on ADR-8 by moving slice-owned tests out of the library test
target. However, its mechanism required one `[[test]]` registration in
`Cargo.toml` per slice test root.

That registration model fixes the `cargo test --lib` problem, but it adds a new
maintenance burden: every new slice integration root requires a `Cargo.toml`
edit.

We still want all of the following:

- slice tests stay physically owned by the feature slice
- slice fixtures stay under the owning feature slice
- `cargo test --lib` does not run slice integration tests
- new slice tests do not require a `Cargo.toml` update

Rust already provides a stable project-root integration test discovery point via
the `tests/` directory. We can use that as a small, durable harness while still
storing the real tests under `src/features/<feature>/tests/`.

## Decision

Keep slice test files under `src/features/<feature>/tests/`, but expose them to
Cargo through one or more stable root integration harness files in `tests/`.

Example:

```rust
// tests/feature_slices.rs
#[path = "../src/features/convert_bpmn_to_mdx/tests/integration.rs"]
mod convert_bpmn_to_mdx;

#[path = "../src/features/graph_validation/tests/integration.rs"]
mod graph_validation;
```

Rules:

- slice-owned tests live under `src/features/<feature>/tests/`
- slice-owned fixtures live under `src/features/<feature>/tests/assets/`
- root `tests/*.rs` files act only as stable harness entrypoints for Cargo
- do not add per-slice `[[test]]` entries to `Cargo.toml` just to discover
  slice tests
- do not wire slice integration tests into library modules with
  `#[cfg(test)] mod tests;`

## Consequences

**Positive:**
- Slice tests remain co-located with their owning feature
- `cargo test --lib` stays limited to actual library tests
- Adding new tests inside an existing slice test file does not require
  `Cargo.toml` changes
- Cargo still discovers the tests through the conventional root `tests/`
  mechanism

**Negative:**
- A small root harness file still exists, even though the real tests live in the
  slice
- Adding a brand new slice test root still requires touching the harness file
- Included test modules primarily exercise public slice seams rather than
  private internals

## Related

- ADR-8: Organize tests as slice-local fractals
- ADR-9: Use explicit slice integration test targets
- Supersedes ADR-9 for test discovery and registration
