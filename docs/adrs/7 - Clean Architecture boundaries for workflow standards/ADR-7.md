---
type: adr
date: 2026-04-01
status: proposed
---
# ADR-7: Clean Architecture Boundaries for Workflow Standards

## Context

detent currently focuses on BPMN first, but the longer-term direction is broader:

1. Support multiple workflow standards over time
2. Leverage each standard's formal definition where possible
3. Avoid coupling the execution engine to BPMN-specific XML/XSD concerns
4. Keep the codebase extensible even when future standards use a different formalism than XSD

This means BPMN should be treated as the first standard adapter, not as the shape of the core system.

The current ADR set already points in this direction:

- ADR-2 establishes a shared intermediate representation and bidirectional transformation architecture
- ADR-4 rejects BPMN XSD code generation as the core type strategy and keeps handcrafted types for the supported subset
- ADR-5 prefers screaming architecture and one type per file for clarity
- ADR-6 prefers modern Rust module layout

We now need an explicit architectural boundary so that:

1. Full BPMN XSD validation can be added without turning the engine into a BPMN-specific engine
2. Future standards such as SWS can be added without rewriting execution semantics
3. Code that belongs to validation of formal schemas is separated from code that belongs to workflow semantics and execution

## Decision

Adopt Clean Architecture boundaries with four conceptual layers:

1. Domain
2. Application
3. Adapter
4. Infrastructure

The core design rule is:

**The execution model and workflow semantics live in a standard-neutral core. Standard-specific parsing, schema validation, and serialization live at the edges.**

### Architectural Boundary

```text
                ┌──────────────────────────────┐
                │          Adapters            │
                │ BPMN XML | MDX | CLI | SWS   │
                └──────────────┬───────────────┘
                               │
                ┌──────────────▼───────────────┐
                │         Application          │
                │ import | compile | validate  │
                │ run-step | status | log      │
                └──────────────┬───────────────┘
                               │
                ┌──────────────▼───────────────┐
                │            Domain            │
                │ workflow IR | invariants |   │
                │ execution rules | policies   │
                └──────────────┬───────────────┘
                               │
                ┌──────────────▼───────────────┐
                │        Infrastructure        │
                │ libxml | fs | locks | time   │
                │ hashing | host SDK bridges   │
                └──────────────────────────────┘
```

## Layer Responsibilities

### Domain

The Domain layer contains the business model of a workflow engine independent of BPMN, XSD, XML, CLI, filesystem, or host SDKs.

It owns:

1. Standard-neutral workflow IR
2. Domain concepts such as nodes, edges, run state, retry policy, conditions, error routing
3. Invariants and semantic validation rules
4. Deterministic execution ordering rules
5. Execution state transitions

It must not depend on:

1. `clap`
2. `libxml`
3. filesystem APIs
4. XML/YAML parsing libraries except where unavoidable at leaf conversion points outside the core
5. BPMN-specific schema types

### Application

The Application layer orchestrates use cases.

It owns:

1. Compile MDX to workflow artifact
2. Import BPMN to MDX
3. Validate input artifacts
4. Advance one execution step
5. Produce status/log views
6. Coordinate ports for persistence, locking, telemetry, and service invocation

It does not own:

1. BPMN XML parsing details
2. libxml schema loading details
3. filesystem details
4. CLI printing

### Adapter

The Adapter layer translates external representations into the domain/application model and back.

Examples:

1. BPMN XML adapter
2. MDX frontmatter/body adapter
3. Future SWS adapter
4. CLI request/response adapter
5. WASM/JS adapter

This is where format-specific validation belongs before normalization into the domain IR.

### Infrastructure

The Infrastructure layer implements technical capabilities required by the application ports.

Examples:

1. `libxml`-based XSD validator
2. Filesystem state store
3. Atomic write + fsync + rename implementation
4. Locking implementation
5. Clock, random jitter, hashing
6. Telemetry sinks
7. Host SDK bridges for Node/Python/WASM

## Consequences

### What Full BPMN XSD Validation Helps Delete

Full schema validation is still useful, but it only removes code in the outer layers.

It can replace:

1. XML shape checks
2. Required-attribute checks already expressible in the XSD
3. Some defensive parsing branches caused by permissive XML deserialization

It does **not** replace:

1. Domain graph invariants
2. Deterministic execution rules
3. Import/export rules for MDX round-trip behavior
4. Engine state transition rules
5. Cross-node semantic constraints defined by the spec

So the correct split is:

1. Schema validation per standard in Adapter/Infrastructure
2. Semantic workflow validation in Domain/Application

### Multi-Standard Support

This architecture allows future standards to plug in as new adapters without changing the core engine model.

For example:

1. BPMN may use XSD + XML parsing
2. SWS may use another schema or DSL definition system
3. Another future standard may use JSON Schema, Protobuf, or a grammar-based parser

The core requirement is that each standard adapter can normalize its source representation into the same domain IR, or into a compatible family of IRs if the standards diverge materially.

## Design Patterns by Layer

Rust is not classically object-oriented, but the same design ideas still apply through enums, traits, modules, newtypes, and free functions.

The goal is not to force GoF vocabulary everywhere. The goal is to choose patterns that solve the actual architectural problem in a Rust-native way.

### Domain Patterns

#### 1. Value Object

Use for IDs, names, digests, expressions, and other validated primitives.

Rust form:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(String);

impl TryFrom<String> for NodeId {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(DomainError::InvalidNodeId);
        }
        Ok(Self(value))
    }
}
```

Why it fits:

1. Prevents invalid strings from spreading through the core
2. Makes cross-standard normalization safer

#### 2. Entity / Aggregate

Use for concepts with identity and invariants, such as `Workflow`, `Run`, `NodeState`.

Rust form:

```rust
pub struct Workflow {
    pub id: WorkflowId,
    pub nodes: BTreeMap<NodeId, Node>,
    pub flows: BTreeMap<FlowId, SequenceFlow>,
}

impl Workflow {
    pub fn validate(&self) -> Result<(), DomainError> {
        validate_unique_ids(self)?;
        validate_reachability(self)?;
        validate_gateway_invariants(self)?;
        Ok(())
    }
}
```

Why it fits:

1. Keeps invariants near the model
2. Avoids scattering workflow rules across adapters

#### 3. Domain Service

Use when logic does not naturally belong to a single entity.

Examples:

1. Topological ordering
2. Runnable node selection
3. Condition evaluation planning

Rust form:

```rust
pub fn next_runnable_node(
    workflow: &Workflow,
    run: &RunState,
) -> Result<Option<NodeId>, DomainError> {
    // deterministic selection logic
    # Ok(None)
}
```

Why it fits:

1. Free functions in a module are often cleaner than forcing methods
2. Keeps entities smaller and preserves explicit dependencies

#### 4. Specification / Policy

Use for composable domain rules.

Rust form:

```rust
pub trait WorkflowRule {
    fn check(&self, workflow: &Workflow) -> Result<(), DomainError>;
}

pub struct ReachabilityRule;
pub struct GatewayInvariantRule;
```

Or, more simply, a collection of free functions if dynamic composition is unnecessary.

Why it fits:

1. Makes semantic validation modular
2. Lets the engine apply the same rules regardless of source standard

#### 5. State Machine

Execution is explicitly stateful and deterministic per the spec.

Rust form:

```rust
pub enum NodeExecutionStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

impl NodeExecutionStatus {
    pub fn transition(self, event: ExecutionEvent) -> Result<Self, DomainError> {
        match (self, event) {
            (Self::Pending, ExecutionEvent::Start) => Ok(Self::Running),
            (Self::Running, ExecutionEvent::Succeed) => Ok(Self::Succeeded),
            (Self::Running, ExecutionEvent::Fail) => Ok(Self::Failed),
            _ => Err(DomainError::InvalidTransition),
        }
    }
}
```

Why it fits:

1. Encodes legal transitions directly in types and pattern matches
2. Matches Rust's strengths better than mutable OO state objects

### Application Patterns

#### 1. Use Case / Application Service

Each user-visible action should be represented as an explicit application use case.

Examples:

1. `CompileWorkflow`
2. `ImportWorkflow`
3. `ValidateWorkflow`
4. `StepRun`
5. `GetRunStatus`

Rust form:

```rust
pub struct ValidateWorkflow<P, S> {
    pub parser: P,
    pub schema_validator: S,
}

impl<P, S> ValidateWorkflow<P, S>
where
    P: WorkflowParser,
    S: SchemaValidator,
{
    pub fn execute(&self, input: ValidationRequest) -> Result<ValidationReport, AppError> {
        self.schema_validator.validate(&input.source)?;
        let workflow = self.parser.parse(&input.source)?;
        workflow.validate()?;
        Ok(ValidationReport::success())
    }
}
```

Why it fits:

1. Makes orchestration explicit
2. Keeps CLI handlers thin
3. Provides a stable place to compose multiple adapters and infrastructure services

#### 2. Port and Adapter

Application depends on traits that describe what it needs, not on concrete IO implementations.

Rust form:

```rust
pub trait SchemaValidator {
    fn validate(&self, source: &WorkflowSource) -> Result<(), AppError>;
}

pub trait RunStateStore {
    fn load_run(&self, run_id: &RunId) -> Result<RunState, AppError>;
    fn save_node_state(&self, state: &NodeState) -> Result<(), AppError>;
}
```

Why it fits:

1. Keeps the application layer testable
2. Makes native, WASM, and future backends swappable

#### 3. Command / Query Separation

Use separate request types for mutating and read-only use cases.

Rust form:

```rust
pub struct StepRunCommand {
    pub run_id: RunId,
    pub force: bool,
}

pub struct GetRunStatusQuery {
    pub run_id: RunId,
}
```

Why it fits:

1. Avoids monolithic service APIs
2. Maps cleanly to CLI commands and future host SDK APIs

#### 4. Pipeline

Validation and compilation naturally form explicit stages.

Rust form:

```rust
pub fn validate_bpmn(input: &WorkflowSource) -> Result<ValidationReport, AppError> {
    validate_schema(input)?;
    let workflow = parse_bpmn_to_ir(input)?;
    validate_workflow_semantics(&workflow)?;
    Ok(ValidationReport::success())
}
```

Why it fits:

1. Keeps the sequence of responsibilities visible
2. Makes it obvious which step is standard-specific vs core-semantic

### Adapter Patterns

#### 1. Translator / Mapper

Translate from BPMN XML types, MDX frontmatter, or future SWS structures into the domain IR.

Rust form:

```rust
pub trait IntoWorkflow {
    fn into_workflow(self) -> Result<Workflow, AdapterError>;
}
```

Examples:

1. BPMN `Definitions` to `Workflow`
2. MDX folder to `Workflow`
3. `Workflow` to BPMN XML definitions

Why it fits:

1. Prevents BPMN-specific details from leaking into the domain
2. Supports multiple source standards cleanly

#### 2. Anti-Corruption Layer

Each external standard should be isolated behind its own translation boundary.

For BPMN, this means:

1. Parse and validate BPMN according to BPMN rules
2. Normalize into detent's workflow IR
3. Never let raw BPMN XML structures become the engine's core model

This becomes even more important when adding SWS or another workflow notation.

#### 3. Presenter / Formatter

CLI output and future API output formatting belong here, not in use-case orchestration.

Rust form:

```rust
pub fn render_validation_result(report: &ValidationReport) -> String {
    // text output for CLI
    # String::new()
}
```

### Infrastructure Patterns

#### 1. Repository Implementation

The trait lives at the application boundary; the concrete implementation lives in infrastructure.

Rust form:

```rust
pub struct FileRunStateStore {
    root: PathBuf,
}

impl RunStateStore for FileRunStateStore {
    fn load_run(&self, run_id: &RunId) -> Result<RunState, AppError> {
        // fs implementation
        # unimplemented!()
    }

    fn save_node_state(&self, state: &NodeState) -> Result<(), AppError> {
        // atomic write implementation
        # unimplemented!()
    }
}
```

#### 2. Gateway / Client Wrapper

Use for libraries and host integrations with awkward APIs.

Examples:

1. `libxml` validator wrapper
2. Node/Python host invocation wrapper
3. WASM interop wrapper

Rust form:

```rust
pub struct LibxmlSchemaValidator;

impl SchemaValidator for LibxmlSchemaValidator {
    fn validate(&self, source: &WorkflowSource) -> Result<(), AppError> {
        // call libxml here
        # Ok(())
    }
}
```

Why it fits:

1. Contains third-party API quirks
2. Prevents library-specific types from leaking upward

#### 3. Strategy via Trait Implementations

Rust does Strategy naturally through traits and concrete implementations.

Examples:

1. Native XSD validator vs WASM XSD validator
2. Filesystem lock vs no-op lock in tests
3. Real clock vs test clock

#### 4. Builder for Technical Configuration

Use builders when constructing infrastructure services with many configuration options.

Rust form:

```rust
pub struct EngineServicesBuilder {
    // optional infrastructure dependencies
}
```

This is appropriate in infrastructure wiring, but should not be the primary modeling tool for the domain.

## Rust-Specific Guidance

The Rust-native version of these patterns should prefer:

1. Newtypes over primitive strings in the core
2. Enums over inheritance hierarchies
3. Traits for ports, not for everything
4. Free functions for stateless domain services
5. Generic parameters where static dispatch is useful
6. `dyn Trait` only where runtime variability is needed
7. `Result<T, E>` with domain/application-specific error enums instead of exception-style control flow
8. Module boundaries and visibility rules instead of deep object graphs

Avoid translating OO pattern names too literally.

For example:

1. Do not create Java-style service classes for every action
2. Do not model every concept as a trait if an enum or plain struct is clearer
3. Do not let infrastructure crates define core types

## Module Organization

These layers are **architectural boundaries**, not a requirement to abandon screaming architecture.

This ADR does **not** override ADR-5 or ADR-6.

We should preserve discoverability while enforcing dependency direction. That means either of the following can be valid:

1. Top-level layer modules such as `domain/`, `application/`, `adapters/`, `infrastructure/`
2. Capability-first modules with internal layer boundaries where appropriate

The important rule is dependency direction:

1. Domain depends on nothing outward
2. Application depends on Domain
3. Adapters depend on Application and Domain contracts
4. Infrastructure depends on Application ports and technical libraries

If these two goals conflict, dependency direction is more important than folder aesthetics.

## Initial Refactoring Direction

As detent grows beyond BPMN-only support, move toward the following split:

1. Domain
   - workflow IR
   - graph invariants
   - execution state machine
   - retry and routing policies
2. Application
   - compile/import/validate/step/status/log use cases
   - ports for schema validation, persistence, telemetry, locks, service invocation
3. Adapters
   - BPMN parser/serializer/normalizer
   - MDX parser/serializer/normalizer
   - future SWS parser/serializer/normalizer
   - CLI input/output translation
4. Infrastructure
   - libxml-based XSD validator
   - file-backed run state store
   - lock implementation
   - host runtime bridges

## Status and Follow-Up

This ADR establishes the target architectural boundary.

Follow-up decisions should define:

1. The exact standard-neutral workflow IR
2. Whether BPMN and future standards normalize into one IR or a small family of IRs
3. The minimal application ports needed for execution and validation
4. How to package native and WASM schema validators behind the same port

## References

1. Robert C. Martin, *Clean Architecture*
2. Eric Evans, *Domain-Driven Design*
3. Vaughn Vernon, *Implementing Domain-Driven Design*
4. ADR-2: Bidirectional Compiler Architecture
5. ADR-4: Handcrafted BPMN Types Instead of XSD Codegen
6. ADR-5: Screaming Architecture with One Type Per File
7. ADR-6: Use `folder.rs` Instead of `folder/mod.rs` for Module Definitions