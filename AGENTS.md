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
| **ADR-8** | accepted | Each feature owns `tests/` and `tests/assets/` as a slice-local fractal; no project-root test harness files |

## Architecture

- `src/` contains only the CLI entry point (`main.rs`), crate root (`lib.rs`), and `features/`. No other implementation should live directly under `src/`.
- Feature-layer code lives under `src/features/<feature>/` using up to four layer folders/files: domain, use_cases, adapters, infrastructure. Compatibility shims under `src/` must not become the primary implementation home.

## Documentation

- When making edits, planning or starting a new session, always follow the rules in [[issues/AGENTS]].
