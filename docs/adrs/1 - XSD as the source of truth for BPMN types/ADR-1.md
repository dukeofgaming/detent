---
type: adr
date: 2026-01-27
status: proposed
---
# ADR-1: XSD as Source of Truth for BPMN Types

## Context

We need to validate MDX frontmatter (YAML) that represents BPMN elements. The options are:

1. **Manual JSON Schema**: Hand-write and maintain JSON schemas for frontmatter
2. **XSD → JSON Schema conversion**: Convert BPMN XSDs to JSON Schema, then validate
3. **XSD → Rust types generation**: Generate Rust structs from XSD, use serde for (de)serialization

We want to avoid maintaining duplicate schemas and prefer leveraging the official OMG BPMN 2.0 XSDs as the canonical source.

Additionally, we need runtime XSD validation for BPMN files to ensure conformance to the BPMN 2.0 specification.

## Decision

**Use `xsd-parser` crate to generate Rust types from BPMN XSDs** (unchanged).

**Use `libxml` crate for runtime XSD schema validation** (added 2026-01-27).

The generated types will:
- Derive `serde::Serialize` and `serde::Deserialize` for YAML/JSON support
- Use `quick-xml` for XML serialization/deserialization
- Serve as the single source of truth for both XML and YAML representations

For runtime XSD validation:
- Use `libxml` crate (bindings to libxml2) behind an optional feature flag `xsd-validation`
- Validate BPMN XML files against the bundled BPMN20.xsd schema
- This validation runs *before* semantic/graph checks (reachability, gateway invariants, etc.)

## Rationale

### Type Generation (unchanged)
1. **Single source of truth**: OMG XSDs → Rust types (no manual schema maintenance)
2. **Compile-time safety**: Type errors caught at compile time
3. **WASM compatible**: `xsd-parser` and `quick-xml` work in WASM targets
4. **Bidirectional**: Same types work for parsing BPMN XML and validating MDX YAML frontmatter
5. **Performance**: Generated code is efficient, no runtime schema interpretation
6. **Existing library**: `xsd-parser` v1.4.0 is mature with serde + quick-xml support

### Runtime XSD Validation (added 2026-01-27)
1. **Mature and battle-tested**: libxml2 handles complex multi-file XSD schemas (BPMN20.xsd imports DI.xsd, DC.xsd, BPMNDI.xsd, Semantic.xsd)
2. **Good error reporting**: Detailed validation error messages
3. **Feature-gated**: Native dependency is optional; disabled by default for WASM builds
4. **Authoritative validation**: Uses the official OMG XSDs as the validation source

## Alternatives Considered

### XSD → JSON Schema conversion
- Tools like `xsd2json` exist but have lossy conversions
- Would still need runtime validation
- Extra translation layer

### Manual JSON Schema
- Violates "no manual maintenance" requirement
- Drift risk between XSD and JSON Schema

### Pure-Rust XSD validation (`xmlschema` crate)
- Evaluated `xmlschema` v0.0.1 (2026-01-27): too immature, only parses XSD, no `validate()` API exposed
- Evaluated `xsd` crate v0.2.2: stub library with no real functionality
- No pure-Rust XSD validator currently handles complex schemas with imports
- **Future consideration**: When a mature pure-Rust validator emerges, abstract behind a trait and swap implementations

## Consequences

### Positive
- Zero schema maintenance burden
- Type-safe throughout the codebase
- Same types validate both XML (compiled output) and YAML (frontmatter)
- Build-time code generation keeps dependencies minimal at runtime
- Full BPMN 2.0 XSD validation when the `xsd-validation` feature is enabled

### Negative
- Build-time dependency on `xsd-parser`
- Generated code may need customization for edge cases
- Need to regenerate when XSD updates (rare for BPMN 2.0)
- `xsd-validation` feature requires native libxml2 installation (not WASM-compatible)
- Without `xsd-validation`, validation relies only on serde deserialization + semantic checks

## WASM Compatibility

The `xsd-validation` feature is **disabled by default** and should remain disabled for WASM builds:
- libxml2 cannot be easily compiled to WASM
- For WASM targets, rely on serde deserialization errors and semantic validation
- When a pure-Rust XSD validator matures, it can be added as a WASM-compatible alternative

## Implementation

### Type Generation
1. Add `xsd-parser` as a build dependency
2. Create `build.rs` to generate types from `src/assets/schemas/*.xsd`
3. Output generated code to `src/bpmn/generated.rs`
4. Add serde derives for dual XML/YAML support

### XSD Validation
1. Add `libxml = "0.3"` as an optional dependency behind `xsd-validation` feature
2. Create `src/bpmn/xsd_validator.rs` module with `validate_against_xsd()` function
3. Bundle BPMN XSD files in `src/assets/schemas/` (already present)
4. Update `validate` command to run XSD validation when feature is enabled
