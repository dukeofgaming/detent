SPEC-1-detent

Background

You are building detent: an in-process, language-agnostic BPMN execution engine with round-trip sync between a BPMN XML file and a folder of MDX files.
	•	Source of truth (authoring model): MDX files, one per BPMN node (“symbol”).
	•	Build artifact: a BPMN XML file (.bpmn) generated deterministically from MDX.
	•	Round-trip: a BPMN XML file can be imported back into MDX frontmatter without destroying MDX bodies.
	•	Execution: the engine traverses the workflow as a deterministic state machine, persisting per-node results as Markdown files under .state/<run-id>/.
	•	Observability: each state file contains YAML frontmatter for structured state and newline-delimited JSON (NDJSON) in the body for incremental OpenTelemetry-style events.
	•	CLI: single-step invocation model (one command advances one step), with stdout emitting the aggregated NDJSON execution stream.

This spec defines an implementable MVP that a contractor team can build directly.

⸻

Requirements

Must Have

Workflow representation
	•	MDX source files are one-per-node under a flow folder (default ./flows/).
	•	MDX frontmatter is the canonical model for BPMN nodes and edges.
	•	MDX body is documentation only and is not used for compilation semantics.

BPMN compatibility (MVP subset)
	•	BPMN 2.0 subset support:
	•	Start Event
	•	End Event
	•	Service Task
	•	Script Task (MVP treated as Service Task, host-implemented)
	•	Exclusive Gateway (XOR)
	•	Parallel Gateway (AND split/join)
	•	Sequence Flows
	•	Boundary Error Event (basic)

BPMN file contract
	•	detent always operates on a BPMN file.
	•	Global flag: --bpmn <path>.
	•	If --bpmn is omitted, default path is ./process.bpmn.

Compiler
	•	Compile a set of MDX files into a deterministic BPMN XML file.
	•	Validate schema and graph:
	•	unique IDs
	•	valid incoming/outgoing references
	•	StartEvent exists
	•	all nodes reachable from StartEvent (warn or error; MVP = error)
	•	gateway invariants
	•	optional: disallow cycles in MVP (recommended) unless explicitly enabled later

Importer (round-trip)
	•	Import BPMN XML back to MDX:
	•	Frontmatter is updated/created per node.
	•	Existing MDX bodies are preserved.
	•	Imported frontmatter order may be normalized.

Execution model
	•	Engine executes workflows in-process.
	•	Inputs:
	•	runId (string)
	•	inputData (JSON)
	•	Execution is deterministic:
	•	topological order then id tie-break
	•	one runnable node per detent step invocation
	•	State persistence:
	•	.state/<runId>/<nodeId>.md per executed node
	•	YAML frontmatter stores structured execution status/output
	•	body stores NDJSON event log lines

Idempotency & determinism
	•	detent step must be safe to re-run.
	•	If the previous invocation crashed mid-write, the next invocation recovers safely.
	•	A run is bound to a specific model version digest and refuses to proceed if the BPMN/MDX model changes (unless --force).

Host integration (language agnostic)
	•	Core engine is language-agnostic via a WASM component runtime boundary.
	•	MVP host SDKs:
	•	Node.js
	•	Python
	•	ServiceTask and ScriptTask callouts are executed by host via a function registry.

Observability
	•	NDJSON events in node state files.
	•	detent log prints an aggregated NDJSON stream for the run.

⸻

Should Have
	•	Retries with exponential backoff + jitter (per-node config).
	•	Advisory file locks to serialize concurrent step execution per run.
	•	Dry-run capability for planning next step.
	•	JSON Schemas for MDX frontmatter and state frontmatter.

⸻

Could Have
	•	Watch mode: recompile on MDX change.
	•	Timers and user tasks.
	•	BPMN DI layout preservation.

⸻

Won’t Have (MVP)
	•	Subprocesses, compensation, transactions, multi-instance, message correlation, DMN.

⸻

Method

Concepts & data model

Terminology
	•	Node: a BPMN symbol (StartEvent, ServiceTask, Gateway, etc.). Represented by a single *.mdx file.
	•	Edge: SequenceFlow. Represented implicitly by incoming/outgoing references on nodes.
	•	Flow: a folder containing MDX nodes (e.g., flows/onboarding/).
	•	Run: a single execution instance identified by runId and stored under .state/<runId>/.

Filesystem layout
Recommended structure:

./process.bpmn                 # default BPMN file target
./flows/<flowName>/*.mdx       # sources (MDX per node)
./build/<flowName>.bpmn        # optional output location if you choose
./.state/<runId>/*.md          # per-node execution state

Notes:
	•	detent always expects a BPMN file. If you author multiple flows, treat --bpmn as selecting which flow artifact you operate on.

⸻

MDX source format

Frontmatter rules
	•	YAML frontmatter at the top of each MDX file.
	•	Must contain at minimum:
	•	id (string, unique)
	•	type (string, e.g. bpmn:ServiceTask)
	•	incoming (array of flow IDs, may be empty)
	•	outgoing (array of flow IDs, may be empty)

Node MDX schema (conceptual)

id: <string>
name: <string?>
type: bpmn:StartEvent|bpmn:EndEvent|bpmn:ServiceTask|bpmn:ScriptTask|bpmn:ExclusiveGateway|bpmn:ParallelGateway
incoming: [<flowId>...]
outgoing: [<flowId>...]
documentation: <string?>
extensions: <map?>

# Service/Script task additions
implementation: service
serviceRef: <string>
ioSpec:
  inputMapping: <map?>
  outputMapping: <map?>
retry:
  maxAttempts: <int>
  backoff:
    baseMs: <int>
    factor: <float>
    jitter: <bool>
  
# XOR gateway additions
conditions:
  <flowId>: <expr>
  <flowId>: otherwise

# Boundary error (basic)
boundaryErrors:
  - errorRef: <string>
    outgoing: [<flowId>...]

Example MDX files
StartEvent (StartEvent_1.mdx)

---
id: StartEvent_1
type: bpmn:StartEvent
incoming: []
outgoing: [Flow_Start_to_Gather]
---
# Start
Docs only.

ServiceTask (Task_GatherProfile.mdx)

---
id: Task_GatherProfile
name: Gather Profile
type: bpmn:ServiceTask
implementation: service
serviceRef: onboarding.gatherProfile
incoming: [Flow_Start_to_Gather]
outgoing: [Flow_Gather_to_Kyc]
ioSpec:
  inputMapping:
    userId: $.input.userId
  outputMapping:
    profile: $.ctx.profile
retry:
  maxAttempts: 3
  backoff:
    baseMs: 200
    factor: 2.0
    jitter: true
---
## Gather Profile
Docs only.

ExclusiveGateway (Gw_KycRoute.mdx)

---
id: Gw_KycRoute
name: KYC Route
type: bpmn:ExclusiveGateway
incoming: [Flow_Gather_to_Kyc]
outgoing: [Flow_Kyc_Manual, Flow_Kyc_Auto]
conditions:
  Flow_Kyc_Manual: $.ctx.kyc.score < 0.7
  Flow_Kyc_Auto: otherwise
---
## KYC routing
Docs only.


⸻

BPMN XML mapping

Compilation rules (MDX → BPMN)
	•	Compile nodes to BPMN XML elements based on type.
	•	Derive SequenceFlows from node outgoing[] values.

SequenceFlow identity
	•	Flow IDs in incoming/outgoing are string identifiers.
	•	Compiler emits <bpmn:sequenceFlow id="<flowId>" sourceRef="<nodeId>" targetRef="<targetNodeId>"/>.

Target lookup
	•	For each flowId in a node’s outgoing[], compiler finds the unique node that has that flowId in its incoming[].
	•	Validation error if:
	•	no target exists
	•	multiple targets exist

Gateway conditions
	•	For XOR: each outgoing flow may map to <bpmn:conditionExpression>.
	•	Use a standard expression container (e.g., formalExpression) containing the textual condition.

Importer rules (BPMN → MDX)
	•	Parse BPMN XML and reconstruct per-node frontmatter:
	•	preserve node ids
	•	reconstruct incoming/outgoing lists from sequenceFlow graph
	•	extract condition expressions into conditions map on ExclusiveGateway
	•	Merge behavior:
	•	If MDX exists: update frontmatter, keep body.
	•	If MDX does not exist: create file with generated frontmatter and placeholder body.

⸻

Execution engine

Execution overview
	•	Execution is a deterministic stepper.
	•	Every detent step:
	1.	Loads BPMN model (compiled graph) and run state.
	2.	Determines runnable nodes.
	3.	Picks exactly one runnable node (deterministic ordering).
	4.	Executes it (or routes token for gateways/events).
	5.	Persists .state/<runId>/<nodeId>.md.
	6.	Emits NDJSON to stdout (aggregated).

Model versioning
	•	Engine calculates a model digest (e.g., SHA-256) for the current BPMN file content.
	•	Each run stores version digest in node state frontmatter.
	•	step refuses to proceed if digest changed since run start, unless --force.

Runnable rules
Let S(nodeId) be the state file for that node.
	•	StartEvent:
	•	Runnable if run has no state yet (or if StartEvent has not completed).
	•	ServiceTask / ScriptTask:
	•	Runnable when all required inbound tokens have arrived.
	•	ExclusiveGateway:
	•	Runnable when its single inbound token has arrived.
	•	On execute: select exactly one outgoing flow based on condition evaluation.
	•	ParallelGateway:
	•	Split: when inbound token arrives, mark arrivals and enable all outgoing.
	•	Join: runnable when arrivals cover all incoming.
	•	EndEvent:
	•	Runnable when inbound token arrives.

Token tracking
MVP token tracking uses node state files:
	•	Each node state frontmatter maintains:
	•	upstream: list of upstream node IDs that have fired into this node (or a structured arrival set)
	•	For joins (AND): require that arrivals include all upstream nodes implied by incoming flows.

Condition evaluation (XOR)
	•	Expression language: JMESPath.
	•	Evaluation context:
	•	input: initial run input JSON
	•	ctx: mutable run context (outputs merged here)
	•	outputs: map of nodeId -> last output

Selection algorithm
	•	Iterate outgoing flows in the order listed in outgoing[].
	•	For each flow:
	•	if conditions[flowId] == "otherwise", remember as fallback
	•	else evaluate expression; first truthy wins
	•	If none true, choose otherwise if present; otherwise fail node.

Task execution model (MVP)
ServiceTask
	•	Build payload JSON by applying ioSpec.inputMapping:
	•	mapping values are JMESPath selectors or string templates.
	•	Call host:
	•	invoke-service(serviceRef, payloadJson)
	•	On success:
	•	parse returned JSON object
	•	apply ioSpec.outputMapping into ctx (and store node output)
	•	On failure:
	•	record error
	•	apply retry policy if configured

ScriptTask (MVP)
	•	Treated as ServiceTask, executed by host as a registered handler.
	•	No untrusted inline script execution in MVP.

Retries (Should Have; included in MVP if feasible)
	•	Per-task retry.maxAttempts.
	•	Backoff:
	•	baseMs * factor^(attempt-1), optional jitter
	•	Engine behavior:
	•	if a task fails and attempts remain, mark as failed for the attempt but keep the node runnable for future steps until attempts exhausted
	•	when exhausted, either:
	•	route boundary error if configured
	•	or mark run as failed/detented

⸻

State persistence

State file format
	•	Path: .state/<runId>/<nodeId>.md
	•	YAML frontmatter for structured status/output.
	•	Markdown body contains NDJSON events.

Frontmatter schema (conceptual)

runId: <string>
nodeId: <string>
type: <string>
status: pending|running|completed|failed|skipped
attempt: <int>
startedAt: <iso8601>
endedAt: <iso8601?>
upstream: [<nodeId>...]
output: <json?>
error:
  code: <string?>
  message: <string?>
  details: <json?>
version: <string>  # model digest
otel:
  trace_id: <hex>
  span_id: <hex>
  parent_span_id: <hex?>
  attributes: <map>

Body NDJSON
	•	One JSON object per line.
	•	Required keys per event:
	•	time (iso8601)
	•	event.name (string)
	•	attributes (object)

Example body lines:

{"time":"2026-01-17T05:00:00.010Z","event.name":"node.start","attributes":{"bpmn.node_id":"Task_GatherProfile","attempt":1}}
{"time":"2026-01-17T05:00:00.200Z","event.name":"service.invoked","attributes":{"serviceRef":"onboarding.gatherProfile"}}
{"time":"2026-01-17T05:00:00.600Z","event.name":"node.end","attributes":{"status":"completed"}}

Atomic writes
	•	Write to temp file under same directory.
	•	fsync file.
	•	rename temp -> final.
	•	fsync directory.

Locks (Should Have)
	•	Create .state/<runId>/.lock advisory lock.
	•	detent step must acquire run lock before mutating any .state files.

⸻

Language-agnostic boundary (WASM + WIT)

Interface

package bpmn:host

interface services {
  invoke-service: func(service-ref: string, payload: string) -> expected<string, string>
}

interface telemetry {
  emit-log: func(line: string)
}

world engine-host {
  import services
  import telemetry
}

Host SDK behavior (Node + Python)
	•	Provide a registry of service handlers:
	•	register(serviceRef, handlerFn)
	•	Implement invoke-service:
	•	lookup handler
	•	parse payload JSON
	•	call handler
	•	return JSON string on success, stringified error on failure
	•	Implement emit-log:
	•	write to stdout or append to per-run aggregation

⸻

Implementation

Recommended deliverables

Packages
	•	engine/ (Rust): core compiler/importer/executor, compiled to engine.wasm.
	•	cli/ (Rust): detent CLI, embeds or loads engine.wasm.
	•	sdk-node/: host integration for Node.
	•	sdk-python/: host integration for Python.
	•	schemas/: JSON Schemas.
	•	samples/: user onboarding fixtures and demo services.

Suggested MVP tech choices
	•	Rust for core (fast IO, strong typing, WASM tooling).
	•	Wasmtime as embedded runtime (or other stable WASM runtime).
	•	JMESPath for mapping/conditions.

Step-by-step build plan
	1.	Define schemas
	•	schemas/frontmatter.schema.json
	•	schemas/state.schema.json
	2.	MDX parser
	•	Load *.mdx files.
	•	Extract YAML frontmatter.
	•	Validate against schema.
	3.	Graph builder
	•	Build node map by id.
	•	Build flow map by flowId.
	•	Validate:
	•	unique node IDs
	•	each flowId has exactly one source and one target
	•	reachability from StartEvent
	4.	Compiler
	•	Serialize BPMN XML to --bpmn output path.
	•	Ensure deterministic formatting:
	•	stable element ordering
	•	stable attribute ordering
	•	normalized whitespace
	5.	Importer
	•	Parse BPMN XML from --bpmn.
	•	Reconstruct node metadata.
	•	Write/merge flows/<flowName>/<nodeId>.mdx.
	6.	Run initialization
	•	detent run creates .state/<runId>/.
	•	Option A (recommended): create a small run.md metadata file under .state/<runId>/ containing input JSON and model digest.
	•	Seed StartEvent state.
	7.	Stepper
	•	detent step:
	•	lock run
	•	load model digest
	•	load all .state/<runId>/*.md
	•	compute runnable nodes
	•	select deterministic next node
	•	execute + persist
	•	print aggregated NDJSON to stdout
	8.	Execution primitives
	•	Implement StartEvent, EndEvent.
	•	Implement ServiceTask and host calls.
	•	Implement XOR and JMESPath evaluation.
	•	Implement AND split/join.
	•	Implement retries.
	•	Implement boundary error routing.
	9.	Aggregation
	•	detent log reads all node state files for run, concatenates NDJSON body lines, sorts by time, prints to stdout.
	10.	SDKs

	•	Node: registry + WASM loader + implementation of WIT functions.
	•	Python: same.

	11.	Docs + samples

	•	Provide flows/onboarding/ MDX node set.
	•	Provide host demo service implementations for Node and Python.

⸻

Milestones

M0 — Bootstrap
	•	Repo + CI + skeleton packages.

M1 — Schemas + Onboarding sample MDX
	•	Valid frontmatter + graph validation.

M2 — Compiler
	•	detent compile generates deterministic process.bpmn.

M3 — Importer
	•	detent import round-trips without destroying MDX bodies.

M4 — Core stepper + state files + NDJSON
	•	detent run, detent step, .state writes.

M5 — ServiceTask via host registry (Node)
	•	Real callouts + output mapping.

M6 — XOR + JMESPath
	•	Branch selection works.

M7 — Parallel Gateway
	•	split/join correctness.

M8 — CLI completeness
	•	status, log, dry-run.

M9 — Reliability
	•	locks, crash-safety, atomic writes.

M10 — Retries + boundary error
	•	consistent attempt handling.

M11 — Versioning guard
	•	model digest refusal unless --force.

M12 — Python parity
	•	Python SDK executing same sample.

⸻

Gathering Results

Acceptance criteria (MVP)
	•	Round-trip: MDX → BPMN → MDX preserves all semantic frontmatter; preserves MDX bodies.
	•	Execution completes onboarding flow under Node and Python hosts.
	•	Determinism: two runs with identical inputs produce identical normalized states and logs.
	•	Reliability: kill mid-step does not corrupt state; recovery works.
	•	Concurrency: parallel detent step invocations serialize per run.

KPIs (tracked)
	•	.state/<runId> size budget for onboarding sample.
	•	NDJSON log shape compliance (parsable JSON per line; required keys present).
	•	Code coverage threshold.

Test plan
	•	Schema validation tests.
	•	Golden tests for compiler output.
	•	Round-trip tests.
	•	Execution tests:
	•	happy path
	•	XOR branching fixtures
	•	AND join fixture
	•	retries fixture
	•	boundary error fixture
	•	Determinism diff (normalized timestamps/span ids removed).
	•	Crash-safety fault injection.
	•	Concurrency test.

⸻

CLI Specification & Documentation (Examples)

detent always operates on a BPMN file.
	•	Global flag: --bpmn <path>
	•	Default: ./process.bpmn

Compile (MDX → BPMN)

# Compile a flow folder to a BPMN file (explicit)
detent compile flows/onboarding --bpmn build/onboarding.bpmn

# Compile to default BPMN file (writes ./process.bpmn)
detent compile flows/onboarding

Import (BPMN → MDX)

# Import from explicit BPMN into MDX (merge frontmatter, preserve bodies)
detent import --bpmn build/onboarding.bpmn

# Import from default BPMN (reads ./process.bpmn)
detent import

Run (seed a run)

# Seed a new run with input JSON against explicit BPMN
detent run --bpmn build/onboarding.bpmn --run run-demo --input samples/onboarding.json

# Seed a run against default BPMN
detent run --run run-demo --input samples/onboarding.json

Step (advance exactly one node)

# Advance one deterministic step against explicit BPMN
detent step --bpmn build/onboarding.bpmn --run run-demo

# Advance one step against default BPMN
detent step --run run-demo

# Advance until completion (quiet)
while detent step --run run-demo >/dev/null; do :; done

# Force continuation if model digest changed
detent step --run run-demo --force

Status (summarize run)

detent status --bpmn build/onboarding.bpmn --run run-demo
detent status --run run-demo

Log (aggregate NDJSON)

# Print aggregated NDJSON
detent log --bpmn build/onboarding.bpmn --run run-demo
detent log --run run-demo

# Pretty print / save
detent log --run run-demo | jq
detent log --run run-demo > logs/run-demo.ndjson

Dry-run (plan next step without side effects)

detent dry-run --bpmn build/onboarding.bpmn --run run-demo
detent dry-run --run run-demo

Typical end-to-end session

# 1) Compile MDX → BPMN
detent compile flows/onboarding --bpmn build/onboarding.bpmn

# 2) Start a run
detent run --bpmn build/onboarding.bpmn --run run-demo --input samples/onboarding.json

# 3) See next step
detent dry-run --bpmn build/onboarding.bpmn --run run-demo

# 4) Step until done
while detent step --bpmn build/onboarding.bpmn --run run-demo >/dev/null; do :; done

# 5) Inspect
detent status --bpmn build/onboarding.bpmn --run run-demo
detent log --bpmn build/onboarding.bpmn --run run-demo | jq
ls -1 .state/run-demo


⸻

Ralph Wiggum Loop Example (Trending AI Pattern)

This demonstrates the “keeps trying the same thing” pattern without BPMN graph cycles by using retries.

MDX (task with retry)

---
id: Task_RalphTry
name: Ralph Tries Again
type: bpmn:ServiceTask
implementation: service
serviceRef: demo.ralph.loop
incoming: [Flow_Start_to_Ralph]
outgoing: [Flow_Ralph_to_End]
retry:
  maxAttempts: 3
  backoff:
    baseMs: 150
    factor: 2.0
    jitter: true
---
## Ralph
Docs only.

CLI run

# Compile to default ./process.bpmn
detent compile flows/ralph

# Seed the run
detent run --run ralph-1 --input samples/ralph.json

# Step: first two attempts fail transiently; third succeeds
detent step --run ralph-1
detent step --run ralph-1
detent step --run ralph-1

# Finish the run
detent step --run ralph-1

# Inspect attempts and logs
detent status --run ralph-1
detent log --run ralph-1 | jq

