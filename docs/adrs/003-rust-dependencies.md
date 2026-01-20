# ADR-003: Rust Dependencies and Stack

## Status
Accepted

## Date
2026-01-20

## Context

We need to select Rust dependencies that:
1. Work in WASM targets (for in-browser execution)
2. Are performant
3. Are well-maintained
4. Minimize reinvention

## Decision

### Core Dependencies

| Purpose | Crate | Rationale |
|---------|-------|-----------|
| **XSD → Rust codegen** | `xsd-parser` | Generates types from XSD with serde/quick-xml support |
| **XML parsing** | `quick-xml` | Fast, async-capable, WASM-compatible |
| **YAML parsing** | `serde_yaml` | Already in project, works with serde |
| **JSON** | `serde_json` | Already in project |
| **Serialization** | `serde` | Already in project, derive macros |
| **MDX frontmatter** | custom parser | Simple YAML between `---` delimiters |

### Build Dependencies

| Purpose | Crate | Rationale |
|---------|-------|-----------|
| **Code generation** | `xsd-parser` | Build-time type generation |

### Considered but Deferred

| Crate | Purpose | Why Deferred |
|-------|---------|--------------|
| `yaserde` | XML+serde | `xsd-parser` + `quick-xml` covers this |
| `xml-schema` | XSD parsing | Less mature than `xsd-parser` |
| `jsonschema` | JSON Schema validation | Not needed with type-based validation |

## WASM Compatibility

All selected crates support `wasm32-unknown-unknown`:
- `quick-xml`: Pure Rust, no system dependencies
- `serde_yaml`: Uses `unsafe-libyaml` feature for WASM
- `serde_json`: Pure Rust
- `xsd-parser`: Build-time only, not in WASM runtime

## Consequences

### Positive
- Minimal dependency surface
- All crates are mature and maintained
- WASM-ready from the start
- Consistent serde-based (de)serialization

### Negative
- `xsd-parser` is a build dependency, increasing build time
- May need to vendor or customize generated code for BPMN edge cases

## Implementation

Add to `Cargo.toml`:

```toml
[dependencies]
quick-xml = { version = "0.39", features = ["serialize"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1"
xsd-parser-types = "0.1"  # Runtime types used by generated code

[build-dependencies]
xsd-parser = "1.4"
```

## Related ADRs
- ADR-001: XSD as Source of Truth
- ADR-002: Bidirectional Compiler Architecture
