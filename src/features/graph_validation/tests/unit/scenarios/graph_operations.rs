use cucumber::{given, then, when};
use detent::features::graph_validation::domain::graph::Graph;

use super::GraphValidationWorld;

#[given("a linear process graph")]
fn given_linear_graph(world: &mut GraphValidationWorld) {
    world.workflow = Some(super::linear_process());
}

#[when(regex = r#"^I find node "([^"]+)"$"#)]
fn when_find_node(world: &mut GraphValidationWorld, node_id: String) {
    let workflow = world.workflow.as_ref().expect("workflow must be set");
    let graph = Graph::new(workflow);
    world.found_node = graph.find_node(&node_id).map(|n| n.id.clone());
}

#[then(regex = r#"^the node id is "([^"]+)"$"#)]
fn then_node_id(world: &mut GraphValidationWorld, expected: String) {
    assert_eq!(
        world.found_node.as_deref(),
        Some(expected.as_str()),
        "expected node {:?}",
        expected
    );
}

#[then("no node is found")]
fn then_no_node(world: &mut GraphValidationWorld) {
    assert!(world.found_node.is_none(), "expected no node but found {:?}", world.found_node);
}

#[when(regex = r#"^I list successors of "([^"]+)"$"#)]
fn when_successors(world: &mut GraphValidationWorld, node_id: String) {
    let workflow = world.workflow.as_ref().expect("workflow must be set");
    let graph = Graph::new(workflow);
    world.last_successors = graph.successors(&node_id).map(|n| n.id.clone()).collect();
}

#[then("successors are empty")]
fn then_successors_empty(world: &mut GraphValidationWorld) {
    assert!(world.last_successors.is_empty());
}
