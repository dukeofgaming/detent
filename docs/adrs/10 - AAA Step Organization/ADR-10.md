---
type: adr
title: ADR-10 - AAA Step Organization
date: 2026-06-21
status: accepted
supersedes:
---

## Context

BDD scenarios follow the Given/When/Then structure. Cucumber treats `Given`,
`When`, and `Then` as interchangeable — it matches step text to registered
functions regardless of the keyword used. This flexibility creates a risk:
step bodies can accumulate multiple responsibilities across the AAA
(Arrange–Act–Assert) boundary, making failures opaque and Gherkin scenarios
misleading.

Concrete patterns observed in our codebase:

### Given steps that Act ("Given I have already done X")

`Given the tdd BPMN fixture is imported to MDX` at
`bpmn_mdx_roundtrip.rs:132` reads a fixture file, parses BPMN XML, and
runs the import use-case — three operations in one step. The scenario has
no `When` step, so the Gherkin is lying about what it exercises.

### When steps that Assert inside the body

`When I inspect the first sequence flow` at `bpmn_types.rs:47` parses BPMN,
then calls three `assert!` macros. The companion `Then` step is a no-op
function body. The Gherkin says "Then the sequence flow has non-empty id
and refs" — but the assertion fires inside the `When`, making the `Then`
step dead code.

Similarly, `When I parse each MDX file` at `mdx_types.rs:182` parses files
and immediately asserts `type:`/`id:` presence. Its `Then` step is empty.

### When steps that chain two architectural layers (Act+Act)

`When I attempt to import the BPMN to MDX` at
`functional/steps/when.rs:19` calls `parse_bpmn` (adapter layer) and then
`import_to_mdx` (use-case layer). If parsing fails, the step sets
`world.import_failed = true`, so a `Then import fails` assertion passes
even though the import layer was never reached.

### Then steps that Act (Act+Assert)

`Then importing the compiled definitions produces 6 MDX outputs` at
`mdx_compiles_and_imports_back.rs:5` calls `import_to_mdx` — an action —
before asserting the output count. A panic in the action layer is
indistinguishable from an assertion failure.

### When steps that also prepare Assert fixtures (Act+Act+Assert prep)

`When I import and compile-roundtrip the fixture` at
`bpmn_mdx_roundtrip.rs:200` runs import, compile, and re-import, then
serializes two hashmaps into a string for later comparison. If the `Then`
fails, the developer sees two opaque serialized maps with no clue which
of the three operations was the culprit.

These patterns share a root cause: a step body performs work from two or
more AAA zones. The Gherkin outline no longer reflects reality, and test
failures are harder to diagnose.

## Decision

**Each step body must stay inside a single AAA zone.** The practical rule:

| Gherkin keyword | AAA zone | What belongs |
|-----------------|----------|-------------|
| **Given** | **Arrange only** | Populate world state from fixtures, helpers, or in-memory builders. Never invoke use-case or adapter entry points. |
| **When** | **Act only** | Invoke exactly one use-case or adapter entry point. Never assert; store the outcome (result, error flag, output) on the World. |
| **Then** | **Assert only** | Inspect World state and assert. Never invoke use-case or adapter entry points. |

Consequences for the problematic examples above:

| Example | Before | After |
|---------|--------|-------|
| Given tdd fixture is imported to MDX | One Given: read + parse + import | Three steps: `Given the tdd BPMN fixture` → `When I import it to MDX` (shared step) → `Then` asserts metadata |
| When I inspect the first sequence flow | When parses and asserts | `When I parse the BPMN XML` → `Then the sequence flow has non-empty id and refs` |
| When I attempt to import | When parses + imports | `When I parse the BPMN to definitions` → `When I import the parsed definitions to MDX` (two separate When steps, each single-layer) |
| Then importing produces N outputs | Then imports + asserts | `When I import the compiled definitions to MDX` → `Then N MDX outputs are produced` |
| When I import and compile-roundtrip | When does 3 operations + comparison prep | `When I import the fixture to MDX` → `When I compile the imported MDX to BPMN` → `When I import the compiled BPMN to MDX` → `Then imported frontmatter matches...` |
| When I roundtrip MDX as type | When parses + serializes + asserts | `When I parse the MDX file as a <type>` → `When I serialize and deserialize the element` → `Then the element survives a serde roundtrip` |

### Option: keep pre-import parsed-defs on World

For the import sequence, we already have a `parsed_defs` field. The
`When I import the parsed definitions to MDX` shared step (at
`functional/steps/when.rs:33`) expects `parsed_defs` to be set. The
refactored flow uses this as an explicit boundary between the parse
and import layers.

### Edge cases

**One-line assertions can live in `Then`.** Short `assert!` / `assert_eq!`
calls after a single `When` action are fine in `Then` — the violation is
when the assertion lives in the `When` body and the `Then` is a no-op.

**Repeated Givens across scenarios.** If the same Arrange block appears in
many scenarios, make it a shared `Given` step in level-`steps/given.rs`.
That's correct because it stays in the Arrange zone.

**What about `And`?** In Gherkin, `And` continues the preceding keyword.
An `And` after `Given` is Arrange; after `When` is Act; after `Then` is
Assert. Follow the same rule based on its position.

## Consequences

### Positive

1. **Self-diagnosing failures.** When a `Then` assertion fails, the
   developer knows the action succeeded and the output was wrong. When a
   `When` step fails, the developer knows the action itself broke.

2. **Honest Gherkin.** The feature file accurately describes what each
   scenario does — no hidden operations in Given or hidden assertions in
   When.

3. **Reusable Act steps.** `When I import the parsed definitions to MDX`
   serves both the import-fails and import-succeeds paths because it stores
   the outcome on the World rather than asserting.

4. **Cleaner World.** The World struct fields (`parse_failed`,
   `compile_failed`, `import_outputs`) have clearer ownership — each is
   written by exactly one `When` and read by one or more `Then` steps.

### Negative

1. **More Gherkin lines per scenario.** Splitting a 3-step scenario into
   5-6 steps adds verbosity. Mitigation: the extra lines are declarative
   and self-explanatory.

2. **Refactoring cost.** ~10 existing step definitions need restructuring
   to separate AAA concerns. Mitigation: this is a one-time cost.

3. **More `When` steps in sequence.** Two consecutive `When` steps (e.g.,
   parse then import) in the same scenario might feel unusual. Mitigation:
   BDD tools accept multiple `When` lines; the convention is clear so long
   as each `When` does exactly one Act.

## Related

- [[ADR-8]] — per-level BDD layout with `steps/` directories
- [[ADR-7]] — screaming architecture applied to step files
- [[ADR-9]] — vertical slice ownership of test code
