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
