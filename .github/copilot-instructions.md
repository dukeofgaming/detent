# Copilot instructions for detent

## Big picture
- detent is a single-crate Rust starting point whose primary documentation lives in [docs/spec.md](docs/spec.md), which defines the BPMN/MDX round-trip, execution engine, and CLI workflows that every change should align with.
- The current binary in [Cargo.toml](Cargo.toml) only pulls in `serde`, `serde_json`, and `serde_yaml`; [src/main.rs](src/main.rs) is a placeholder that prints Hello, so new logic must be built around this scaffold rather than modifying an existing flow.

## Architecture hints to internalize
- The spec says MDX files under the flow folders described in [docs/spec.md](docs/spec.md) are the canonical node definitions, with YAML frontmatter holding id, type, incoming, outgoing, implementation, serviceRef, ioSpec, retry, gateway conditions, and boundary errors; MDX bodies stay untouched except as documentation.
- Compiler output is always a deterministic BPMN document (the spec treats a default BPMN file as the fallback when `--bpmn` is omitted) with stable ordering, normalized whitespace, and sequence flows derived directly from outgoing references in the MDX graph.
- Execution persists per-node Markdown state files with YAML frontmatter (run metadata, upstream arrivals, model digest, attempt counts, otel attributes) and an NDJSON body of events; writes must be atomic (temp file + fsync + rename) and each run should acquire a lock before mutating run state, as the spec prescribes.
- The spec envisions a WASM-host boundary (see the `engine-host` WIT world in [docs/spec.md](docs/spec.md)) where the engine calls services.invoke-service and telemetry.emit-log; Node and Python SDKs should register handlers and forward the payload/response JSON strings.

## Developer workflows
- Build and test using the standard Rust commands (`cargo build`, `cargo test`). Expect the only current test suite to live in [src/main.rs](src/main.rs) and assert that the hello helper still returns the same string.
- Follow the CLI flow documented in [docs/spec.md](docs/spec.md): compile, import, run, dry-run, step, status, and log commands should all honor the deterministic state machine in the spec, keep the run-state tree and NDJSON logs consistent, and respect the model digest guard unless `--force` is passed.
- Remember that the spec treats the generated BPMN artifact as the single source of execution (with `--bpmn` selecting the file and a documented default when omitted) and that future work should place MDX nodes under the described flow folders.

## Patterns and conventions
- Determinism is non-negotiable: model digests guard each run (reject steps when the BPMN/MDX model changes unless --force is passed), runnable nodes are ordered by topological sort then ID, and gateway condition evaluation uses JMESPath over input, ctx, and outputs maps.
- Execution state is step-by-step: each detent step invocation loads the compiled graph, reads every run-state file, identifies exactly one runnable node, executes it, persists the resulting state artifact, and emits aggregated NDJSON to stdout (sorted by timestamp) so that logs can be re-played or inspected via detent log.
- Service/Script tasks map input/output via ioSpec (JMESPath selectors or string templates), obey per-node retry policies with backoff/jitter, and can route to boundary error flows when retries are exhausted; the Ralph Wiggum loop example in the spec demonstrates repeated attempts.
- Parallel gateways track upstream arrivals before allowing AND joins, Exclusive gateways evaluate each outgoing flow in list order (with an otherwise fallback), and the start/end events gate the run lifecycle exactly as documented in the spec.

## Integration and observability hints
- Node state files mix structured frontmatter with NDJSON bodies; the spec requires each body line to include time, event.name, and attributes, so tooling that reads/writes these files must parse both sections and keep NDJSON sorted for the aggregated log command.
- Hosts (Node/Python) should provide registries for service handlers, parse JSON payloads received via services.invoke-service, and forward logs through telemetry.emit-log (stdout or per-run aggregation) so that the engine stays language-agnostic.

## When you're unsure
- Treat [docs/spec.md](docs/spec.md) as the source of truth; cite relevant sections when proposing a deviation.
- No additional hidden scripts or conventions exist beyond what is outlined in the spec—if you need samples (flows, inputs, retries), ask for fixtures before inventing them.
- After implementing new workflows, rerun `cargo test` and consider adding schema/golden tests inspired by the spec's test plan if coverage is still minimal.

## Feedback loop
- If any instruction seems vague or you need more detail about node modeling, CLI flags, or state persistence, please point it out so I can clarify or expand this guide.