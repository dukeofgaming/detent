---
type: adr
title: ADR-2 - Bidirectional Compiler Architecture
date: 2026-01-20
status: accepted
supersedes:
---

## Context

The spec defines two transformations:
- **Compile** (MDX → BPMN): Convert flow folder of MDX files to deterministic BPMN XML
- **Import** (BPMN → MDX): Convert BPMN XML to MDX files, preserving existing MDX bodies

We need to decide on the architecture for these transformations. The architecture should leverage the XSD-generated Rust types from [[ADR-1]] as the shared type system.

## Decision

**Implement a unified bidirectional compiler with shared intermediate representation (IR).**

```
    ┌─────────────┐
    │  MDX Files  │
    └──────┬──────┘
           │ parse
           ▼
    ┌─────────────┐
    │  BPMN Graph │◄──── IR (in-memory)
    │    (IR)     │
    └──────┬──────┘
           │ serialize
           ▼
    ┌─────────────┐
    │  BPMN XML   │
    └─────────────┘
```

The IR uses the XSD-generated Rust types ([[ADR-1]]), ensuring the same type system validates both directions.

The module layout organizes concerns as follows:
```
src/
├── bpmn/
│   ├── generated.rs    # XSD-generated types
│   ├── graph.rs        # Graph operations on BPMN model
│   └── mod.rs
├── mdx/
│   ├── parser.rs       # Frontmatter extraction
│   ├── writer.rs       # MDX generation with body preservation
│   └── mod.rs
├── compiler/
│   ├── compile.rs      # MDX → IR → BPMN
│   ├── import.rs       # BPMN → IR → MDX
│   └── mod.rs
└── cli/
    └── mod.rs          # CLI commands
```

### Options

1. **Unified bidirectional compiler with shared IR**: Single codebase, shared type system
2. **Two separate tools**: Independent compile and import tools with duplicated logic
3. **Add-on approach**: Build compile first, bolt import on later

### Rationale

1. **Shared types**: XSD-generated types serve as the IR
2. **Round-trip integrity**: Parse and serialize through the same structures
3. **Deterministic output**: Stable ordering in serialization
4. **Future extensibility**: IR supports debugger and language server use cases
5. **Separation of concerns**: Parsing (syntax) vs. compilation (semantics) are distinct

## Consequences

### Positive

1. Single codebase for both directions
2. Consistent validation in both paths
3. Natural foundation for LSP/debugger (IR is queryable)
4. Round-trip tests are straightforward

### Negative

1. Slightly more complex than two separate tools
2. IR must preserve information from both sources
