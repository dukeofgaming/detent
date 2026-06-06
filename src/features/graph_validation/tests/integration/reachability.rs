use detent::features::graph_validation::domain::graph::Graph;

use super::linear_process;
use super::process_with_orphan;

#[test]
fn test_entry_nodes_returns_start_events() {
    let process = linear_process();
    let graph = Graph::new(&process);
    let entries = graph.entry_nodes();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id(), "start_1");
}

#[test]
fn test_exit_nodes_returns_end_events() {
    let process = linear_process();
    let graph = Graph::new(&process);
    let exits = graph.exit_nodes();
    assert_eq!(exits.len(), 1);
    assert_eq!(exits[0].id(), "end_1");
}

#[test]
fn test_reachable_from_start_reaches_all() {
    let process = linear_process();
    let graph = Graph::new(&process);
    let mut reached = graph.reachable_from("start_1");
    reached.sort();
    assert_eq!(reached, vec!["end_1", "start_1", "task_1"]);
}

#[test]
fn test_reachable_from_task_reaches_end() {
    let process = linear_process();
    let graph = Graph::new(&process);
    let mut reached = graph.reachable_from("task_1");
    reached.sort();
    assert_eq!(reached, vec!["end_1", "task_1"]);
}

#[test]
fn test_reachable_from_orphan_is_self_only() {
    let process = process_with_orphan();
    let graph = Graph::new(&process);
    let reached = graph.reachable_from("orphan_1");
    assert_eq!(reached, vec!["orphan_1"]);
}

#[test]
fn test_validate_detects_unreachable_node() {
    let process = process_with_orphan();
    let errors = Graph::new(&process).validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("unreachable") && e.contains("orphan_1")));
}

#[test]
fn test_validate_detects_dead_end_node() {
    let mut process = linear_process();
    process.sequence_flows[1].target_ref = "nowhere".to_string();

    let errors = Graph::new(&process).validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("dead end") && e.contains("task_1")));
}
