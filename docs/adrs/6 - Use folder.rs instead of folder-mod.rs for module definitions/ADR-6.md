---
type: adr
date: 2026-01-21
status: accepted
---
# ADR-6: Use `folder.rs` Instead of `folder/mod.rs` for Module Definitions

## Context

Rust offers two ways to define a module with submodules:

1. **Legacy style**: `folder/mod.rs` (pre-2018 convention)
2. **Modern style**: `folder.rs` alongside `folder/` directory (Rust 2018+)

Our codebase currently uses `mod.rs` files throughout:
- `src/bpmn/mod.rs`
- `src/bpmn/types/mod.rs`
- `src/bpmn/types/events/mod.rs`
- `src/mdx/mod.rs`
- `src/commands/mod.rs`

## Decision

**Adopt the modern `folder.rs` style for all module definitions.**

Example transformation:
```
# Before (current)
src/bpmn/mod.rs
src/bpmn/types/mod.rs
src/bpmn/types/events/mod.rs

# After (modern)
src/bpmn.rs
src/bpmn/types.rs
src/bpmn/types/events.rs
```

## Rationale

### 1. Official Rust Recommendation
The Rust Reference states:
> "The `mod.rs` convention still works but is no longer the recommended style."
> — [The Rust Reference: Modules](https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames)

### 2. Better Editor Experience
Multiple `mod.rs` tabs are indistinguishable. With the modern style:
- `bpmn.rs`, `types.rs`, `events.rs` are immediately identifiable in tabs
- Fuzzy finding (`Ctrl+P`) produces distinct results

### 3. Rust 2018 Edition Feature
This style was specifically introduced in Rust 2018 to address the `mod.rs` ergonomic issues:
> "In Rust 2018, you can put `foo.rs` directly in the root [...] without needing `foo/mod.rs`."
> — [Rust Edition Guide: Path Clarity](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html)

### 4. Community Adoption
Major projects have migrated:
- The Rust compiler (`rustc`) uses modern style
- `cargo` itself uses modern style in newer modules
- Popular crates like `tokio`, `serde` use it for new code

## Impact Assessment

### Files to Rename (9 total)
| Current Path | New Path |
|--------------|----------|
| `src/bpmn/mod.rs` | `src/bpmn.rs` |
| `src/bpmn/types/mod.rs` | `src/bpmn/types.rs` |
| `src/bpmn/types/events/mod.rs` | `src/bpmn/types/events.rs` |
| `src/bpmn/types/gateways/mod.rs` | `src/bpmn/types/gateways.rs` |
| `src/bpmn/types/tasks/mod.rs` | `src/bpmn/types/tasks.rs` |
| `src/mdx/mod.rs` | `src/mdx.rs` |
| `src/mdx/types/mod.rs` | `src/mdx/types.rs` |
| `src/commands/mod.rs` | `src/commands.rs` |

### Risk: Low
- Pure file renames with no code changes required
- All imports and public API remain identical
- `cargo test` validates correctness after refactoring

### Breaking Changes: None
- External crate consumers see no difference
- Internal `use` statements unchanged

## Consequences

### Positive
- Tab names in editors are now unique and meaningful
- Aligns with modern Rust idioms (2018+)
- Easier navigation in file explorers and fuzzy finders

### Negative
- Git history shows file moves (use `git log --follow` to trace)
- Contributors familiar only with legacy style may need brief onboarding

## Exception: `src/` Root Modules

Module declarations directly under `src/` use `mod.rs` rather than `folder.rs`.
This is because `src/` must contain no direct `.rs` files except `main.rs` and
`lib.rs`. Feature implementation lives under `src/features/`, and deeper
modules there follow the `folder.rs` convention (for example,
`src/features/convert_bpmn_to_mdx/use_cases.rs`).

## References

1. [The Rust Reference: Module Source Filenames](https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames)
2. [Rust Edition Guide: Path Clarity](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html)
3. [RFC 2126: Path Clarity](https://rust-lang.github.io/rfcs/2126-path-clarity.html)
