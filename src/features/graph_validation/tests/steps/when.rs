use std::collections::HashMap;

use cucumber::when;
use detent::features::graph_validation::domain::graph::Graph;

use super::super::{FlowAnalysis, GraphValidationWorld};

// Used by: functional, integration
#[when("I validate the process graph")]
fn when_validate(world: &mut GraphValidationWorld) {
    let workflow = world
        .workflow
        .as_ref()
        .expect("workflow must be set by a Given step");
    let graph = Graph::new(workflow);
    match graph.validate() {
        Ok(()) => world.succeeded = true,
        Err(errs) => world.errors = Some(errs),
    }
}

// Used by: functional, integration
#[when("I analyze the process flow")]
fn when_analyze(world: &mut GraphValidationWorld) {
    let workflow = world.workflow.as_ref().expect("workflow must be set");
    let graph = Graph::new(workflow);

    let node_ids: Vec<String> = workflow.nodes.iter().map(|n| n.id.clone()).collect();

    let mut successors = HashMap::new();
    let mut predecessors = HashMap::new();
    let mut reachable = HashMap::new();
    for id in &node_ids {
        successors.insert(
            id.clone(),
            graph.successors(id).map(|n| n.id.clone()).collect(),
        );
        predecessors.insert(
            id.clone(),
            graph.predecessors(id).map(|n| n.id.clone()).collect(),
        );
        reachable.insert(id.clone(), graph.reachable_from(id));
    }

    let entry_nodes = graph
        .entry_nodes()
        .iter()
        .map(|n| n.id.clone())
        .collect();
    let exit_nodes = graph
        .exit_nodes()
        .iter()
        .map(|n| n.id.clone())
        .collect();

    world.analysis = Some(FlowAnalysis {
        successors,
        predecessors,
        entry_nodes,
        exit_nodes,
        reachable,
    });
}
