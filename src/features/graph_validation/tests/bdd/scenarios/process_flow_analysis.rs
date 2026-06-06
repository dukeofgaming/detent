use std::collections::HashMap;

use cucumber::{then, when};
use detent::features::graph_validation::domain::graph::Graph;

use super::{FlowAnalysis, GraphValidationWorld};

#[when("I analyze the process flow")]
fn when_analyze(world: &mut GraphValidationWorld) {
    let process = world.process.as_ref().expect("process must be set");
    let graph = Graph::new(process);

    let node_ids: Vec<String> = process
        .flow_elements()
        .nodes()
        .map(|n| n.id().to_string())
        .collect();

    let mut successors = HashMap::new();
    let mut predecessors = HashMap::new();
    let mut reachable = HashMap::new();
    for id in &node_ids {
        successors.insert(
            id.clone(),
            graph.successors(id).map(|n| n.id().to_string()).collect(),
        );
        predecessors.insert(
            id.clone(),
            graph.predecessors(id).map(|n| n.id().to_string()).collect(),
        );
        reachable.insert(id.clone(), graph.reachable_from(id));
    }

    let entry_nodes = graph
        .entry_nodes()
        .iter()
        .map(|n| n.id().to_string())
        .collect();
    let exit_nodes = graph
        .exit_nodes()
        .iter()
        .map(|n| n.id().to_string())
        .collect();

    world.analysis = Some(FlowAnalysis {
        successors,
        predecessors,
        entry_nodes,
        exit_nodes,
        reachable,
    });
}

fn analysis(world: &GraphValidationWorld) -> &FlowAnalysis {
    world
        .analysis
        .as_ref()
        .expect("process flow not analyzed; missing a 'When I analyze the process flow' step")
}

#[then(regex = r"^(\S+) flows to (\S+)$")]
fn then_flows_to(world: &mut GraphValidationWorld, from: String, to: String) {
    let succs = &analysis(world).successors[&from];
    assert_eq!(succs.len(), 1, "expected 1 successor for {}", from);
    assert_eq!(succs[0], to, "expected successor of {} to be {}", from, to);
}

#[then(regex = r"^(\S+) has no outgoing flows$")]
fn then_no_outgoing(world: &mut GraphValidationWorld, node: String) {
    let succs = &analysis(world).successors[&node];
    assert!(succs.is_empty(), "expected {} to have no outgoing flows", node);
}

#[then(regex = r"^(\S+) has no incoming flows$")]
fn then_no_incoming(world: &mut GraphValidationWorld, node: String) {
    let preds = &analysis(world).predecessors[&node];
    assert!(preds.is_empty(), "expected {} to have no incoming flows", node);
}

#[then(regex = r"^(\S+) comes from (\S+)$")]
fn then_comes_from(world: &mut GraphValidationWorld, node: String, from: String) {
    let preds = &analysis(world).predecessors[&node];
    assert_eq!(preds.len(), 1, "expected 1 predecessor for {}", node);
    assert_eq!(preds[0], from, "expected predecessor of {} to be {}", node, from);
}

#[then("the entry node is start_1")]
fn then_entry_node(world: &mut GraphValidationWorld) {
    let entries = &analysis(world).entry_nodes;
    assert_eq!(entries, &vec!["start_1".to_string()]);
}

#[then("the exit node is end_1")]
fn then_exit_node(world: &mut GraphValidationWorld) {
    let exits = &analysis(world).exit_nodes;
    assert_eq!(exits, &vec!["end_1".to_string()]);
}

#[then(regex = r"^(\S+) can reach (\S+)$")]
fn then_can_reach(world: &mut GraphValidationWorld, from: String, to: String) {
    let reached = &analysis(world).reachable[&from];
    assert!(reached.contains(&to), "{} should be reachable from {}", to, from);
}

#[then(regex = r"^(\S+) can only reach itself$")]
fn then_can_only_reach_self(world: &mut GraphValidationWorld, from: String) {
    let reached = &analysis(world).reachable[&from];
    assert_eq!(reached, &vec![from.clone()], "{} should only reach itself", from);
}
