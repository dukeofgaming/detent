## Coding Mandates

- Always run `cargo test` at the end of a complete logical code change, and make sure.

- All tests go in `src/tests/`, not in `src/`

- Never write production code without writing tests in TDD fashion.

## Architecture Conventions

- **Adapter→domain conversion functions** (e.g., `to_domain_process`) must live in the **adapter layer** (`src/compiler/<standard>/`), not in the domain layer (`lib/core/`) or its re-export shim (`src/graph_validation/`). The domain layer must not depend on adapter types.
- Domain-layer modules under `src/graph_validation/` are thin re-export shims only — no adapter-importing logic.

### TDD

- Follow TDD principles when writing code, meaning:
    1. Write a failing test that defines a function or improvements of a function
    2. Write the minimum amount of code to make the test pass
    3. Refactor the code while keeping the tests passing