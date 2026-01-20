# BPMN Schema Guidance

- **BPMN20.xsd**: validate the canonical BPMN model (definitions, flow elements, event types, gateways, tasks, etc.). Use this whenever you want to ensure the XML follows the OMG process semantics described in docs/spec.md, for both CLI-generated and hand-edited BPMN artifacts.

- **BPMNDI.xsd**: complement the main model when validating diagram interchange information (shapes, edges, plane layout). Any BPMN file generated for graph rendering typically declares both the BPMN and BPMNDI namespaces.

- **DI.xsd** and **DC.xsd**: these are dependencies of BPMNDI that define diagram constructs (`DI`) and drawing primitives (`DC`). Cascade the validation by pointing your XML tool to both files whenever BPMNDI is referenced so the entire diagram layer resolves.

- **Semantic.xsd**: contains property-type semantics (for example, `tFormalExpression` data). Include it when your process references expressions or extensions that rely on the semantic definitions bundled with BPMN20.

For full validation, supply all five schemas to your XML tool (or reference them via `xsi:schemaLocation`) so both the core BPMN model and the attached diagram metadata are checked consistently. Store them locally in `src/assets/schemas` so deterministic tooling (like a `detent validate` helper) can point to the same approved copies across runs and CI.
