//! Tests for bpmn::graph - Graph operations on the BPMN IR

use detent::bpmn::graph::Graph;
use detent::bpmn::{EndEvent, Process, SequenceFlow, StartEvent, Task};

/// Helper: simple linear process: Start → Task → End
fn linear_process() -> Process {
    Process {
        id: "process_1".to_string(),
        name: Some("Linear Process".to_string()),
        is_executable: Some(true),
        process_type: None,
        documentation: None,
        start_events: vec![StartEvent {
            id: "start_1".to_string(),
            name: None,
            outgoing: vec!["flow_1".to_string()],
            documentation: None,
        }],
        end_events: vec![EndEvent {
            id: "end_1".to_string(),
            name: None,
            incoming: vec!["flow_2".to_string()],
            documentation: None,
        }],
        tasks: vec![Task {
            id: "task_1".to_string(),
            name: Some("Do Something".to_string()),
            incoming: vec!["flow_1".to_string()],
            outgoing: vec!["flow_2".to_string()],
            documentation: None,
        }],
        service_tasks: vec![],
        script_tasks: vec![],
        exclusive_gateways: vec![],
        parallel_gateways: vec![],
        sequence_flows: vec![
            SequenceFlow {
                id: "flow_1".to_string(),
                name: None,
                source_ref: "start_1".to_string(),
                target_ref: "task_1".to_string(),
                condition_expression: None,
                documentation: None,
            },
            SequenceFlow {
                id: "flow_2".to_string(),
                name: None,
                source_ref: "task_1".to_string(),
                target_ref: "end_1".to_string(),
                condition_expression: None,
                documentation: None,
            },
        ],
    }
}

// --- find_node ---

#[test]
fn test_find_node_returns_existing_node() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let node = graph.find_node("task_1");
    assert!(node.is_some());
    assert_eq!(node.unwrap().id(), "task_1");
}

#[test]
fn test_find_node_returns_none_for_missing_id() {
    let process = linear_process();
    let graph = Graph::new(&process);

    assert!(graph.find_node("nonexistent").is_none());
}

// --- successors ---

#[test]
fn test_successors_of_start_event() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let succs: Vec<_> = graph.successors("start_1").collect();
    assert_eq!(succs.len(), 1);
    assert_eq!(succs[0].id(), "task_1");
}

#[test]
fn test_successors_of_task() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let succs: Vec<_> = graph.successors("task_1").collect();
    assert_eq!(succs.len(), 1);
    assert_eq!(succs[0].id(), "end_1");
}

#[test]
fn test_successors_of_end_event_is_empty() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let succs: Vec<_> = graph.successors("end_1").collect();
    assert!(succs.is_empty());
}

#[test]
fn test_successors_of_nonexistent_node_is_empty() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let succs: Vec<_> = graph.successors("nonexistent").collect();
    assert!(succs.is_empty());
}

// --- predecessors ---

#[test]
fn test_predecessors_of_end_event() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let preds: Vec<_> = graph.predecessors("end_1").collect();
    assert_eq!(preds.len(), 1);
    assert_eq!(preds[0].id(), "task_1");
}

#[test]
fn test_predecessors_of_task() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let preds: Vec<_> = graph.predecessors("task_1").collect();
    assert_eq!(preds.len(), 1);
    assert_eq!(preds[0].id(), "start_1");
}

#[test]
fn test_predecessors_of_start_event_is_empty() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let preds: Vec<_> = graph.predecessors("start_1").collect();
    assert!(preds.is_empty());
}

// --- validate ---

#[test]
fn test_validate_valid_process_succeeds() {
    let process = linear_process();
    let graph = Graph::new(&process);

    assert!(graph.validate().is_ok());
}

#[test]
fn test_validate_detects_dangling_source_ref() {
    let mut process = linear_process();
    // Point flow_1's source to a nonexistent node
    process.sequence_flows[0].source_ref = "ghost".to_string();

    let graph = Graph::new(&process);
    let errors = graph.validate().unwrap_err();
    assert!(errors.iter().any(|e: &String| e.contains("ghost")));
}

#[test]
fn test_validate_detects_dangling_target_ref() {
    let mut process = linear_process();
    // Point flow_2's target to a nonexistent node
    process.sequence_flows[1].target_ref = "ghost".to_string();

    let graph = Graph::new(&process);
    let errors = graph.validate().unwrap_err();
    assert!(errors.iter().any(|e: &String| e.contains("ghost")));
}
