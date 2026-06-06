use cucumber::{then, when};
use detent::features::graph_validation::domain::graph::Graph;

use super::GraphValidationWorld;

fn analyze(world: &GraphValidationWorld) -> Graph<'_> {
    let process = world.process.as_ref().expect("process must be set");
    Graph::new(process)
}

#[when("I analyze the process flow")]
fn when_analyze(_world: &mut GraphValidationWorld) {
}

#[then(regex = r"^(\S+) flows to (\S+)$")]
fn then_flows_to(world: &mut GraphValidationWorld, from: String, to: String) {
    let graph = analyze(world);
    let succs: Vec<_> = graph.successors(&from).collect();
    assert_eq!(succs.len(), 1, "expected 1 successor for {}", from);
    assert_eq!(succs[0].id(), to, "expected successor of {} to be {}", from, to);
}

#[then(regex = r"^(\S+) has no outgoing flows$")]
fn then_no_outgoing(world: &mut GraphValidationWorld, node: String) {
    let graph = analyze(world);
    let succs: Vec<_> = graph.successors(&node).collect();
    assert!(succs.is_empty(), "expected {} to have no outgoing flows", node);
}

#[then(regex = r"^(\S+) has no incoming flows$")]
fn then_no_incoming(world: &mut GraphValidationWorld, node: String) {
    let graph = analyze(world);
    let preds: Vec<_> = graph.predecessors(&node).collect();
    assert!(preds.is_empty(), "expected {} to have no incoming flows", node);
}

#[then(regex = r"^(\S+) comes from (\S+)$")]
fn then_comes_from(world: &mut GraphValidationWorld, node: String, from: String) {
    let graph = analyze(world);
    let preds: Vec<_> = graph.predecessors(&node).collect();
    assert_eq!(preds.len(), 1, "expected 1 predecessor for {}", node);
    assert_eq!(preds[0].id(), from, "expected predecessor of {} to be {}", node, from);
}

#[then("the entry node is start_1")]
fn then_entry_node(world: &mut GraphValidationWorld) {
    let graph = analyze(world);
    let entries = graph.entry_nodes();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id(), "start_1");
}

#[then("the exit node is end_1")]
fn then_exit_node(world: &mut GraphValidationWorld) {
    let graph = analyze(world);
    let exits = graph.exit_nodes();
    assert_eq!(exits.len(), 1);
    assert_eq!(exits[0].id(), "end_1");
}

#[then(regex = r"^(\S+) can reach (\S+)$")]
fn then_can_reach(world: &mut GraphValidationWorld, from: String, to: String) {
    let graph = analyze(world);
    let reached = graph.reachable_from(&from);
    assert!(reached.contains(&to), "{} should be reachable from {}", to, from);
}

#[then(regex = r"^(\S+) can only reach itself$")]
fn then_can_only_reach_self(world: &mut GraphValidationWorld, from: String) {
    let graph = analyze(world);
    let reached = graph.reachable_from(&from);
    assert_eq!(reached, vec![from.clone()], "{} should only reach itself", from);
}
