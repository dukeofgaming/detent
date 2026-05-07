# detent

A stepper workflow engine with BPMN support, written in Rust.

## Design philosophy

- **Local First**: Many BPMN execution engines require infrastructure to run, detent is self-contained as a single binary.

- **CLI-first**: Designed with a command-line interface in mind, making it easy to integrate into scripts and automation workflows.

- **MDX as source of truth**: Workflows are authored as a folder of MDX files (one per flow element), compiled deterministically to BPMN XML. Round-trip is lossless for frontmatter semantics.

## CLI Reference

### Global flags

| Flag | Description |
|------|-------------|
| `--help` | Print help information |
| `--version` | Print version information |

### Commands

#### `detent validate <files...>`

Validate BPMN (`.bpmn`, `.bpmn2`) or MDX (`.mdx`) files.

**Validation pipeline:**
1. Parse file (XML for BPMN, YAML frontmatter for MDX)
2. XSD schema validation (BPMN only, when `xsd-validation` feature is enabled)
3. Structural validation (element IDs, required fields)
4. Graph semantic validation (start/end event presence, dangling references, duplicate IDs)

**Exit codes:** `0` all files valid, `1` any file invalid.

**Examples:**
```
detent validate process.bpmn
detent validate flows/*.mdx
detent validate process.bpmn flows/*.mdx
```

---

#### `detent import <bpmn_file> [-o <output_directory>]`

Import a BPMN XML file into MDX files — one `.mdx` per flow element.

Extracts frontmatter from each BPMN element (start events, end events, tasks, gateways, sequence flows) and writes them as individual MDX files. Existing MDX bodies are preserved on re-import (merge behavior).

**Arguments:**
| Argument | Description |
|----------|-------------|
| `bpmn_file` | Path to BPMN XML file (required) |
| `-o, --output-directory` | Output directory for MDX files (default: `.`) |

**Examples:**
```
detent import process.bpmn
detent import build/onboarding.bpmn -o flows/onboarding
```

---

#### `detent compile <directory> [-o <output>]`

Compile a directory of MDX files into BPMN XML.

Reads all `.mdx` files from the directory, parses their frontmatter according to the `type:` field (e.g. `bpmn:startEvent`, `bpmn:task`, `bpmn:sequenceFlow`), and assembles a BPMN `Definitions` document. Runs graph validation before emitting output.

**Arguments:**
| Argument | Description |
|----------|-------------|
| `directory` | Directory containing `.mdx` files (required) |
| `-o, --output` | Output BPMN XML file (default: stdout) |

**Examples:**
```
detent compile flows/onboarding
detent compile flows/onboarding -o build/process.bpmn
```

---

### Data flow

```
  ┌──────────┐    import     ┌──────┐
  │ BPMN XML │ ──────────→  │ MDX  │
  │ .bpmn    │ ←──────────  │ .mdx │
  └──────────┘    compile    └──────┘
       │                          │
       │ validate                 │ validate
       ▼                          ▼
   parse + XSD +             parse YAML +
   structural +              structural +
   graph semantic            graph semantic
```

## Project layout

| Path | Layer | Description |
|------|-------|-------------|
| `lib/core/` | Domain | Standard-neutral types and graph operations |
| `src/compiler/bpmn/` | Adapter | BPMN parse/serialize/types + XSD validation |
| `src/compiler/mdx/` | Adapter | MDX frontmatter types |
| `src/compiler/commands/` | Application | CLI command implementations |
| `src/graph_validation/` | Domain (re-export) | Thin shim to `detent-core` |
| `tests/` | Integration | Test suites for compiler and graph validation |
