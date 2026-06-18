## ADR Compliance

**Every edit must follow every ADR with `accepted` status.** ADRs marked `proposed` or `superseded` are informational only and not binding.

| ADR | Status | Binding Mandate |
|-----|--------|-----------------|
| **ADR-1** | superseded (by ADR-4) | — |
| **ADR-2** | accepted | Shared IR (BPMN types) for bidirectional compile/import; round-trip integrity |
| **ADR-3** | accepted | Dependencies must work in WASM targets; prefer serde, quick-xml, clap |
| **ADR-4** | accepted | Handcrafted BPMN types (not XSD codegen); types in feature slices under `src/features/` |
| **ADR-5** | superseded (by ADR-11) | — |
| **ADR-6** | accepted | Use `folder.rs` by default; feature roots use `src/features/<feature>/mod.rs` so each slice stays self-contained |
| **ADR-7** | proposed | Clean Architecture: Domain/use-cases/adapters/infrastructure live inside feature slices under `src/features/` — not binding |
| **ADR-8** | superseded (by ADR-9) | — |
| **ADR-9** | superseded (by ADR-10) | — |
| **ADR-10** | accepted | Tests live under `src/features/<feature>/tests/` and are discovered through stable root `tests/` harness files. No `#[cfg(test)] mod tests` in source files unless testing private internals with no better seam. |
| **ADR-11** | accepted | Every file/directory name screams its single concern; avoid generic names like `utils/`, `common/`, `steps.rs` |
| **ADR-12** | accepted | BDD test layout: `world.rs`, `slice.feature`, `scenarios/`, `steps/{given,when,then,and}.rs` |

## Architecture

- `src/` contains only the CLI entry point (`main.rs`), crate root (`lib.rs`), and `features/`. No other implementation should live directly under `src/`.
- Feature-layer code lives under `src/features/<feature>/` using up to four layer folders/files: domain, use_cases, adapters, infrastructure. Compatibility shims under `src/` must not become the primary implementation home.

## Coding Mandates

- Always run `cargo test` at the end of a complete logical code change, and make sure all tests pass before considering the change done.
- Slice-level integration tests live under `src/features/<feature>/tests/` and are discovered through stable root harness files in `tests/`.
- Do not wire slice tests into library code with `#[cfg(test)] mod tests;` unless a test must exercise private internals and there is no better seam.
- Never write production code without writing tests in TDD fashion.
