use cucumber::{then, when};

use super::GraphValidationWorld;

#[when("I validate the process graph")]
fn when_validate(world: &mut GraphValidationWorld) {
    let process = world.process.as_ref().expect("process must be set by a Given step");
    let graph = detent::features::graph_validation::domain::graph::Graph::new(process);
    match graph.validate() {
        Ok(()) => world.succeeded = true,
        Err(errs) => world.errors = Some(errs),
    }
}

#[then("validation succeeds")]
fn then_succeeds(world: &mut GraphValidationWorld) {
    assert!(
        world.succeeded,
        "expected validation to succeed but got errors: {:?}",
        world.errors
    );
}

#[then(regex = r#"^validation fails reporting "([^"]+)"$"#)]
fn then_fails_with(world: &mut GraphValidationWorld, snippet: String) {
    let errs = world
        .errors
        .as_ref()
        .expect("expected validation to fail but it succeeded");
    assert!(
        errs.iter().any(|e| e.contains(&snippet)),
        "no error contained {:?}; errors were: {:?}",
        snippet,
        errs
    );
}
