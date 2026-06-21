use cucumber::then;

use super::super::{analysis_of, GraphValidationWorld};

fn analysis(world: &GraphValidationWorld) -> &super::super::FlowAnalysis {
    analysis_of(world)
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
    assert!(
        reached.contains(&to),
        "{} should be reachable from {}",
        to,
        from
    );
}
