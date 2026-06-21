use cucumber::when;

use super::super::GraphValidationWorld;

#[when("I validate the process graph")]
fn when_validate(world: &mut GraphValidationWorld) {
    let workflow = world.workflow.as_ref().expect("workflow must be set by a Given step");
    let graph = detent::features::graph_validation::domain::graph::Graph::new(workflow);
    match graph.validate() {
        Ok(()) => world.succeeded = true,
        Err(errs) => world.errors = Some(errs),
    }
}
