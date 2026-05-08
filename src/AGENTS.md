## Architecture
- `src/` contains only the CLI entry point (`main.rs`) and module declarations (`lib.rs`, `compiler/mod.rs`). No production logic files directly under `src/` — all implementation lives in sub-modules (`src/compiler/`, `src/graph_validation/`) or `lib/` workspace crates.
- Domain-layer types and operations go in `lib/core/` (the `detent-core` workspace crate). Adapter-layer code goes in `src/compiler/<standard>/`. Re-export shims under `src/` must not import adapter types.

## Coding Mandates

- Always run `cargo test` at the end of a complete logical code change, and make sure.

- Integration tests go in `tests/` at the project root, not in `src/`.

- Never write production code without writing tests in TDD fashion.

### TDD

- Follow TDD principles when writing code, meaning:
    1. Write a failing test that defines a function or improvements of a function
    2. Write the minimum amount of code to make the test pass
    3. Refactor the code while keeping the tests passing
