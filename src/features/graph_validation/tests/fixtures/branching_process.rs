use detent::features::graph_validation::domain::bpmn::{
    EndEvent, ExclusiveGateway, Process, SequenceFlow, StartEvent, Task,
};

/// A branching-and-merging process:
///
/// ```text
/// start_1 -> gateway_1 -+-> task_a -+-> end_1
///                       +-> task_b -+
/// ```
///
/// `gateway_1` has two outgoing flows and `end_1` has two incoming flows, so
/// this exercises multi-successor and multi-predecessor graph behaviour that a
/// strictly linear process cannot.
pub fn branching_process() -> Process {
    Process {
        id: "process_branching".to_string(),
        name: Some("Branching Process".to_string()),
        is_executable: Some(true),
        process_type: None,
        documentation: None,
        start_events: vec![StartEvent {
            id: "start_1".to_string(),
            name: None,
            outgoing: vec!["flow_in".to_string()],
            documentation: None,
        }],
        end_events: vec![EndEvent {
            id: "end_1".to_string(),
            name: None,
            incoming: vec!["flow_a_out".to_string(), "flow_b_out".to_string()],
            documentation: None,
        }],
        tasks: vec![
            Task {
                id: "task_a".to_string(),
                name: Some("Task A".to_string()),
                incoming: vec!["flow_a".to_string()],
                outgoing: vec!["flow_a_out".to_string()],
                documentation: None,
            },
            Task {
                id: "task_b".to_string(),
                name: Some("Task B".to_string()),
                incoming: vec!["flow_b".to_string()],
                outgoing: vec!["flow_b_out".to_string()],
                documentation: None,
            },
        ],
        service_tasks: vec![],
        script_tasks: vec![],
        exclusive_gateways: vec![ExclusiveGateway {
            id: "gateway_1".to_string(),
        }],
        parallel_gateways: vec![],
        sequence_flows: vec![
            SequenceFlow {
                id: "flow_in".to_string(),
                name: None,
                source_ref: "start_1".to_string(),
                target_ref: "gateway_1".to_string(),
                condition_expression: None,
                documentation: None,
            },
            SequenceFlow {
                id: "flow_a".to_string(),
                name: None,
                source_ref: "gateway_1".to_string(),
                target_ref: "task_a".to_string(),
                condition_expression: None,
                documentation: None,
            },
            SequenceFlow {
                id: "flow_b".to_string(),
                name: None,
                source_ref: "gateway_1".to_string(),
                target_ref: "task_b".to_string(),
                condition_expression: None,
                documentation: None,
            },
            SequenceFlow {
                id: "flow_a_out".to_string(),
                name: None,
                source_ref: "task_a".to_string(),
                target_ref: "end_1".to_string(),
                condition_expression: None,
                documentation: None,
            },
            SequenceFlow {
                id: "flow_b_out".to_string(),
                name: None,
                source_ref: "task_b".to_string(),
                target_ref: "end_1".to_string(),
                condition_expression: None,
                documentation: None,
            },
        ],
    }
}
