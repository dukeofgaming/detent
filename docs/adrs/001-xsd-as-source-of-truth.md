# ADR-001: XSD as Source of Truth for BPMN Types

## Status
Accepted

## Date
2026-01-20

## Context

We need to validate MDX frontmatter (YAML) that represents BPMN elements. The options are:

1. **Manual JSON Schema**: Hand-write and maintain JSON schemas for frontmatter
2. **XSD → JSON Schema conversion**: Convert BPMN XSDs to JSON Schema, then validate
3. **XSD → Rust types generation**: Generate Rust structs from XSD, use serde for (de)serialization

We want to avoid maintaining duplicate schemas and prefer leveraging the official OMG BPMN 2.0 XSDs as the canonical source.

## Decision

**Use `xsd-parser` crate to generate Rust types from BPMN XSDs.**

The generated types will:
- Derive `serde::Serialize` and `serde::Deserialize` for YAML/JSON support
- Use `quick-xml` for XML serialization/deserialization
- Serve as the single source of truth for both XML and YAML representations

## Rationale

1. **Single source of truth**: OMG XSDs → Rust types (no manual schema maintenance)
2. **Compile-time safety**: Type errors caught at compile time
3. **WASM compatible**: `xsd-parser` and `quick-xml` work in WASM targets
4. **Bidirectional**: Same types work for parsing BPMN XML and validating MDX YAML frontmatter
5. **Performance**: Generated code is efficient, no runtime schema interpretation
6. **Existing library**: `xsd-parser` v1.4.0 is mature with serde + quick-xml support

## Alternatives Considered

### XSD → JSON Schema conversion
- Tools like `xsd2json` exist but have lossy conversions
- Would still need runtime validation
- Extra translation layer

### Manual JSON Schema
- Violates "no manual maintenance" requirement
- Drift risk between XSD and JSON Schema

## Consequences

### Positive
- Zero schema maintenance burden
- Type-safe throughout the codebase
- Same types validate both XML (compiled output) and YAML (frontmatter)
- Build-time code generation keeps dependencies minimal at runtime

### Negative
- Build-time dependency on `xsd-parser`
- Generated code may need customization for edge cases
- Need to regenerate when XSD updates (rare for BPMN 2.0)

## Implementation

1. Add `xsd-parser` as a build dependency
2. Create `build.rs` to generate types from `src/assets/schemas/*.xsd`
3. Output generated code to `src/bpmn/generated.rs`
4. Add serde derives for dual XML/YAML support
