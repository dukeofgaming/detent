use std::collections::{HashMap, HashSet};

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

    /// Returns all entry nodes (start events).
    pub fn entry_nodes(&self) -> Vec<&FlowNode> {
        self.process
            .start_events
            .iter()
            .filter_map(|e| self.nodes.get(&e.id))
            .collect()
    }

    /// Returns all exit nodes (end events).
    pub fn exit_nodes(&self) -> Vec<&FlowNode> {
        self.process
            .end_events
            .iter()
            .filter_map(|e| self.nodes.get(&e.id))
            .collect()
    }

    /// BFS traversal from `node_id`. Returns all reachable node IDs (including the start node).
    pub fn reachable_from(&self, node_id: &str) -> Vec<String> {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut queue: Vec<&str> = vec![node_id];

        while let Some(current) = queue.pop() {
            if !visited.insert(current) {
                continue;
            }
            for succ in self.successors(current) {
                queue.push(succ.id());
            }
        }

        visited.into_iter().map(String::from).collect()
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // — Structural invariants —

        if self.process.start_events.is_empty() {
            errors.push("Process has no start event".to_string());
        }

        if self.process.end_events.is_empty() {
            errors.push("Process has no end event".to_string());
        }

        // — Dangling reference checks —

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

        // — Duplicate ID checks —

        let mut seen_ids: HashSet<String> = HashSet::new();
        for id in self.process.flow_elements().nodes().map(|n| n.id().to_string()) {
            if !seen_ids.insert(id.clone()) {
                errors.push(format!("Duplicate element ID '{}'", id));
            }
        }
        for flow in &self.process.sequence_flows {
            if !seen_ids.insert(flow.id.clone()) {
                errors.push(format!("Duplicate element ID '{}'", flow.id));
            }
        }

        // — Reachability checks —

        let reachable: HashSet<String> = self
            .entry_nodes()
            .iter()
            .flat_map(|entry| self.reachable_from(entry.id()))
            .collect();

        for node in self.nodes.values() {
            if !reachable.contains(node.id()) {
                errors.push(format!(
                    "Node '{}' is unreachable from any start event",
                    node.id()
                ));
            }
        }

        // — Dead-end checks —

        let can_reach_end: HashSet<String> = self
            .exit_nodes()
            .iter()
            .flat_map(|exit| {
                // Reverse BFS: find all nodes that can reach this exit
                let mut visited: HashSet<&str> = HashSet::new();
                let mut queue: Vec<&str> = vec![exit.id()];
                while let Some(current) = queue.pop() {
                    if !visited.insert(current) {
                        continue;
                    }
                    for pred in self.predecessors(current) {
                        queue.push(pred.id());
                    }
                }
                visited.into_iter().map(String::from).collect::<Vec<_>>()
            })
            .collect();

        for node in self.nodes.values() {
            if !can_reach_end.contains(node.id()) {
                errors.push(format!(
                    "Node '{}' is a dead end (cannot reach any end event)",
                    node.id()
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
