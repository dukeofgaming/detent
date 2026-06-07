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
fn test_successors_of_nonexistent_node_is_empty() {
    let process = linear_process();
    let graph = Graph::new(&process);

    let succs: Vec<_> = graph.successors("nonexistent").collect();
    assert!(succs.is_empty());
}
