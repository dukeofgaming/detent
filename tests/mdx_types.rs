//! Unit tests for MDX types

use detent::mdx::{MdxFile, NodeFrontmatter};

#[test]
fn test_parse_mdx_file() {
    let content = r#"---
id: start_1
type: bpmn:startEvent
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
fn test_parse_node_frontmatter() {
    let content = r#"---
id: task_1
type: bpmn:serviceTask
name: Gather Profile
incoming:
  - flow_1
outgoing:
  - flow_2
implementation: service
serviceRef: onboarding.gatherProfile
---

# Task
"#;

    let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
    let node = mdx
        .parse_node_frontmatter()
        .expect("Failed to parse frontmatter");

    assert_eq!(node.id, "task_1");
    assert_eq!(node.node_type, "bpmn:serviceTask");
    assert_eq!(node.name, Some("Gather Profile".to_string()));
    assert_eq!(node.incoming, vec!["flow_1"]);
    assert_eq!(node.outgoing, vec!["flow_2"]);
    assert_eq!(node.implementation, Some("service".to_string()));
    assert_eq!(
        node.service_ref,
        Some("onboarding.gatherProfile".to_string())
    );
}

#[test]
fn test_roundtrip_frontmatter() {
    let node = NodeFrontmatter {
        id: "start_1".to_string(),
        node_type: "bpmn:startEvent".to_string(),
        name: None,
        incoming: vec![],
        outgoing: vec!["flow_1".to_string()],
        documentation: None,
        implementation: None,
        service_ref: None,
        script_format: None,
        script: None,
        gateway_direction: None,
        default: None,
        conditions: None,
        io_spec: None,
        retry: None,
    };

    let yaml = serde_yaml::to_string(&node).expect("Failed to serialize");
    let parsed: NodeFrontmatter = serde_yaml::from_str(&yaml).expect("Failed to parse");

    assert_eq!(node, parsed);
}
