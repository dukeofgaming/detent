## Architecture
- When making edits, always make sure you are not going against the architecture of the project, to ensure so, read `docs/adrs`
- If the user wants to do something that goes against the architecture, remind them of which ADRs it goes against and why.
- `src/` is for the CLI entry point (`main.rs`) and thin module declarations (`lib.rs`, `compiler/mod.rs`). No production logic files directly under `src/` — all implementation lives in sub-modules (`src/compiler/`, `src/graph_validation/`) or `lib/` workspace crates.
- Domain-layer types and operations go in `lib/core/` (the `detent-core` workspace crate). Adapter-layer code goes in `src/compiler/<standard>/`. Re-export shims under `src/` must not import adapter types.

## Documentation

- When making edits, planning or starting a new session, always follow the rules in [[issues/AGENTS]].
