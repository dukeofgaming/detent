## ADR Compliance Checklist

Before any edit, verify against ALL active ADRs. Proposed/superseded ADRs are informational only.

| ADR | Status | Key Mandate |
|-----|--------|-------------|
| **ADR-1** | superseded (by ADR-4) | XSD as source of truth — no longer active |
| **ADR-2** | accepted | Shared IR (BPMN types) for bidirectional compile/import; round-trip integrity |
| **ADR-3** | accepted | Dependencies must work in WASM targets; prefer serde, quick-xml, clap |
| **ADR-4** | accepted | Handcrafted BPMN types (not XSD codegen); types in feature slices under `src/features/` |
| **ADR-5** | accepted | One type per file; file names match type names (`start_event.rs` → `StartEvent`) |
| **ADR-6** | accepted | Use `folder.rs` instead of `folder/mod.rs` for module definitions (exception: `src/` itself uses `mod.rs` because no direct `.rs` files live under `src/`) |
| **ADR-7** | proposed | Clean Architecture: Domain/use-cases/adapters/infrastructure live inside feature slices under `src/features/` |
| **ADR-8** | accepted | Tests co-located in `{module}/tests.rs` with `#[cfg(test)]`; no test harness files in `tests/` |

## Architecture

- `src/` contains only the CLI entry point (`main.rs`) and module declarations (`lib.rs`). No `.rs` files directly under `src/` except `main.rs` and `lib.rs` — all implementation lives in sub-modules (`src/transpiler/`, `src/graph_validation/`) or `lib/` workspace crates. Module declarations under `src/` use `mod.rs` (`src/transpiler/mod.rs`, `src/graph_validation/mod.rs`).
- Feature-layer code lives under `src/features/<feature>/` using up to four layer folders/files: domain, use_cases, adapters, infrastructure. Compatibility shims under `src/` must not become the primary implementation home.

## Coding Mandates

- Always run `cargo test` at the end of a complete logical code change, and make sure.

- Integration tests go in `tests/` at the project root, not in `src/`.

- Never write production code without writing tests in TDD fashion.

### TDD

- Follow TDD principles when writing code, meaning:
    1. Write a failing test that defines a function or improvements of a function
    2. Write the minimum amount of code to make the test pass
    3. Refactor the code while keeping the tests passing
