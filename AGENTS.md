## ADR Compliance Checklist

Before any edit, verify against ALL active ADRs. Proposed/superseded ADRs are informational only.

| ADR | Status | Key Mandate |
|-----|--------|-------------|
| **ADR-1** | superseded (by ADR-4) | XSD as source of truth — no longer active |
| **ADR-2** | accepted | Shared IR (BPMN types) for bidirectional compile/import; round-trip integrity |
| **ADR-3** | accepted | Dependencies must work in WASM targets; prefer serde, quick-xml, clap |
| **ADR-4** | accepted | Handcrafted BPMN types (not XSD codegen); types in `lib/core/` or `src/transpiler/` |
| **ADR-5** | accepted | One type per file; file names match type names (`start_event.rs` → `StartEvent`) |
| **ADR-6** | accepted | Use `folder.rs` instead of `folder/mod.rs` for module definitions (exception: `src/` itself uses `mod.rs` because no direct `.rs` files live under `src/`) |
| **ADR-7** | proposed | Clean Architecture: Domain in `lib/core/`, Adapters in `src/transpiler/<standard>/`, re-export shims don't import adapter types |
| **ADR-8** | accepted | Tests co-located in `{module}/tests.rs` with `#[cfg(test)]`; no test harness files in `tests/` |

## Architecture

- `src/` is for the CLI entry point (`main.rs`) and thin module declarations (`lib.rs`). No `.rs` files directly under `src/` except `main.rs` and `lib.rs` — all implementation lives in sub-modules (`src/transpiler/`, `src/graph_validation/`) or `lib/` workspace crates. Module declarations under `src/` use `mod.rs` (`src/transpiler/mod.rs`, `src/graph_validation/mod.rs`).
- Domain-layer types and operations go in `lib/core/` (the `detent-core` workspace crate). Adapter-layer code goes in `src/transpiler/<standard>/`. Re-export shims under `src/` must not import adapter types.

## Documentation

- When making edits, planning or starting a new session, always follow the rules in [[issues/AGENTS]].
