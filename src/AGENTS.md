## ADR Compliance Checklist

Before any edit, verify against ALL active ADRs. Proposed ADRs are informational only.

Retired numbers **1, 5, 8, 9** — consolidated into **ADR-4**, **ADR-11**, **ADR-10** (see their Context sections).

| ADR | Status | Key Mandate |
|-----|--------|-------------|
| **ADR-2** | accepted | Shared IR (BPMN types) for bidirectional compile/import; round-trip integrity |
| **ADR-3** | accepted | Dependencies must work in WASM targets; prefer serde, quick-xml, clap |
| **ADR-4** | accepted | Handcrafted BPMN types (not XSD codegen); types in feature slices under `src/features/` |
| **ADR-6** | accepted | Use `folder.rs` by default; feature roots use `src/features/<feature>/mod.rs` so each slice stays self-contained |
| **ADR-7** | proposed | Clean Architecture: Domain/use-cases/adapters/infrastructure live inside feature slices under `src/features/` |
| **ADR-10** | accepted | Slice tests stay under `src/features/<feature>/tests/` and are discovered through stable root `tests/` harness files |
| **ADR-11** | accepted | Every file/directory name screams its single concern; avoid generic names like `utils/`, `common/`, `steps.rs` |
| **ADR-12** | accepted | BDD test layout: `world.rs`, `slice.feature`, `scenarios/`, `steps/{given,when,then,and}.rs` |

## Architecture

- `src/` contains only the CLI entry point (`main.rs`), crate root (`lib.rs`), `features/`, and runtime assets under `src/assets/` (BPMN XSD schemas for optional `xsd-validation`; not codegen source of truth per ADR-4).
- Feature-layer code lives under `src/features/<feature>/` using up to four layer folders/files: domain, use_cases, adapters, infrastructure. Compatibility shims under `src/` must not become the primary implementation home.

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
