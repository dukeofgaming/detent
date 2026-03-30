---
type: adr
date: 2026-01-20
status: accepted
---
# ADR-2: Bidirectional Compiler Architecture

## Context

The spec defines two transformations:
- **Compile** (MDX → BPMN): Convert flow folder of MDX files to deterministic BPMN XML
- **Import** (BPMN → MDX): Convert BPMN XML to MDX files, preserving existing MDX bodies

We need to decide on the architecture for these transformations.

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

The IR uses the XSD-generated Rust types (ADR-1), ensuring the same type system validates both directions.

## Rationale

1. **Shared types**: XSD-generated types serve as the IR
2. **Round-trip integrity**: Parse and serialize through the same structures
3. **Deterministic output**: Stable ordering in serialization
4. **Future extensibility**: IR supports debugger and language server use cases
5. **Separation of concerns**: Parsing (syntax) vs. compilation (semantics) are distinct

## Architecture

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

## Consequences

### Positive
- Single codebase for both directions
- Consistent validation in both paths
- Natural foundation for LSP/debugger (IR is queryable)
- Round-trip tests are straightforward

### Negative
- Slightly more complex than two separate tools
- IR must preserve information from both sources

## Related ADRs
- ADR-1: XSD as Source of Truth
