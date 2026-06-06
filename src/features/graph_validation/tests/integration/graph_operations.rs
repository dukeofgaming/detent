use detent::features::graph_validation::domain::graph::Graph;

use super::linear_process;

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
