//! Graph operations on BPMN Process (the IR)
//!
//! Provides traversal and validation over flow nodes and sequence flows.
//! The Graph borrows a Process and offers lookups, successor/predecessor
//! traversal, and structural validation.

use std::collections::HashMap;

use super::types::{FlowNode, Process, SequenceFlow};

/// A read-only graph view over a BPMN Process.
///
/// Indexes nodes and edges on construction for efficient lookups.
pub struct Graph<'a> {
    process: &'a Process,
    /// node_id → FlowNode (cloned for uniform access)
    nodes: HashMap<String, FlowNode>,
    /// source_id → list of sequence flows from that source
    outgoing_edges: HashMap<&'a str, Vec<&'a SequenceFlow>>,
    /// target_id → list of sequence flows into that target
    incoming_edges: HashMap<&'a str, Vec<&'a SequenceFlow>>,
}

impl<'a> Graph<'a> {
    /// Build a graph index from a Process.
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

    /// Look up a flow node by ID.
    pub fn find_node(&self, id: &str) -> Option<&FlowNode> {
        self.nodes.get(id)
    }

    /// Get successor nodes (nodes this node connects to via outgoing sequence flows).
    pub fn successors(&self, node_id: &str) -> impl Iterator<Item = &FlowNode> {
        self.outgoing_edges
            .get(node_id)
            .into_iter()
            .flat_map(|flows| flows.iter())
            .filter_map(|flow| self.nodes.get(flow.target_ref.as_str()))
    }

    /// Get predecessor nodes (nodes that connect to this node via incoming sequence flows).
    pub fn predecessors(&self, node_id: &str) -> impl Iterator<Item = &FlowNode> {
        self.incoming_edges
            .get(node_id)
            .into_iter()
            .flat_map(|flows| flows.iter())
            .filter_map(|flow| self.nodes.get(flow.source_ref.as_str()))
    }

    /// Validate the graph structure.
    ///
    /// Returns `Ok(())` if valid, or `Err(Vec<String>)` with all validation errors.
    /// Checks:
    /// - All sequence flow sourceRef values point to existing nodes
    /// - All sequence flow targetRef values point to existing nodes
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
