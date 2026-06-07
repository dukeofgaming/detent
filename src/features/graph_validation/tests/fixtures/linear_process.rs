use detent::features::graph_validation::domain::bpmn::{
    EndEvent, Process, SequenceFlow, StartEvent, Task,
};

pub fn linear_process() -> Process {
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

/// A linear process whose final flow has been retargeted to `new_target`,
/// detaching `end_1`. Depending on the assertion, this surfaces either a
/// dangling target (the new target does not exist) or a dead end (`task_1`
/// can no longer reach any end event).
///
/// `allow(dead_code)`: this fixture file is shared via `#[path]` by both the
/// BDD and unit test harnesses; this helper is only consumed by the BDD one.
#[allow(dead_code)]
pub fn linear_process_with_retargeted_exit(new_target: String) -> Process {
    let mut process = linear_process();
    process.sequence_flows[1].target_ref = new_target;
    process
}
