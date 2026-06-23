use detent::features::graph_validation::domain::workflow::{Flow, Node, NodeType, Workflow};

pub fn linear_process() -> Workflow {
    Workflow {
        id: "process_1".to_string(),
        nodes: vec![
            Node {
                id: "start_1".to_string(),
                node_type: NodeType::Start,
            },
            Node {
                id: "task_1".to_string(),
                node_type: NodeType::Action,
            },
            Node {
                id: "end_1".to_string(),
                node_type: NodeType::End,
            },
        ],
        flows: vec![
            Flow {
                id: "flow_1".to_string(),
                source: "start_1".to_string(),
                target: "task_1".to_string(),
            },
            Flow {
                id: "flow_2".to_string(),
                source: "task_1".to_string(),
                target: "end_1".to_string(),
            },
        ],
    }
}

#[allow(dead_code)]
pub fn linear_process_with_retargeted_exit(new_target: String) -> Workflow {
    let mut w = linear_process();
    w.flows[1].target = new_target;
    w
}
