use detent::features::graph_validation::domain::workflow::{Flow, Node, NodeType, Workflow};

pub fn branching_process() -> Workflow {
    Workflow {
        id: "process_branching".to_string(),
        nodes: vec![
            Node {
                id: "start_1".to_string(),
                node_type: NodeType::Start,
            },
            Node {
                id: "gateway_1".to_string(),
                node_type: NodeType::Gateway,
            },
            Node {
                id: "task_a".to_string(),
                node_type: NodeType::Action,
            },
            Node {
                id: "task_b".to_string(),
                node_type: NodeType::Action,
            },
            Node {
                id: "end_1".to_string(),
                node_type: NodeType::End,
            },
        ],
        flows: vec![
            Flow {
                id: "flow_in".to_string(),
                source: "start_1".to_string(),
                target: "gateway_1".to_string(),
            },
            Flow {
                id: "flow_a".to_string(),
                source: "gateway_1".to_string(),
                target: "task_a".to_string(),
            },
            Flow {
                id: "flow_b".to_string(),
                source: "gateway_1".to_string(),
                target: "task_b".to_string(),
            },
            Flow {
                id: "flow_a_out".to_string(),
                source: "task_a".to_string(),
                target: "end_1".to_string(),
            },
            Flow {
                id: "flow_b_out".to_string(),
                source: "task_b".to_string(),
                target: "end_1".to_string(),
            },
        ],
    }
}
