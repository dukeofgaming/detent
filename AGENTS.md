## ADR Compliance Checklist

Before any edit, verify against ALL active ADRs. Proposed/superseded ADRs are informational only.

| ADR | Status | Key Mandate |
|-----|--------|-------------|
| **ADR-1** | superseded (by ADR-4) | XSD as source of truth — no longer active |
| **ADR-2** | accepted | Shared IR (BPMN types) for bidirectional compile/import; round-trip integrity |
| **ADR-3** | accepted | Dependencies must work in WASM targets; prefer serde, quick-xml, clap |
| **ADR-4** | accepted | Handcrafted BPMN types (not XSD codegen); types in feature slices under `src/features/` |
| **ADR-5** | accepted | One type per file; file names match type names (`start_event.rs` → `StartEvent`) |
| **ADR-6** | accepted | Use `folder.rs` by default; feature roots use `src/features/<feature>/mod.rs` so each slice stays self-contained |
| **ADR-7** | proposed | Clean Architecture: Domain/use-cases/adapters/infrastructure live inside feature slices under `src/features/` |
| **ADR-8** | superseded (by ADR-9) | Slice-local test ownership retained, but compilation model replaced |
| **ADR-9** | superseded (by ADR-10) | Explicit Cargo test targets solved lib coupling, but required TOML updates |
| **ADR-10** | accepted | Slice tests stay under `src/features/<feature>/tests/` and are discovered through stable root `tests/` harness files, not lib modules or per-slice TOML entries |

## Architecture

- `src/` contains only the CLI entry point (`main.rs`), crate root (`lib.rs`), and `features/`. No other implementation should live directly under `src/`.
- Feature-layer code lives under `src/features/<feature>/` using up to four layer folders/files: domain, use_cases, adapters, infrastructure. Compatibility shims under `src/` must not become the primary implementation home.

## Documentation

- When making edits, planning or starting a new session, always follow the rules in [[issues/AGENTS]].
