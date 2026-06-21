use cucumber::then;

use super::{analysis_of, FlowAnalysis, GraphValidationWorld};

fn analysis(world: &GraphValidationWorld) -> &FlowAnalysis {
    analysis_of(world)
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

#[then(regex = r"^(\S+) can only reach itself$")]
fn then_can_only_reach_self(world: &mut GraphValidationWorld, from: String) {
    let reached = &analysis(world).reachable[&from];
    assert_eq!(reached, &vec![from.clone()], "{} should only reach itself", from);
}
