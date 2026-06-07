use cucumber::when;

use super::super::GraphValidationWorld;

#[when("I validate the process graph")]
fn when_validate(world: &mut GraphValidationWorld) {
    let process = world.process.as_ref().expect("process must be set by a Given step");
    let graph = detent::features::graph_validation::domain::graph::Graph::new(process);
    match graph.validate() {
        Ok(()) => world.succeeded = true,
        Err(errs) => world.errors = Some(errs),
    }
}
