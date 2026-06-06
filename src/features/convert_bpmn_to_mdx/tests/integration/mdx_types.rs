use detent::features::convert_bpmn_to_mdx::adapters::bpmn::{SequenceFlow, StartEvent, Task};
use detent::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;
use std::fs;

use super::hello_world_asset_path;

#[test]
fn test_parse_mdx_file() {
    let content = fs::read_to_string(hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx"))
        .expect("Failed to read MDX");
    let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");
    assert!(mdx.frontmatter.contains("id: _1E892844-423C-464F-ADC4-22F1EC73851B"));
    assert!(mdx.body.contains("Start Event"));
}

#[test]
fn test_parse_start_event() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")).unwrap(),
    )
    .unwrap();
    let event = mdx.parse_start_event().unwrap();
    assert_eq!(event.id, "_1E892844-423C-464F-ADC4-22F1EC73851B");
}

#[test]
fn test_parse_end_event() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx")).unwrap(),
    )
    .unwrap();
    let event = mdx.parse_end_event().unwrap();
    assert_eq!(event.id, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
}

#[test]
fn test_parse_task() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")).unwrap(),
    )
    .unwrap();
    let task = mdx.parse_task().unwrap();
    assert_eq!(task.id, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(task.name, Some("Hello World".to_string()));
}

#[test]
fn test_parse_task_with_documentation() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")).unwrap(),
    )
    .unwrap();
    let task = mdx.parse_task().unwrap();
    assert!(task.documentation.is_some());
    assert_eq!(task.documentation.unwrap().text, "T");
}

#[test]
fn test_parse_sequence_flow_start_to_task() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_4083739B-66F0-4B92-A348-A37DF3B29083.mdx")).unwrap(),
    )
    .unwrap();
    let flow = mdx.parse_sequence_flow().unwrap();
    assert_eq!(flow.source_ref, "_1E892844-423C-464F-ADC4-22F1EC73851B");
    assert_eq!(flow.target_ref, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
}

#[test]
fn test_parse_sequence_flow_task_to_end() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx")).unwrap(),
    )
    .unwrap();
    let flow = mdx.parse_sequence_flow().unwrap();
    assert_eq!(flow.source_ref, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(flow.target_ref, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
}

#[test]
fn test_roundtrip_start_event() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")).unwrap(),
    )
    .unwrap();
    let event = mdx.parse_start_event().unwrap();
    let yaml = serde_yaml::to_string(&event).unwrap();
    let parsed: StartEvent = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(event, parsed);
}

#[test]
fn test_roundtrip_task() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")).unwrap(),
    )
    .unwrap();
    let task = mdx.parse_task().unwrap();
    let yaml = serde_yaml::to_string(&task).unwrap();
    let parsed: Task = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(task, parsed);
}

#[test]
fn test_roundtrip_sequence_flow() {
    let mdx = MdxFile::parse(
        &fs::read_to_string(hello_world_asset_path("_4083739B-66F0-4B92-A348-A37DF3B29083.mdx")).unwrap(),
    )
    .unwrap();
    let flow = mdx.parse_sequence_flow().unwrap();
    let yaml = serde_yaml::to_string(&flow).unwrap();
    let parsed: SequenceFlow = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(flow, parsed);
}

#[test]
fn test_all_mdx_files_parseable() {
    for path in [
        hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx"),
        hello_world_asset_path("_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx"),
        hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx"),
        hello_world_asset_path("_4083739B-66F0-4B92-A348-A37DF3B29083.mdx"),
        hello_world_asset_path("_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx"),
    ] {
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read {}", path.display()));
        let mdx = MdxFile::parse(&content)
            .unwrap_or_else(|_| panic!("Failed to parse {}", path.display()));
        assert!(mdx.frontmatter.contains("type:"));
        assert!(mdx.frontmatter.contains("id:"));
    }
}
