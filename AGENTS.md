## ADR Compliance

**Every edit must follow every ADR with `accepted` status.** ADRs marked `proposed` or `superseded` are informational only and not binding.

| ADR | Status | Binding Mandate |
|-----|--------|-----------------|
| **ADR-1** | accepted | Shared IR (BPMN types) for bidirectional compile/import; round-trip integrity |
| **ADR-2** | accepted | Dependencies must work in WASM targets; prefer serde, quick-xml, clap |
| **ADR-3** | accepted | Handcrafted BPMN types (not XSD codegen); types in feature slices under `src/features/` |
| **ADR-4** | accepted | Use `folder.rs` by default; feature roots use `src/features/<feature>/mod.rs` so each slice stays self-contained |
| **ADR-5** | proposed | Clean Architecture: Domain/use-cases/adapters/infrastructure live inside feature slices under `src/features/` — not binding |
| **ADR-6** | accepted | Tests live under `src/features/<feature>/tests/`; discovered through stable root `tests/` harness (`tests/feature_slices.rs`); zero `#[path]` inside slices; no `build.rs` generation |
| **ADR-7** | accepted | Every file/directory name screams its single concern |
| **ADR-8** | accepted | BDD per test level: `tests/world.rs`, optional `{level}/steps/` (only when level has shared steps), `{level}/scenarios/`, cross-level steps at slice-root `tests/steps/` |
| **ADR-9** | accepted | Feature code organized as vertical slices under `src/features/<feature>/`; slices own code, tests, and fixtures; no cross-slice adapter imports |
| **ADR-10** | accepted | Step bodies stay within a single AAA zone: Given arranges only, When acts only, Then asserts only |

## Architecture

- `src/` contains only the CLI entry point (`main.rs`), crate root (`lib.rs`), `features/`, and runtime assets under `src/assets/` (BPMN XSD schemas for optional `xsd-validation`; not codegen source of truth per [[ADR-3]]).
- Feature-layer code lives under `src/features/<feature>/` as vertical slices ([[ADR-9]]), each using up to four layer folders/files: domain, use_cases, adapters, infrastructure. Compatibility shims under `src/` must not become the primary implementation home.

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

## Completion Workflow

- After every complete logical code change, run `cargo test` and ensure all tests pass before considering the change done.
- Never commit automatically. The agent must not invoke `git commit` or the `/commit` skill without explicit developer confirmation.
- When all tests pass, present a commit proposal to the developer:
  - **Summary** (one line): A short, direct line suitable as a commit title.
  - **Details**: A bulleted breakdown of what changed and why.
  - The files or scope included.
  - The test command and result.
  - Ask whether the developer wants to commit the current state.
- Only proceed to commit after the developer explicitly confirms.
