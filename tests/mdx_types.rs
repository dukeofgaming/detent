//! Unit tests for MDX types (using BPMN types directly for frontmatter)
//!
//! The MDX frontmatter uses plain field names like `id`, `name`, `sourceRef`
//! which match the BPMN XML attribute names (without the @ prefix that
//! quick-xml uses internally).

use detent::bpmn::{Documentation, SequenceFlow, StartEvent, Task};
use detent::mdx::MdxFile;

#[test]
fn test_parse_mdx_file() {
    let content = r#"---
type: bpmn:startEvent
id: start_1
outgoing:
  - flow_1
---

# Start Event

This is the start of the process.
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    assert!(mdx.frontmatter.contains("id: start_1"));
    assert!(mdx.body.contains("Start Event"));
}

#[test]
fn test_parse_start_event() {
    let content = r#"---
type: bpmn:startEvent
id: start_1
name: Start
outgoing:
  - flow_1
---

# Start Event
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let event = mdx.parse_start_event().expect("Failed to parse StartEvent");

    assert_eq!(event.id, "start_1");
    assert_eq!(event.name, Some("Start".to_string()));
    assert_eq!(event.outgoing, vec!["flow_1"]);
}

#[test]
fn test_parse_end_event() {
    let content = r#"---
type: bpmn:endEvent
id: end_1
name: End
incoming:
  - flow_2
---

# End Event
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let event = mdx.parse_end_event().expect("Failed to parse EndEvent");

    assert_eq!(event.id, "end_1");
    assert_eq!(event.name, Some("End".to_string()));
    assert_eq!(event.incoming, vec!["flow_2"]);
}

#[test]
fn test_parse_task() {
    let content = r#"---
type: bpmn:task
id: task_1
name: Process Data
incoming:
  - flow_1
outgoing:
  - flow_2
---

# Task
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let task = mdx.parse_task().expect("Failed to parse Task");

    assert_eq!(task.id, "task_1");
    assert_eq!(task.name, Some("Process Data".to_string()));
    assert_eq!(task.incoming, vec!["flow_1"]);
    assert_eq!(task.outgoing, vec!["flow_2"]);
}

#[test]
fn test_parse_service_task() {
    let content = r#"---
type: bpmn:serviceTask
id: service_1
name: Call API
implementation: '##WebService'
incoming:
  - flow_1
outgoing:
  - flow_2
---

# Service Task
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let task = mdx.parse_service_task().expect("Failed to parse ServiceTask");

    assert_eq!(task.id, "service_1");
    assert_eq!(task.name, Some("Call API".to_string()));
    assert_eq!(task.implementation, Some("##WebService".to_string()));
}

#[test]
fn test_parse_script_task() {
    let content = r#"---
type: bpmn:scriptTask
id: script_1
name: Run Script
scriptFormat: javascript
script: console.log('hello')
incoming:
  - flow_1
outgoing:
  - flow_2
---

# Script Task
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let task = mdx.parse_script_task().expect("Failed to parse ScriptTask");

    assert_eq!(task.id, "script_1");
    assert_eq!(task.name, Some("Run Script".to_string()));
    assert_eq!(task.script_format, Some("javascript".to_string()));
    assert_eq!(task.script, Some("console.log('hello')".to_string()));
}

#[test]
fn test_parse_sequence_flow() {
    let content = r#"---
type: bpmn:sequenceFlow
id: flow_1
name: To Task
sourceRef: start_1
targetRef: task_1
---

# Sequence Flow
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let flow = mdx
        .parse_sequence_flow()
        .expect("Failed to parse SequenceFlow");

    assert_eq!(flow.id, "flow_1");
    assert_eq!(flow.name, Some("To Task".to_string()));
    assert_eq!(flow.source_ref, "start_1");
    assert_eq!(flow.target_ref, "task_1");
}

#[test]
fn test_parse_exclusive_gateway() {
    let content = r#"---
type: bpmn:exclusiveGateway
id: gw_1
name: Decision
gatewayDirection: Diverging
default: flow_default
incoming:
  - flow_1
outgoing:
  - flow_2
  - flow_3
---

# Exclusive Gateway
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let gateway = mdx
        .parse_exclusive_gateway()
        .expect("Failed to parse ExclusiveGateway");

    assert_eq!(gateway.id, "gw_1");
    assert_eq!(gateway.name, Some("Decision".to_string()));
    assert_eq!(gateway.gateway_direction, Some("Diverging".to_string()));
    assert_eq!(gateway.default, Some("flow_default".to_string()));
    assert_eq!(gateway.outgoing, vec!["flow_2", "flow_3"]);
}

#[test]
fn test_parse_parallel_gateway() {
    let content = r#"---
type: bpmn:parallelGateway
id: gw_2
name: Fork
gatewayDirection: Diverging
incoming:
  - flow_1
outgoing:
  - flow_2
  - flow_3
---

# Parallel Gateway
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let gateway = mdx
        .parse_parallel_gateway()
        .expect("Failed to parse ParallelGateway");

    assert_eq!(gateway.id, "gw_2");
    assert_eq!(gateway.name, Some("Fork".to_string()));
    assert_eq!(gateway.gateway_direction, Some("Diverging".to_string()));
}

#[test]
fn test_roundtrip_start_event() {
    let event = StartEvent {
        id: "start_1".to_string(),
        name: Some("Start".to_string()),
        outgoing: vec!["flow_1".to_string()],
        documentation: None,
    };

    let yaml = serde_yaml::to_string(&event).expect("Failed to serialize");
    let parsed: StartEvent = serde_yaml::from_str(&yaml).expect("Failed to parse");

    assert_eq!(event, parsed);
}

#[test]
fn test_roundtrip_task() {
    let task = Task {
        id: "task_1".to_string(),
        name: Some("Process".to_string()),
        incoming: vec!["flow_1".to_string()],
        outgoing: vec!["flow_2".to_string()],
        documentation: Some(Documentation {
            text: "Task documentation".to_string(),
        }),
    };

    let yaml = serde_yaml::to_string(&task).expect("Failed to serialize");
    let parsed: Task = serde_yaml::from_str(&yaml).expect("Failed to parse");

    assert_eq!(task, parsed);
}

#[test]
fn test_roundtrip_sequence_flow() {
    let flow = SequenceFlow {
        id: "flow_1".to_string(),
        name: Some("To Task".to_string()),
        source_ref: "start_1".to_string(),
        target_ref: "task_1".to_string(),
        condition_expression: None,
        documentation: None,
    };

    let yaml = serde_yaml::to_string(&flow).expect("Failed to serialize");
    let parsed: SequenceFlow = serde_yaml::from_str(&yaml).expect("Failed to parse");

    assert_eq!(flow, parsed);
}

#[test]
fn test_parse_with_documentation() {
    let content = r#"---
type: bpmn:task
id: task_1
name: Documented Task
incoming:
  - flow_1
outgoing:
  - flow_2
documentation:
  text: This is the task documentation.
---

# Task with Documentation
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let task = mdx.parse_task().expect("Failed to parse Task");

    assert_eq!(task.id, "task_1");
    assert!(task.documentation.is_some());
    assert_eq!(
        task.documentation.unwrap().text,
        "This is the task documentation."
    );
}
