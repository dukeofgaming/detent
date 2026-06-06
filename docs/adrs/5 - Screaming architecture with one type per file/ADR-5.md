---
type: adr
title: ADR-5 - Screaming Architecture with One Type Per File
date: 2026-01-20
status: accepted
supersedes:
---

## Context

Our BPMN types module (`src/bpmn/types/`) initially grouped related types
into single files:

- `events.rs` contained `StartEvent`, `EndEvent`
- `tasks.rs` contained `Task`, `ServiceTask`, `ScriptTask`
- `gateways.rs` contained `ExclusiveGateway`, `ParallelGateway`

While this reduced file count, it created several issues:

1. **Discoverability**: Finding a type requires scanning file contents
2. **Merge conflicts**: Multiple developers modifying related types collide
3. **Cognitive load**: Files grow as types accumulate behaviors
4. **Import ambiguity**: `use types::tasks` doesn't reveal what's inside

This ADR has been superseded by [[ADR-11]].

## Decision

**Adopt "screaming architecture" with one type per file in folder modules.**

The new structure:

```
src/bpmn/types/
├── events/
│   ├── mod.rs          # re-exports StartEvent, EndEvent
│   ├── start_event.rs  # StartEvent only
│   └── end_event.rs    # EndEvent only
├── tasks/
│   ├── mod.rs          # re-exports Task, ServiceTask, ScriptTask
│   ├── task.rs
│   ├── service_task.rs
│   └── script_task.rs
├── gateways/
│   ├── mod.rs          # re-exports ExclusiveGateway, ParallelGateway
│   ├── exclusive_gateway.rs
│   └── parallel_gateway.rs
└── mod.rs              # unchanged public API
```

### Options

1. **Single file per category** (e.g., `events.rs`, `tasks.rs`): Fewer files but harder to discover
2. **One type per file** (screaming architecture): Self-documenting, isolates changes — chosen
3. **Flat file with all types**: Single `types.rs` — does not scale

### Rationale

**1. File names "scream" their contents.** The pattern `exclusive_gateway.rs` contains `ExclusiveGateway` is self-evident. No need to open files to find types.

**2. Stable public API.** Parent `mod.rs` re-exports all types, so external imports remain unchanged:

```rust
use crate::bpmn::types::{StartEvent, EndEvent};
```

**3. Scales with BPMN coverage.** As we add more BPMN elements (boundaryEvent, userTask, subProcess), each gets its own file without bloating existing ones.

**4. Isolated change sets.** Modifying `ScriptTask` only touches `script_task.rs`, reducing merge conflicts and making code review clearer.

**Metaprogramming Complement.** Alongside this structural change, we added a generic parsing method to reduce repetitive boilerplate:

```rust
impl MdxFile {
    pub fn parse_as<T: DeserializeOwned>(&self) -> Result<T, serde_yaml::Error> {
        serde_yaml::from_str(&self.frontmatter)
    }
}
```

This allows callers to write `mdx.parse_as::<StartEvent>()` instead of needing a dedicated `parse_start_event()` method for each type. The type-specific convenience methods remain as thin wrappers for discoverability.

This pattern follows Robert C. Martin's "Screaming Architecture" (Clean Architecture, Chapter 21) and Rust API Guidelines on module organization.

## Consequences

### Positive

1. **Clarity**: Architecture structure mirrors domain concepts
2. **Maintainability**: Changes isolated to single-purpose files
3. **Extensibility**: Adding types requires no modification to existing files
4. **IDE navigation**: "Go to file" directly locates types

### Negative

1. **More files**: ~15 files vs ~6 files initially. Mitigated by IDE file navigation making file count irrelevant.
2. **More imports in mod.rs**: Each folder needs re-exports. Mitigated by mod.rs boilerplate being minimal (2 lines per type).
