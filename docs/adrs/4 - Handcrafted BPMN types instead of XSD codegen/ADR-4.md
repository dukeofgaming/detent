---
type: adr
title: ADR-4 - Handcrafted BPMN Types Instead of XSD Codegen
date: 2026-01-20
status: accepted
supersedes: 1
---

## Context

[[ADR-1]] proposed using `xsd-parser` to generate Rust types from BPMN 2.0 XSDs. After implementation, this approach failed due to:

1. **Schema complexity**: BPMN 2.0 XSD includes 5+ interconnected schemas (BPMN20, BPMNDI, DI, DC, Semantic) with complex inheritance
2. **Code generation errors**: Generated code had duplicate type definitions, unresolved imports, and type conflicts
3. **Abstract type handling**: Many BPMN types are abstract with substitution groups that don't map cleanly to Rust

## Decision

**Handcraft minimal Rust types for the BPMN 2.0 subset defined in spec.md.**

The types will use `serde` derives for both XML and YAML serialization, use `quick-xml` for XML parsing, and cover only the elements required by the MVP spec.

Supported BPMN elements (per spec.md):

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

Implementation: create handcrafted types under the BPMN **adapter** layer of the
relevant feature slice (e.g.
`src/features/convert_bpmn_to_mdx/adapters/bpmn/types/`) with `Definitions`
(root element), `Process`, `FlowElement` enum (StartEvent, EndEvent, Task
variants, Gateway variants), `SequenceFlow`, and common attributes (id, name,
incoming, outgoing). These types are the compile/import IR ([[ADR-2]]); they
must not be placed in or re-exported from the domain layer ([[ADR-7]]).

### Options

1. **XSD codegen via `xsd-parser`**: Automated but failed due to schema complexity (5+ interconnected schemas, complex inheritance, abstract types with substitution groups that don't map cleanly to Rust).
2. **XSD → Protobuf → Rust**: No mature converter exists. GitHub search for "xsd to protobuf" returned only 2 results, both doing the reverse direction (proto→xsd). Tools like `xsdata` (Python) can generate code from XSD but don't output .proto. Would require manual Proto schema creation anyway. Not viable.
3. **Handcrafted types**: Manual but tailored to exactly the subset needed — chosen.
4. **XSD → JSON Schema** (future option): For runtime YAML validation. Could use `xsdata` (Python) with JSON output format to generate JSON Schema from XSD, then validate YAML against JSON Schema using a Rust crate like `jsonschema`. This could be a pre-commit hook or test-time validation, not build-time codegen.

### Rationale

1. **Immediate compilation**: No build-time codegen complexity or build-script dependencies
2. **Types tailored exactly to our needs**: Only the elements required by spec.md
3. **Full control over serde attributes**: Fine-grained control over YAML frontmatter format
4. **WASM-compatible**: No build-script dependencies at runtime
5. **Simpler dependency tree**: Removes `xsd-parser` from build dependencies

## Consequences

### Positive

1. Immediate compilation, no build-time codegen complexity
2. Types tailored exactly to our needs
3. Full control over serde attributes for YAML frontmatter format
4. WASM-compatible (no build-script dependencies at runtime)
5. Simpler dependency tree

### Negative

1. Manual maintenance if XSD changes (rare for BPMN 2.0, stable since 2011)
2. Risk of drift from XSD (mitigated by test-time validation: unit tests parse known-good BPMN files into Rust types, round-trip tests serialize back to XML and validate with `xmllint --schema`, integration tests compare generated BPMN with reference files)
3. Need to add types manually as we expand BPMN coverage
