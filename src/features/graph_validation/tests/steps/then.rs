use cucumber::then;

use super::super::GraphValidationWorld;

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
