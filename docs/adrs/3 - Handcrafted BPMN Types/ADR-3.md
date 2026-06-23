---
type: adr
title: ADR-3 - Handcrafted BPMN Types
date: 2026-01-20
status: accepted
supersedes:
---

## Context

We need Rust types that round-trip BPMN XML and MDX YAML frontmatter for the
supported subset in spec.md. The first approach was **XSD-driven codegen**: use
`xsd-parser` at build time to generate structs from the bundled OMG schemas in
`src/assets/schemas/`, with `quick-xml` + serde for (de)serialization and
optional runtime XSD checks via `libxml` (feature `xsd-validation`).

That approach was abandoned after implementation. Codegen failed because:

1. **Schema complexity**: BPMN 2.0 spans BPMN20, BPMNDI, DI, DC, and Semantic
   XSDs with inheritance and substitution groups that do not map cleanly to Rust
2. **Generated code defects**: duplicate definitions, unresolved imports, abstract
   type handling errors
3. **Wrong abstraction for our scope**: we only need a spec.md subset, not full
   BPMN 2.0 surface area

What we **kept** from the XSD effort:

- Bundled schemas under `src/assets/schemas/` for **optional runtime validation**
  (libxml, native-only, behind `xsd-validation`) — not as codegen input
- `quick-xml` + serde as the serialization stack ([[ADR-2]])

The codebase then moved from flat `src/bpmn/` modules to **feature slices**
(`src/features/convert_bpmn_to_mdx/adapters/bpmn/types/`) while keeping the
handcrafted-type strategy. Types are adapter-layer IR ([[ADR-1]]), not domain
objects ([[ADR-5]]).

## Decision

**Handcraft minimal Rust types for the BPMN 2.0 subset defined in spec.md.**

Types use serde for XML and YAML, `quick-xml` for BPMN parse/serialize, and
live under each slice's BPMN adapter (e.g.
`src/features/convert_bpmn_to_mdx/adapters/bpmn/types/`).

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

Implementation covers `Definitions`, `Process`, per-element structs (tasks,
events, gateways, flows), and common attributes (id, name, incoming, outgoing).
Interleaved process children deserialize via an enum-based visitor (replacing
naive vec-per-type grouping after serde/quick-xml limitations surfaced in
development).

### Options

1. **XSD codegen via `xsd-parser`**: tried first; failed on schema complexity (see Context)
2. **XSD → Protobuf → Rust**: no mature converter; would still require manual schema work
3. **Handcrafted types**: tailored to spec.md subset — **chosen**
4. **XSD → JSON Schema** (future): possible pre-commit/test-time YAML validation without Rust codegen

Rejected alongside codegen:

- **Manual JSON Schema** for frontmatter — duplicate maintenance vs XSD
- **Pure-Rust XSD validators** (`xmlschema`, `xsd` crate) — immature for multi-file BPMN schemas at evaluation time

### Rationale

1. **Shippable**: no build-script codegen or generated-code surgery
2. **Exact subset**: only elements we compile/import/validate
3. **Serde control**: frontmatter field names match BPMN XML conventions
4. **WASM-safe core path**: handcrafted types + quick-xml; libxml optional and infrastructure-only
5. **Simpler deps**: no `xsd-parser` build dependency

## Consequences

### Positive

1. Immediate compilation; full control over types and serde attributes
2. WASM-compatible compile/import path without build-time codegen
3. Round-trip and fixture tests lock behavior to real BPMN files
4. Optional libxml XSD validation still available for native CI when enabled

### Negative

1. Manual type additions when expanding BPMN coverage
2. Theoretical drift from full XSD (mitigated by round-trip tests, optional XSD validation, and reference fixtures)
3. `src/assets/schemas/` can be misread as "source of truth for types" — it is **validation-only**

## Related

- [[ADR-1]] — shared IR for compile/import
- [[ADR-2]] — dependency stack (quick-xml, optional libxml)
- [[ADR-5]] — types stay in adapter layer, not domain
