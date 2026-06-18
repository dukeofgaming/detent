use crate::features::graph_validation::domain::workflow::{Flow, Node, NodeType, Workflow};
use crate::features::convert_bpmn_to_mdx::adapters::bpmn::Process;

pub fn to_workflow(p: &Process) -> Workflow {
    let mut nodes = Vec::new();
    for e in &p.start_events {
        nodes.push(Node {
            id: e.id.clone(),
            node_type: NodeType::Start,
        });
    }
    for e in &p.end_events {
        nodes.push(Node {
            id: e.id.clone(),
            node_type: NodeType::End,
        });
    }
    for t in &p.tasks {
        nodes.push(Node {
            id: t.id.clone(),
            node_type: NodeType::Action,
        });
    }
    for t in &p.manual_tasks {
        nodes.push(Node {
            id: t.id.clone(),
            node_type: NodeType::Action,
        });
    }
    for t in &p.user_tasks {
        nodes.push(Node {
            id: t.id.clone(),
            node_type: NodeType::Action,
        });
    }
    for t in &p.service_tasks {
        nodes.push(Node {
            id: t.id.clone(),
            node_type: NodeType::Action,
        });
    }
    for t in &p.script_tasks {
        nodes.push(Node {
            id: t.id.clone(),
            node_type: NodeType::Action,
        });
    }
    for g in &p.exclusive_gateways {
        nodes.push(Node {
            id: g.id.clone(),
            node_type: NodeType::Gateway,
        });
    }
    for g in &p.parallel_gateways {
        nodes.push(Node {
            id: g.id.clone(),
            node_type: NodeType::Gateway,
        });
    }

    let flows: Vec<Flow> = p
        .sequence_flows
        .iter()
        .map(|f| Flow {
            id: f.id.clone(),
            source: f.source_ref.clone(),
            target: f.target_ref.clone(),
        })
        .collect();

    Workflow {
        id: p.id.clone(),
        nodes,
        flows,
    }
}
