## ADR Compliance

**Every edit must follow every ADR with `accepted` status.** ADRs marked `proposed` or `superseded` are informational only and not binding.

Retired numbers **1, 5, 8, 9** — content consolidated into **ADR-4**, **ADR-11**, and **ADR-10** respectively (files removed; see those ADRs' Context for how we arrived here).

| ADR | Status | Binding Mandate |
|-----|--------|-----------------|
| **ADR-2** | accepted | Shared IR (BPMN types) for bidirectional compile/import; round-trip integrity |
| **ADR-3** | accepted | Dependencies must work in WASM targets; prefer serde, quick-xml, clap |
| **ADR-4** | accepted | Handcrafted BPMN types (not XSD codegen); types in feature slices under `src/features/`; supersedes retired ADR-1 |
| **ADR-6** | accepted | Use `folder.rs` by default; feature roots use `src/features/<feature>/mod.rs` so each slice stays self-contained |
| **ADR-7** | proposed | Clean Architecture: Domain/use-cases/adapters/infrastructure live inside feature slices under `src/features/` — not binding |
| **ADR-10** | accepted | Tests live under `src/features/<feature>/tests/` and are discovered through stable root `tests/` harness files; supersedes retired ADR-8/9 |
| **ADR-11** | accepted | Every file/directory name screams its single concern; supersedes retired ADR-5 |
| **ADR-12** | accepted | BDD test layout: `world.rs`, `slice.feature`, `scenarios/`, `steps/{given,when,then,and}.rs` |

## Architecture

- `src/` contains only the CLI entry point (`main.rs`), crate root (`lib.rs`), `features/`, and runtime assets under `src/assets/` (BPMN XSD schemas for optional `xsd-validation`; not codegen source of truth per [[ADR-4]]).
- Feature-layer code lives under `src/features/<feature>/` using up to four layer folders/files: domain, use_cases, adapters, infrastructure. Compatibility shims under `src/` must not become the primary implementation home.

## Coding Mandates

- Agentic coding work must stay inside this repository workspace and the project devcontainer.
- Do not access the host filesystem outside the workspace root with any tool. This includes host temp directories such as `/var`, `/tmp`, `/private/tmp`, user home paths outside this repo, global tool caches, or copied worktree/devcontainer folders.
- Execute shell commands only inside the project devcontainer. Never run `bash`, `cargo`, `git`, package managers, scripts, or other command workflows on the host system.
- If no devcontainer is active or command execution inside the devcontainer fails because the container is missing/stopped, use the devcontainer tool to start or create the project devcontainer before running commands.
- If the devcontainer cannot be started or attached, stop and report the blocker. Do not fall back to host commands or host filesystem workarounds.
- Always run `cargo test` at the end of a complete logical code change, and make sure all tests pass before considering the change done.
- Slice-level integration tests live under `src/features/<feature>/tests/` and are discovered through stable root harness files in `tests/`.
- Do not wire slice tests into library code with `#[cfg(test)] mod tests;` unless a test must exercise private internals and there is no better seam.
- Never write production code without writing tests in TDD fashion.
