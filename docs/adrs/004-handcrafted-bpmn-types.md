# ADR-004: Handcrafted BPMN Types Instead of XSD Codegen

## Status
Accepted (supersedes ADR-001 approach for type generation)

## Date
2026-01-20

## Context

ADR-001 proposed using `xsd-parser` to generate Rust types from BPMN 2.0 XSDs.
After implementation, this approach failed due to:

1. **Schema complexity**: BPMN 2.0 XSD includes 5+ interconnected schemas 
   (BPMN20, BPMNDI, DI, DC, Semantic) with complex inheritance
2. **Code generation errors**: Generated code had duplicate type definitions,
   unresolved imports, and type conflicts
3. **Abstract type handling**: Many BPMN types are abstract with substitution
   groups that don't map cleanly to Rust

## Decision

**Handcraft minimal Rust types for the BPMN 2.0 subset defined in spec.md.**

The types will:
- Use `serde` derives for both XML and YAML serialization
- Use `quick-xml` for XML parsing
- Cover only the elements required by the MVP spec

## Supported BPMN Elements (per spec.md)

| Element | Priority |
|---------|----------|
| definitions | Must Have |
| process | Must Have |
| startEvent | Must Have |
| endEvent | Must Have |
| serviceTask | Must Have |
| scriptTask | Must Have |
| exclusiveGateway | Must Have |
| parallelGateway | Must Have |
| sequenceFlow | Must Have |
| documentation | Should Have |
| extensionElements | Could Have |

## Validation Strategy

Since types are handcrafted, we validate against XSD at test-time:

1. **Unit tests**: Parse known-good BPMN files into Rust types
2. **Round-trip tests**: Serialize back to XML, validate with `xmllint --schema`
3. **Integration tests**: Compare generated BPMN with reference files

## Consequences

### Positive
- Immediate compilation, no build-time codegen complexity
- Types tailored exactly to our needs
- Full control over serde attributes for YAML frontmatter format
- WASM-compatible (no build-script dependencies at runtime)
- Simpler dependency tree

### Negative
- Manual maintenance if XSD changes (rare for BPMN 2.0, stable since 2011)
- Risk of drift from XSD (mitigated by validation tests)
- Need to add types manually as we expand BPMN coverage

## Implementation

Create `src/bpmn/types.rs` with:
- `Definitions` (root element)
- `Process`
- `FlowElement` enum (StartEvent, EndEvent, Task variants, Gateway variants)
- `SequenceFlow`
- Common attributes (id, name, incoming, outgoing)

## Alternative Approaches Investigated

### XSD → Protobuf → Rust (2026-01-20)

After the xsd-parser failure, we investigated using Protobuf as an intermediate:
1. Convert XSD to .proto files
2. Use `prost-build` to generate Rust types from .proto

**Research findings:**
- GitHub search for "xsd to protobuf" returned only 2 results, both doing the
  *reverse* direction (proto→xsd), not xsd→proto
- Tools like `xsdata` (Python) can generate code from XSD but don't output .proto
- No mature, maintained XSD→Proto converter exists
- Would require manual Proto schema creation anyway

**Conclusion:** XSD→Proto path is not viable. Continue with handcrafted types.

### XSD → JSON Schema (Future Option)

If runtime validation of YAML frontmatter is needed:
- Use `xsdata` (Python) with JSON output format to generate JSON Schema from XSD
- Validate YAML against JSON Schema using a Rust crate like `jsonschema`
- This could be a pre-commit hook or test-time validation, not build-time codegen

## Related ADRs
- ADR-001: XSD as Source of Truth (partially superseded for codegen; XSD remains
  authoritative for validation)
- ADR-003: Rust Dependencies (updated to remove build-time xsd-parser)
