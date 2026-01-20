//! Unit tests for MDX types (using BPMN types directly for frontmatter)
//!
//! These tests use the real MDX files from tests/assets to validate parsing.

use detent::bpmn::{SequenceFlow, StartEvent, Task};
use detent::mdx::MdxFile;
use std::fs;

const MDX_START_EVENT: &str = "tests/assets/processes/hello-world/_1E892844-423C-464F-ADC4-22F1EC73851B.mdx";
const MDX_END_EVENT: &str = "tests/assets/processes/hello-world/_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx";
const MDX_TASK: &str = "tests/assets/processes/hello-world/_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx";
const MDX_SEQUENCE_FLOW_1: &str = "tests/assets/processes/hello-world/_4083739B-66F0-4B92-A348-A37DF3B29083.mdx";
const MDX_SEQUENCE_FLOW_2: &str = "tests/assets/processes/hello-world/_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx";

#[test]
fn test_parse_mdx_file() {
    // Arrange
    let content = fs::read_to_string(MDX_START_EVENT).expect("Failed to read MDX file");

    // Act
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");

    // Assert
    assert!(mdx.frontmatter.contains("id: _1E892844-423C-464F-ADC4-22F1EC73851B"));
    assert!(mdx.body.contains("Start Event"));
}

#[test]
fn test_parse_start_event() {
    // Arrange
    let content = fs::read_to_string(MDX_START_EVENT).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");

    // Act
    let event = mdx.parse_start_event().expect("Failed to parse StartEvent");

    // Assert
    assert_eq!(event.id, "_1E892844-423C-464F-ADC4-22F1EC73851B");
    assert_eq!(event.outgoing, vec!["_4083739B-66F0-4B92-A348-A37DF3B29083"]);
}

#[test]
fn test_parse_end_event() {
    // Arrange
    let content = fs::read_to_string(MDX_END_EVENT).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");

    // Act
    let event = mdx.parse_end_event().expect("Failed to parse EndEvent");

    // Assert
    assert_eq!(event.id, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
    assert_eq!(event.incoming, vec!["_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB"]);
}

#[test]
fn test_parse_task() {
    // Arrange
    let content = fs::read_to_string(MDX_TASK).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");

    // Act
    let task = mdx.parse_task().expect("Failed to parse Task");

    // Assert
    assert_eq!(task.id, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(task.name, Some("Hello World".to_string()));
    assert_eq!(task.incoming, vec!["_4083739B-66F0-4B92-A348-A37DF3B29083"]);
    assert_eq!(task.outgoing, vec!["_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB"]);
}

#[test]
fn test_parse_task_with_documentation() {
    // Arrange
    let content = fs::read_to_string(MDX_TASK).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");

    // Act
    let task = mdx.parse_task().expect("Failed to parse Task");

    // Assert
    assert!(task.documentation.is_some());
    assert_eq!(task.documentation.unwrap().text, "T");
}

#[test]
fn test_parse_sequence_flow_start_to_task() {
    // Arrange
    let content = fs::read_to_string(MDX_SEQUENCE_FLOW_1).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");

    // Act
    let flow = mdx.parse_sequence_flow().expect("Failed to parse SequenceFlow");

    // Assert
    assert_eq!(flow.id, "_4083739B-66F0-4B92-A348-A37DF3B29083");
    assert_eq!(flow.source_ref, "_1E892844-423C-464F-ADC4-22F1EC73851B");
    assert_eq!(flow.target_ref, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
}

#[test]
fn test_parse_sequence_flow_task_to_end() {
    // Arrange
    let content = fs::read_to_string(MDX_SEQUENCE_FLOW_2).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");

    // Act
    let flow = mdx.parse_sequence_flow().expect("Failed to parse SequenceFlow");

    // Assert
    assert_eq!(flow.id, "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB");
    assert_eq!(flow.source_ref, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(flow.target_ref, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
}

#[test]
fn test_roundtrip_start_event() {
    // Arrange
    let content = fs::read_to_string(MDX_START_EVENT).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");
    let event = mdx.parse_start_event().expect("Failed to parse StartEvent");

    // Act
    let yaml = serde_yaml::to_string(&event).expect("Failed to serialize");
    let parsed: StartEvent = serde_yaml::from_str(&yaml).expect("Failed to parse");

    // Assert
    assert_eq!(event, parsed);
}

#[test]
fn test_roundtrip_task() {
    // Arrange
    let content = fs::read_to_string(MDX_TASK).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");
    let task = mdx.parse_task().expect("Failed to parse Task");

    // Act
    let yaml = serde_yaml::to_string(&task).expect("Failed to serialize");
    let parsed: Task = serde_yaml::from_str(&yaml).expect("Failed to parse");

    // Assert
    assert_eq!(task, parsed);
}

#[test]
fn test_roundtrip_sequence_flow() {
    // Arrange
    let content = fs::read_to_string(MDX_SEQUENCE_FLOW_1).expect("Failed to read MDX file");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");
    let flow = mdx.parse_sequence_flow().expect("Failed to parse SequenceFlow");

    // Act
    let yaml = serde_yaml::to_string(&flow).expect("Failed to serialize");
    let parsed: SequenceFlow = serde_yaml::from_str(&yaml).expect("Failed to parse");

    // Assert
    assert_eq!(flow, parsed);
}

#[test]
fn test_all_mdx_files_parseable() {
    // Arrange
    let mdx_files = [
        MDX_START_EVENT,
        MDX_END_EVENT,
        MDX_TASK,
        MDX_SEQUENCE_FLOW_1,
        MDX_SEQUENCE_FLOW_2,
    ];

    // Act & Assert
    for path in mdx_files {
        let content = fs::read_to_string(path).expect(&format!("Failed to read {}", path));
        let mdx = MdxFile::parse(&content).expect(&format!("Failed to parse {}", path));

        assert!(
            mdx.frontmatter.contains("type:"),
            "Missing type in {}",
            path
        );
        assert!(mdx.frontmatter.contains("id:"), "Missing id in {}", path);
    }
}
