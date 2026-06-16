---
type: adr
title: ADR-3 - Rust Dependencies and Stack
date: 2026-01-20
status: accepted
supersedes:
---

## Context

We need to select Rust dependencies that work in WASM targets (for in-browser execution), are performant, are well-maintained, and minimize reinvention. This ADR supports the type generation strategy from [[ADR-1]] and the bidirectional compiler architecture from [[ADR-2]].

## Decision

**Compose best-in-class crates rather than using an all-in-one XML framework or hand-rolling parsers.**

Core dependencies:

| Purpose | Crate | Rationale |
|---------|-------|-----------|
| **XSD → Rust codegen** | `xsd-parser` | Generates types from XSD with serde/quick-xml support |
| **XML parsing** | `quick-xml` | Fast, async-capable, WASM-compatible |
| **YAML parsing** | `serde_yaml` | Already in project, works with serde |
| **JSON** | `serde_json` | Already in project |
| **Serialization** | `serde` | Already in project, derive macros |
| **MDX frontmatter** | custom parser | Simple YAML between `---` delimiters |

Build dependencies:

| Purpose | Crate | Rationale |
|---------|-------|-----------|
| **Code generation** | `xsd-parser` | Build-time type generation |

Considered but deferred:

| Crate | Purpose | Why Deferred |
|-------|---------|--------------|
| `yaserde` | XML+serde | `xsd-parser` + `quick-xml` covers this |
| `xml-schema` | XSD parsing | Less mature than `xsd-parser` |
| `jsonschema` | JSON Schema validation | Not needed with type-based validation |

`Cargo.toml` additions:
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

### Options

1. **All-in-one XML framework** (e.g. `yaserde`): Single dependency but less flexible
2. **Compose best-in-class crates**: `quick-xml` + `serde` + `serde_yaml` — chosen
3. **Minimalist**: Hand-roll parsers for each format

### Rationale

1. **Minimal dependency surface**: Few core dependencies, each well-scoped
2. **All crates mature and maintained**: Established ecosystem crates with active maintenance
3. **WASM-ready from the start**: All selected crates support `wasm32-unknown-unknown` — `quick-xml` is pure Rust with no system dependencies, `serde_yaml` uses `unsafe-libyaml` feature for WASM, `serde_json` is pure Rust, and `xsd-parser` is build-time only (not in WASM runtime)
4. **Consistent serde-based (de)serialization**: Single serialization framework across all formats

## Consequences

### Positive

1. Minimal dependency surface
2. All crates are mature and maintained
3. WASM-ready from the start
4. Consistent serde-based (de)serialization

### Negative

1. Build-time `xsd-parser` was removed after [[ADR-4]] superseded codegen; handcrafted types replaced it
2. Optional native-only `libxml` (feature `xsd-validation`) is not WASM-compatible and must live in infrastructure behind a `SchemaValidator` port ([[ADR-7]]), not in adapter modules
