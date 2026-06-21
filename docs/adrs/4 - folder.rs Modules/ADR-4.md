---
type: adr
title: ADR-4 - folder.rs Modules
date: 2026-01-21
status: accepted
supersedes:
---

## Context

Rust offers two ways to define a module with submodules:

1. **Legacy style**: `folder/mod.rs` (pre-2018 convention)
2. **Modern style**: `folder.rs` alongside `folder/` directory (Rust 2018+)

Our codebase previously used legacy `mod.rs` module files under flat `src/bpmn/`
and `src/mdx/` trees. The project now uses feature slices under
`src/features/<feature>/`.

## Decision

**Adopt the modern `folder.rs` style by default for module definitions.**

Feature roots under `src/features/<feature>/` are an explicit exception: each
feature root uses `mod.rs` so the feature slice remains physically
self-contained in one directory.

Example transformation (historical flat layout → modern style):

```
# Before (legacy flat layout)
src/bpmn/mod.rs
src/bpmn/types/events/start_event.rs

# After (modern, inside a feature slice)
src/features/convert_bpmn_to_mdx/adapters/bpmn/types/events/start_event.rs
src/features/convert_bpmn_to_mdx/use_cases.rs
src/features/convert_bpmn_to_mdx/infrastructure.rs
```

Exceptions: Module declarations directly under `src/` use `mod.rs` rather than `folder.rs`, because `src/` must contain no direct `.rs` files except `main.rs` and `lib.rs`. Feature roots use `src/features/<feature>/mod.rs` rather than `src/features/<feature>.rs` — this keeps each vertical slice self-contained as a single directory that can own implementation, tests, and assets together. Deeper modules inside a feature still follow the `folder.rs` convention (for example, `src/features/convert_bpmn_to_mdx/use_cases.rs`).

### Options

1. **Keep legacy `mod.rs` everywhere**: Familiar but ambiguous in editors
2. **Adopt `folder.rs` throughout**: Modern Rust idiom — chosen
3. **Mixed: `folder.rs` in library, `mod.rs` in features**: Unclear standard

### Rationale

**1. Official Rust Recommendation.** The Rust Reference states:
> "The `mod.rs` convention still works but is no longer the recommended style."
> — [The Rust Reference: Modules](https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames)

**2. Better Editor Experience.** Multiple `mod.rs` tabs are indistinguishable. With the modern style:
- `bpmn.rs`, `types.rs`, `events.rs` are immediately identifiable in tabs
- Fuzzy finding (`Ctrl+P`) produces distinct results

**3. Rust 2018 Edition Feature.** This style was specifically introduced in Rust 2018 to address the `mod.rs` ergonomic issues:
> "In Rust 2018, you can put `foo.rs` directly in the root [...] without needing `foo/mod.rs`."
> — [Rust Edition Guide: Path Clarity](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html)

**4. Community Adoption.** Major projects have migrated:
- The Rust compiler (`rustc`) uses modern style
- `cargo` itself uses modern style in newer modules
- Popular crates like `tokio`, `serde` use it for new code

References:
1. [The Rust Reference: Module Source Filenames](https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames)
2. [Rust Edition Guide: Path Clarity](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html)
3. [RFC 2126: Path Clarity](https://rust-lang.github.io/rfcs/2126-path-clarity.html)

## Consequences

The impact assessment identified renames from legacy `mod.rs` barrels to
`folder.rs` layer modules inside feature slices:

| Current pattern | Example |
|-----------------|---------|
| Layer barrel | `src/features/convert_bpmn_to_mdx/use_cases.rs` |
| Feature root exception | `src/features/convert_bpmn_to_mdx/mod.rs` |
| Type file | `.../adapters/bpmn/types/events/start_event.rs` |
| Test wiring exception | `.../tests/integration/mod.rs`, `.../tests/bdd/steps/mod.rs` |

Risk is low: pure file renames with no code changes required. All imports and public API remain identical. `cargo test` validates correctness after refactoring. There are no breaking changes — external crate consumers see no difference and internal `use` statements are unchanged.

### Positive

1. Tab names in editors are now unique and meaningful
2. Aligns with modern Rust idioms (2018+)
3. Easier navigation in file explorers and fuzzy finders

### Negative

1. Git history shows file moves (use `git log --follow` to trace)
2. Contributors familiar only with legacy style may need brief onboarding
