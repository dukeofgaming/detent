use std::collections::{HashMap, HashSet};

use super::workflow::{Flow, Node, NodeType, Workflow};

pub struct Graph<'a> {
    workflow: &'a Workflow,
    node_by_id: HashMap<&'a str, &'a Node>,
    outgoing_by_source: HashMap<&'a str, Vec<&'a Flow>>,
    incoming_by_target: HashMap<&'a str, Vec<&'a Flow>>,
}

impl<'a> Graph<'a> {
    pub fn new(workflow: &'a Workflow) -> Self {
        let node_by_id: HashMap<&str, &Node> = workflow
            .nodes
            .iter()
            .map(|n| (n.id.as_str(), n))
            .collect();

        let mut outgoing_by_source: HashMap<&str, Vec<&Flow>> = HashMap::new();
        let mut incoming_by_target: HashMap<&str, Vec<&Flow>> = HashMap::new();

        for flow in &workflow.flows {
            outgoing_by_source
                .entry(flow.source.as_str())
                .or_default()
                .push(flow);
            incoming_by_target
                .entry(flow.target.as_str())
                .or_default()
                .push(flow);
        }

        Self {
            workflow,
            node_by_id,
            outgoing_by_source,
            incoming_by_target,
        }
    }

    pub fn find_node(&self, id: &str) -> Option<&Node> {
        self.node_by_id.get(id).copied()
    }

    pub fn successors(&self, node_id: &str) -> impl Iterator<Item = &Node> + '_ {
        self.outgoing_by_source
            .get(node_id)
            .into_iter()
            .flat_map(|flows| flows.iter())
            .filter_map(|flow| self.node_by_id.get(flow.target.as_str()))
            .copied()
    }

    pub fn predecessors(&self, node_id: &str) -> impl Iterator<Item = &Node> + '_ {
        self.incoming_by_target
            .get(node_id)
            .into_iter()
            .flat_map(|flows| flows.iter())
            .filter_map(|flow| self.node_by_id.get(flow.source.as_str()))
            .copied()
    }

    pub fn entry_nodes(&self) -> Vec<&Node> {
        self.workflow
            .nodes
            .iter()
            .filter(|n| n.node_type == NodeType::Start)
            .collect()
    }

    pub fn exit_nodes(&self) -> Vec<&Node> {
        self.workflow
            .nodes
            .iter()
            .filter(|n| n.node_type == NodeType::End)
            .collect()
    }

    pub fn reachable_from(&self, node_id: &str) -> Vec<String> {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut queue: Vec<&str> = vec![node_id];

        while let Some(current) = queue.pop() {
            if !visited.insert(current) {
                continue;
            }
            for succ in self.successors(current) {
                queue.push(&succ.id);
            }
        }

        visited.into_iter().map(String::from).collect()
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if !self.workflow.nodes.iter().any(|n| n.node_type == NodeType::Start) {
            errors.push("Process has no start event".to_string());
        }
        if !self.workflow.nodes.iter().any(|n| n.node_type == NodeType::End) {
            errors.push("Process has no end event".to_string());
        }

        for flow in &self.workflow.flows {
            if !self.node_by_id.contains_key(flow.source.as_str()) {
                errors.push(format!(
                    "Sequence flow '{}' has dangling sourceRef '{}'",
                    flow.id, flow.source
                ));
            }
            if !self.node_by_id.contains_key(flow.target.as_str()) {
                errors.push(format!(
                    "Sequence flow '{}' has dangling targetRef '{}'",
                    flow.id, flow.target
                ));
            }
        }

        let mut seen_ids: HashSet<&str> = HashSet::new();
        for node in &self.workflow.nodes {
            if !seen_ids.insert(node.id.as_str()) {
                errors.push(format!("Duplicate element ID '{}'", node.id));
            }
        }
        for flow in &self.workflow.flows {
            if !seen_ids.insert(flow.id.as_str()) {
                errors.push(format!("Duplicate element ID '{}'", flow.id));
            }
        }

        let reachable: HashSet<String> = self
            .entry_nodes()
            .iter()
            .flat_map(|entry| self.reachable_from(&entry.id))
            .collect();

        for node in self.node_by_id.values() {
            if !reachable.contains(node.id.as_str()) {
                errors.push(format!(
                    "Node '{}' is unreachable from any start event",
                    node.id
                ));
            }
        }

        let can_reach_end: HashSet<String> = self
            .exit_nodes()
            .iter()
            .flat_map(|exit| {
                let mut visited: HashSet<&str> = HashSet::new();
                let mut queue: Vec<&str> = vec![&exit.id];
                while let Some(current) = queue.pop() {
                    if !visited.insert(current) {
                        continue;
                    }
                    for pred in self.predecessors(current) {
                        queue.push(&pred.id);
                    }
                }
                visited.into_iter().map(String::from).collect::<Vec<_>>()
            })
            .collect();

        for node in self.node_by_id.values() {
            if !can_reach_end.contains(node.id.as_str()) {
                errors.push(format!(
                    "Node '{}' is a dead end (cannot reach any end event)",
                    node.id
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
