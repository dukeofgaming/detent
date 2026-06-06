use detent::features::graph_validation::domain::graph::Graph;

use super::linear_process;

#[test]
fn test_validate_valid_process_succeeds() {
    let process = linear_process();
    let graph = Graph::new(&process);

    assert!(graph.validate().is_ok());
}

#[test]
fn test_validate_detects_dangling_source_ref() {
    let mut process = linear_process();
    process.sequence_flows[0].source_ref = "ghost".to_string();

    let graph = Graph::new(&process);
    let errors = graph.validate().unwrap_err();
    assert!(errors.iter().any(|e: &String| e.contains("ghost")));
}

#[test]
fn test_validate_detects_dangling_target_ref() {
    let mut process = linear_process();
    process.sequence_flows[1].target_ref = "ghost".to_string();

    let graph = Graph::new(&process);
    let errors = graph.validate().unwrap_err();
    assert!(errors.iter().any(|e: &String| e.contains("ghost")));
}

#[test]
fn test_validate_missing_start_event() {
    let mut process = linear_process();
    process.start_events.clear();

    let graph = Graph::new(&process);
    let errors = graph.validate().unwrap_err();
    assert!(errors.iter().any(|e: &String| e.contains("start event")));
}

#[test]
fn test_validate_missing_end_event() {
    let mut process = linear_process();
    process.end_events.clear();

    let graph = Graph::new(&process);
    let errors = graph.validate().unwrap_err();
    assert!(errors.iter().any(|e: &String| e.contains("end event")));
}

#[test]
fn test_validate_duplicate_node_ids() {
    let mut process = linear_process();
    process.tasks[0].id = "start_1".to_string();

    let graph = Graph::new(&process);
    let errors = graph.validate().unwrap_err();
    assert!(errors.iter().any(|e: &String| e.contains("Duplicate")));
    assert!(errors.iter().any(|e: &String| e.contains("start_1")));
}

#[test]
fn test_validate_duplicate_flow_ids() {
    let mut process = linear_process();
    process.sequence_flows[1].id = "flow_1".to_string();

    let graph = Graph::new(&process);
    let errors = graph.validate().unwrap_err();
    assert!(errors.iter().any(|e: &String| e.contains("Duplicate")));
    assert!(errors.iter().any(|e: &String| e.contains("flow_1")));
}
