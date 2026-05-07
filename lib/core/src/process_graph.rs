use std::collections::HashMap;

use crate::bpmn::{FlowNode, Process, SequenceFlow};

/// A read-only graph view over a Process.
///
/// Indexes nodes and edges on construction for efficient lookups.
/// This is the core domain object for structural validation and traversal.
pub struct Graph<'a> {
    process: &'a Process,
    nodes: HashMap<String, FlowNode>,
    outgoing_edges: HashMap<&'a str, Vec<&'a SequenceFlow>>,
    incoming_edges: HashMap<&'a str, Vec<&'a SequenceFlow>>,
}

impl<'a> Graph<'a> {
    pub fn new(process: &'a Process) -> Self {
        let flow_elements = process.flow_elements();

        let mut nodes = HashMap::new();
        for node in flow_elements.nodes() {
            nodes.insert(node.id().to_string(), node);
        }

        let mut outgoing_edges: HashMap<&str, Vec<&SequenceFlow>> = HashMap::new();
        let mut incoming_edges: HashMap<&str, Vec<&SequenceFlow>> = HashMap::new();

        for flow in &process.sequence_flows {
            outgoing_edges
                .entry(flow.source_ref.as_str())
                .or_default()
                .push(flow);
            incoming_edges
                .entry(flow.target_ref.as_str())
                .or_default()
                .push(flow);
        }

        Graph {
            process,
            nodes,
            outgoing_edges,
            incoming_edges,
        }
    }

    pub fn find_node(&self, id: &str) -> Option<&FlowNode> {
        self.nodes.get(id)
    }

    pub fn successors(&self, node_id: &str) -> impl Iterator<Item = &FlowNode> {
        self.outgoing_edges
            .get(node_id)
            .into_iter()
            .flat_map(|flows| flows.iter())
            .filter_map(|flow| self.nodes.get(flow.target_ref.as_str()))
    }

    pub fn predecessors(&self, node_id: &str) -> impl Iterator<Item = &FlowNode> {
        self.incoming_edges
            .get(node_id)
            .into_iter()
            .flat_map(|flows| flows.iter())
            .filter_map(|flow| self.nodes.get(flow.source_ref.as_str()))
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for flow in &self.process.sequence_flows {
            if !self.nodes.contains_key(flow.source_ref.as_str()) {
                errors.push(format!(
                    "Sequence flow '{}' has dangling sourceRef '{}'",
                    flow.id, flow.source_ref
                ));
            }
            if !self.nodes.contains_key(flow.target_ref.as_str()) {
                errors.push(format!(
                    "Sequence flow '{}' has dangling targetRef '{}'",
                    flow.id, flow.target_ref
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
