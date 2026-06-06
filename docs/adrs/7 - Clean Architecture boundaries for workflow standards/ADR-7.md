---
type: adr
title: ADR-7 - Clean Architecture Boundaries for Workflow Standards
date: 2026-04-01
status: proposed
supersedes:
---

## Context

detent currently focuses on BPMN first, but the longer-term direction is broader:

1. Support multiple workflow standards over time
2. Leverage each standard's formal definition where possible
3. Avoid coupling the execution engine to BPMN-specific XML/XSD concerns
4. Keep the codebase extensible even when future standards use a different formalism than XSD

This means BPMN should be treated as the first standard adapter, not as the shape of the core system.

The current ADR set already points in this direction:

- [[ADR-2]] establishes a shared intermediate representation and bidirectional transformation architecture
- [[ADR-4]] rejects BPMN XSD code generation as the core type strategy and keeps handcrafted types for the supported subset
- [[ADR-5]] prefers screaming architecture and one type per file for clarity
- [[ADR-6]] prefers modern Rust module layout

We now need an explicit architectural boundary so that:

1. Full BPMN XSD validation can be added without turning the engine into a BPMN-specific engine
2. Future standards such as SWS can be added without rewriting execution semantics
3. Code that belongs to validation of formal schemas is separated from code that belongs to workflow semantics and execution

This ADR establishes the target architectural boundary. Follow-up decisions should define: the exact standard-neutral workflow IR; whether BPMN and future standards normalize into one IR or a small family of IRs; the minimal application ports needed for execution and validation; and how to package native and WASM schema validators behind the same port.

## Decision

Adopt Clean Architecture boundaries with four conceptual layers:

1. Domain
2. Application
3. Adapter
4. Infrastructure

The core design rule is:

**The execution model and workflow semantics live in a standard-neutral core. Standard-specific parsing, schema validation, and serialization live at the edges.**

As detent grows beyond BPMN-only support, the codebase should move toward the following split:

- **Domain**: workflow IR, graph invariants, execution state machine, retry and routing policies
- **Application**: compile/import/validate/step/status/log use cases, ports for schema validation, persistence, telemetry, locks, service invocation
- **Adapters**: BPMN parser/serializer/normalizer, MDX parser/serializer/normalizer, future SWS parser/serializer/normalizer, CLI input/output translation
- **Infrastructure**: libxml-based XSD validator, file-backed run state store, lock implementation, host runtime bridges

### Options

1. **BPMN-first architecture**: BPMN types and XML concerns permeate the codebase — tighter coupling
2. **Clean Architecture with standard-neutral core**: Enforces boundaries via layers — chosen
3. **Plugin-based architecture**: Dynamic loading of standard adapters — overengineered for MVP

### Rationale

This architecture is grounded in Robert C. Martin's *Clean Architecture*, Eric Evans' *Domain-Driven Design*, and Vaughn Vernon's *Implementing Domain-Driven Design*. It builds on established ADRs: [[ADR-2]] (bidirectional compiler architecture), [[ADR-4]] (handcrafted BPMN types), [[ADR-5]] (screaming architecture), and [[ADR-6]] (`folder.rs` module style).

**Architectural Boundary:**

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

**Layer Responsibilities:**

*Domain.* The Domain layer contains the business model of a workflow engine independent of BPMN, XSD, XML, CLI, filesystem, or host SDKs. It owns: standard-neutral workflow IR; domain concepts such as nodes, edges, run state, retry policy, conditions, error routing; invariants and semantic validation rules; deterministic execution ordering rules; execution state transitions. It must not depend on: `clap`, `libxml`, filesystem APIs, XML/YAML parsing libraries except where unavoidable at leaf conversion points outside the core, BPMN-specific schema types.

*Application.* The Application layer orchestrates use cases. It owns: compile MDX to workflow artifact; import BPMN to MDX; validate input artifacts; advance one execution step; produce status/log views; coordinate ports for persistence, locking, telemetry, and service invocation. It does not own: BPMN XML parsing details, libxml schema loading details, filesystem details, CLI printing.

*Adapter.* The Adapter layer translates external representations into the domain/application model and back. Examples: BPMN XML adapter, MDX frontmatter/body adapter, future SWS adapter, CLI request/response adapter, WASM/JS adapter. This is where format-specific validation belongs before normalization into the domain IR.

*Infrastructure.* The Infrastructure layer implements technical capabilities required by the application ports. Examples: `libxml`-based XSD validator, filesystem state store, atomic write + fsync + rename implementation, locking implementation, clock, random jitter, hashing, telemetry sinks, host SDK bridges for Node/Python/WASM.

**Module Organization.** These layers are architectural boundaries, not a requirement to abandon screaming architecture. This ADR does not override [[ADR-5]] or [[ADR-6]]. We should preserve discoverability while enforcing dependency direction. That means either of the following can be valid: top-level layer modules such as `domain/`, `application/`, `adapters/`, `infrastructure/`; or capability-first modules with internal layer boundaries where appropriate.

The important rule is dependency direction:

1. Domain depends on nothing outward
2. Application depends on Domain
3. Adapters depend on Application and Domain contracts
4. Infrastructure depends on Application ports and technical libraries

If these two goals conflict, dependency direction is more important than folder aesthetics.

**Rust-Specific Guidance.** The Rust-native version of these patterns should prefer: newtypes over primitive strings in the core; enums over inheritance hierarchies; traits for ports, not for everything; free functions for stateless domain services; generic parameters where static dispatch is useful; `dyn Trait` only where runtime variability is needed; `Result<T, E>` with domain/application-specific error enums instead of exception-style control flow; module boundaries and visibility rules instead of deep object graphs.

Avoid translating OO pattern names too literally. For example: do not create Java-style service classes for every action; do not model every concept as a trait if an enum or plain struct is clearer; do not let infrastructure crates define core types.

**Design Patterns by Layer.** Rust is not classically object-oriented, but the same design ideas still apply through enums, traits, modules, newtypes, and free functions. The goal is not to force GoF vocabulary everywhere, but to choose patterns that solve the actual architectural problem in a Rust-native way.

*Domain Patterns:*

1. *Value Object.* Use for IDs, names, digests, expressions, and other validated primitives. Prevents invalid strings from spreading through the core; makes cross-standard normalization safer.

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

2. *Entity / Aggregate.* Use for concepts with identity and invariants, such as `Workflow`, `Run`, `NodeState`. Keeps invariants near the model; avoids scattering workflow rules across adapters.

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

3. *Domain Service.* Use when logic does not naturally belong to a single entity. Examples: topological ordering, runnable node selection, condition evaluation planning. Free functions in a module are often cleaner than forcing methods; keeps entities smaller and preserves explicit dependencies.

```rust
pub fn next_runnable_node(
    workflow: &Workflow,
    run: &RunState,
) -> Result<Option<NodeId>, DomainError> {
    // deterministic selection logic
    # Ok(None)
}
```

4. *Specification / Policy.* Use for composable domain rules. Makes semantic validation modular; lets the engine apply the same rules regardless of source standard.

```rust
pub trait WorkflowRule {
    fn check(&self, workflow: &Workflow) -> Result<(), DomainError>;
}

pub struct ReachabilityRule;
pub struct GatewayInvariantRule;
```

Or, more simply, a collection of free functions if dynamic composition is unnecessary.

5. *State Machine.* Execution is explicitly stateful and deterministic per the spec. Encodes legal transitions directly in types and pattern matches; matches Rust's strengths better than mutable OO state objects.

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

*Application Patterns:*

1. *Use Case / Application Service.* Each user-visible action should be represented as an explicit application use case. Examples: `CompileWorkflow`, `ImportWorkflow`, `ValidateWorkflow`, `StepRun`, `GetRunStatus`. Makes orchestration explicit; keeps CLI handlers thin; provides a stable place to compose multiple adapters and infrastructure services.

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

2. *Port and Adapter.* Application depends on traits that describe what it needs, not on concrete IO implementations. Keeps the application layer testable; makes native, WASM, and future backends swappable.

```rust
pub trait SchemaValidator {
    fn validate(&self, source: &WorkflowSource) -> Result<(), AppError>;
}

pub trait RunStateStore {
    fn load_run(&self, run_id: &RunId) -> Result<RunState, AppError>;
    fn save_node_state(&self, state: &NodeState) -> Result<(), AppError>;
}
```

3. *Command / Query Separation.* Use separate request types for mutating and read-only use cases. Avoids monolithic service APIs; maps cleanly to CLI commands and future host SDK APIs.

```rust
pub struct StepRunCommand {
    pub run_id: RunId,
    pub force: bool,
}

pub struct GetRunStatusQuery {
    pub run_id: RunId,
}
```

4. *Pipeline.* Validation and compilation naturally form explicit stages. Keeps the sequence of responsibilities visible; makes it obvious which step is standard-specific vs core-semantic.

```rust
pub fn validate_bpmn(input: &WorkflowSource) -> Result<ValidationReport, AppError> {
    validate_schema(input)?;
    let workflow = parse_bpmn_to_ir(input)?;
    validate_workflow_semantics(&workflow)?;
    Ok(ValidationReport::success())
}
```

*Adapter Patterns:*

1. *Translator / Mapper.* Translate from BPMN XML types, MDX frontmatter, or future SWS structures into the domain IR. Prevents BPMN-specific details from leaking into the domain; supports multiple source standards cleanly.

```rust
pub trait IntoWorkflow {
    fn into_workflow(self) -> Result<Workflow, AdapterError>;
}
```

Examples: BPMN `Definitions` to `Workflow`; MDX folder to `Workflow`; `Workflow` to BPMN XML definitions.

2. *Anti-Corruption Layer.* Each external standard should be isolated behind its own translation boundary. For BPMN, this means: parse and validate BPMN according to BPMN rules; normalize into detent's workflow IR; never let raw BPMN XML structures become the engine's core model. This becomes even more important when adding SWS or another workflow notation.

3. *Presenter / Formatter.* CLI output and future API output formatting belong here, not in use-case orchestration.

```rust
pub fn render_validation_result(report: &ValidationReport) -> String {
    // text output for CLI
    # String::new()
}
```

*Infrastructure Patterns:*

1. *Repository Implementation.* The trait lives at the application boundary; the concrete implementation lives in infrastructure.

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

2. *Gateway / Client Wrapper.* Use for libraries and host integrations with awkward APIs. Examples: `libxml` validator wrapper, Node/Python host invocation wrapper, WASM interop wrapper. Contains third-party API quirks; prevents library-specific types from leaking upward.

```rust
pub struct LibxmlSchemaValidator;

impl SchemaValidator for LibxmlSchemaValidator {
    fn validate(&self, source: &WorkflowSource) -> Result<(), AppError> {
        // call libxml here
        # Ok(())
    }
}
```

3. *Strategy via Trait Implementations.* Rust does Strategy naturally through traits and concrete implementations. Examples: native XSD validator vs WASM XSD validator; filesystem lock vs no-op lock in tests; real clock vs test clock.

4. *Builder for Technical Configuration.* Use builders when constructing infrastructure services with many configuration options. This is appropriate in infrastructure wiring, but should not be the primary modeling tool for the domain.

```rust
pub struct EngineServicesBuilder {
    // optional infrastructure dependencies
}
```

## Consequences

### Positive

1. Standard-neutral core allows future workflow standards (SWS, etc.) without engine rewrites
2. Format-specific validation and parsing are properly isolated at the edges
3. Application layer becomes testable through port/adapters
4. Domain invariants and execution rules remain in one place regardless of source format
5. Each layer's dependencies point inward, preventing circular coupling
6. This architecture allows future standards to plug in as new adapters without changing the core engine model. BPMN may use XSD + XML parsing; SWS may use another schema or DSL definition system; another future standard may use JSON Schema, Protobuf, or a grammar-based parser. The core requirement is that each standard adapter can normalize its source representation into the same domain IR, or into a compatible family of IRs if the standards diverge materially.
7. Full BPMN XSD schema validation is still useful but only removes code in the outer layers: XML shape checks, required-attribute checks already expressible in the XSD, and some defensive parsing branches caused by permissive XML deserialization. The correct split is schema validation per standard in Adapter/Infrastructure, and semantic workflow validation in Domain/Application.

### Negative

1. More files and modules than a monolithic approach
2. Requires discipline to prevent domain from importing adapter/infrastructure crates
3. Initial refactoring from BPMN-first codebase to layered architecture is non-trivial
4. Schema validation does not replace: domain graph invariants, deterministic execution rules, import/export rules for MDX round-trip behavior, engine state transition rules, or cross-node semantic constraints defined by the spec
