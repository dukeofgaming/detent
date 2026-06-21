---
type: adr
title: ADR-3 - Rust Dependencies and Stack
date: 2026-01-20
status: accepted
supersedes:
---

## Context

We need to select Rust dependencies that work in WASM targets (for in-browser execution), are performant, are well-maintained, and minimize reinvention. This ADR supports the handcrafted type strategy from [[ADR-4]] and the bidirectional compiler architecture from [[ADR-2]].

## Decision

**Compose best-in-class crates rather than using an all-in-one XML framework or hand-rolling parsers.**

Core dependencies:

| Purpose | Crate | Rationale |
|---------|-------|-----------|
| **XML parsing/serialization** | `quick-xml` | Fast, pure Rust, WASM-compatible |
| **YAML parsing** | `serde_yaml` | Works with serde for MDX frontmatter |
| **JSON** | `serde_json` | General serialization |
| **Serialization** | `serde` | Derive macros across formats |
| **CLI** | `clap` | Command-line interface |
| **MDX frontmatter** | custom parser | YAML between `---` delimiters |
| **XSD validation (optional)** | `libxml` | Native-only; feature `xsd-validation` |

Historical note: build-time `xsd-parser` was tried and removed; see [[ADR-4]].

`Cargo.toml` (runtime dependencies):

```toml
[dependencies]
quick-xml = { version = "0.37", features = ["serialize"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1"
clap = { version = "4", features = ["derive"] }
libxml = { version = "0.3", optional = true }

[features]
default = ["graph-validation"]
xsd-validation = ["libxml"]
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
