use detent::features::graph_validation::domain::bpmn::{
    EndEvent, Process, SequenceFlow, StartEvent, Task,
};
use detent::features::graph_validation::domain::graph::Graph;

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

fn process_with_orphan() -> Process {
    let mut p = linear_process();
    p.tasks.push(Task {
        id: "orphan_1".to_string(),
        name: Some("Orphan".to_string()),
        incoming: vec![],
        outgoing: vec![],
        documentation: None,
    });
    p
}

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
