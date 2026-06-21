## ADR Compliance Checklist

Before any edit, verify against ALL active ADRs. Proposed ADRs are informational only.

| ADR | Status | Key Mandate |
|-----|--------|-------------|
| **ADR-1** | accepted | Shared IR (BPMN types) for bidirectional compile/import; round-trip integrity |
| **ADR-2** | accepted | Dependencies must work in WASM targets; prefer serde, quick-xml, clap |
| **ADR-3** | accepted | Handcrafted BPMN types (not XSD codegen); types in feature slices under `src/features/` |
| **ADR-4** | accepted | Use `folder.rs` by default; feature roots use `src/features/<feature>/mod.rs` so each slice stays self-contained |
| **ADR-5** | proposed | Clean Architecture: Domain/use-cases/adapters/infrastructure live inside feature slices under `src/features/` |
| **ADR-6** | accepted | Slice tests stay under `src/features/<feature>/tests/` and are discovered through stable root `tests/` harness files |
| **ADR-7** | accepted | Every file/directory name screams its single concern; avoid generic names like `utils/`, `common/`, `steps.rs` |
| **ADR-8** | accepted | BDD test layout: `world.rs`, `slice.feature`, `scenarios/`, `steps/{given,when,then,and}.rs` |
| **ADR-9** | accepted | Vertical slices under `src/features/<feature>/`; no cross-slice adapter imports; slice owns tests and fixtures |

## Architecture

- `src/` contains only the CLI entry point (`main.rs`), crate root (`lib.rs`), `features/`, and runtime assets under `src/assets/` (BPMN XSD schemas for optional `xsd-validation`; not codegen source of truth per ADR-3).
- Feature-layer code lives under `src/features/<feature>/` as vertical slices (ADR-9), each using up to four layer folders/files: domain, use_cases, adapters, infrastructure. Compatibility shims under `src/` must not become the primary implementation home.

## Coding Mandates

- Always run `cargo test` at the end of a complete logical code change, and make sure all tests pass.

- Slice-level integration tests live under `src/features/<feature>/tests/` and
  are discovered through stable root harness files in `tests/`.

- Do not wire slice tests into library code with `#[cfg(test)] mod tests;`
  unless a test must exercise private internals and there is no better seam.

- Never write production code without writing tests in TDD fashion.

### TDD

- Follow TDD principles when writing code, meaning:
    1. Write a failing test that defines a function or improvements of a function
    2. Write the minimum amount of code to make the test pass
    3. Refactor the code while keeping the tests passing
