---
type: adr
title: ADR-1 - XSD as Source of Truth for BPMN Types
date: 2026-01-27
status: proposed
supersedes:
---

## Context

We need to validate MDX frontmatter (YAML) that represents BPMN elements. We want to avoid maintaining duplicate schemas and prefer leveraging the official OMG BPMN 2.0 XSDs as the canonical source. Additionally, we need runtime XSD validation for BPMN files to ensure conformance to the BPMN 2.0 specification. This ADR has been superseded by [[ADR-4]].

## Decision

**Use `xsd-parser` crate to generate Rust types from BPMN XSDs.**

**Use `libxml` crate for runtime XSD schema validation** (added 2026-01-27).

The generated types will derive `serde::Serialize` and `serde::Deserialize` for YAML/JSON support, use `quick-xml` for XML serialization/deserialization, and serve as the single source of truth for both XML and YAML representations.

For runtime XSD validation, use `libxml` crate (bindings to libxml2) behind an optional feature flag `xsd-validation` to validate BPMN XML files against the bundled BPMN20.xsd schema. This validation runs *before* semantic/graph checks (reachability, gateway invariants, etc.).

Type generation implementation: add `xsd-parser` as a build dependency, create `build.rs` to generate types from `src/assets/schemas/*.xsd`, output generated code to `src/bpmn/generated.rs`, and add serde derives for dual XML/YAML support.

XSD validation implementation: add `libxml = "0.3"` as an optional dependency behind the `xsd-validation` feature, create `src/bpmn/xsd_validator.rs` with `validate_against_xsd()` function, bundle BPMN XSD files in `src/assets/schemas/`, and update the `validate` command to run XSD validation when the feature is enabled.

### Options

1. **Manual JSON Schema**: Hand-write and maintain JSON schemas for frontmatter. Rejected because it violates the "no manual maintenance" requirement and risks drift between XSD and JSON Schema.
2. **XSD → JSON Schema conversion**: Convert BPMN XSDs to JSON Schema, then validate. Tools like `xsd2json` exist but have lossy conversions. Would still need runtime validation, adding an extra translation layer.
3. **XSD → Rust types generation**: Generate Rust structs from XSD, use serde for (de)serialization. Chosen for compile-time safety, WASM compatibility, and bidirectional type reuse.

Pure-Rust XSD validation approaches were also evaluated: `xmlschema` v0.0.1 is too immature (only parses XSD, no `validate()` API exposed), and `xsd` crate v0.2.2 is a stub library with no real functionality. No pure-Rust XSD validator currently handles complex schemas with imports. When a mature pure-Rust validator emerges, it should be abstracted behind a trait and swapped in.

### Rationale

1. **Single source of truth**: OMG XSDs → Rust types, no manual schema maintenance
2. **Compile-time safety**: Type errors caught at compile time
3. **WASM compatible**: `xsd-parser` and `quick-xml` work in WASM targets
4. **Bidirectional**: Same types work for parsing BPMN XML and validating MDX YAML frontmatter
5. **Performance**: Generated code is efficient, no runtime schema interpretation
6. **Existing library**: `xsd-parser` v1.4.0 is mature with serde + quick-xml support
7. **Mature XSD validation**: libxml2 handles complex multi-file XSD schemas (BPMN20.xsd imports DI.xsd, DC.xsd, BPMNDI.xsd, Semantic.xsd) with detailed error reporting
8. **Feature-gated native dep**: libxml2 dependency is optional behind the `xsd-validation` feature, disabled by default for WASM builds
9. **Authoritative validation**: Uses the official OMG XSDs as the validation source

## Consequences

### Positive

1. Zero schema maintenance burden
2. Type-safe throughout the codebase
3. Same types validate both XML (compiled output) and YAML (frontmatter)
4. Build-time code generation keeps dependencies minimal at runtime
5. Full BPMN 2.0 XSD validation when the `xsd-validation` feature is enabled

### Negative

1. Build-time dependency on `xsd-parser`
2. Generated code may need customization for edge cases
3. Need to regenerate when XSD updates (rare for BPMN 2.0)
4. `xsd-validation` feature requires native libxml2 installation and is not WASM-compatible; libxml2 cannot be easily compiled to WASM
5. Without `xsd-validation`, validation relies only on serde deserialization + semantic checks
6. For WASM targets, only serde deserialization errors and semantic validation are available; when a pure-Rust XSD validator matures, it can be added as a WASM-compatible alternative
