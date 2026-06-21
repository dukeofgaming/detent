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

We need to decide on the architecture for these transformations. The shared
type system is handcrafted BPMN adapter types ([[ADR-4]]), including the
rejected XSD-codegen path documented there.

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

The IR uses handcrafted BPMN adapter types ([[ADR-4]]), ensuring the same type
system validates both compile and import directions.

The module layout organizes concerns inside feature slices:

```
src/features/convert_bpmn_to_mdx/
├── adapters/
│   ├── bpmn/          # parse/serialize + handcrafted BPMN types (IR)
│   └── mdx/           # MDX frontmatter types
├── use_cases/
│   ├── compile.rs     # MDX → Definitions
│   └── import.rs      # Definitions → MDX
└── infrastructure/
    ├── cli/           # filesystem orchestration
    └── xsd_validator.rs  # optional libxml XSD check
```

### Options

1. **Unified bidirectional compiler with shared IR**: Single codebase, shared type system
2. **Two separate tools**: Independent compile and import tools with duplicated logic
3. **Add-on approach**: Build compile first, bolt import on later

### Rationale

1. **Shared types**: Handcrafted BPMN adapter types serve as the compile/import IR ([[ADR-4]])
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
