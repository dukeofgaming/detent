use std::collections::HashSet;

use cucumber::{given, then};

use super::GraphValidationWorld;

#[given("a branching process with an exclusive gateway")]
fn given_branching(world: &mut GraphValidationWorld) {
    world.process = Some(super::branching_process());
}

#[then(regex = r"^(\S+) branches to (\S+) and (\S+)$")]
fn then_branches_to(world: &mut GraphValidationWorld, from: String, a: String, b: String) {
    let succs: &Vec<String> = &super::analysis_of(world).successors[&from];
    let got: HashSet<&str> = succs.iter().map(String::as_str).collect();
    let want: HashSet<&str> = [a.as_str(), b.as_str()].into_iter().collect();
    assert_eq!(got, want, "{} should branch to exactly {} and {}", from, a, b);
}

#[then(regex = r"^(\S+) merges (\S+) and (\S+)$")]
fn then_merges(world: &mut GraphValidationWorld, node: String, a: String, b: String) {
    let preds: &Vec<String> = &super::analysis_of(world).predecessors[&node];
    let got: HashSet<&str> = preds.iter().map(String::as_str).collect();
    let want: HashSet<&str> = [a.as_str(), b.as_str()].into_iter().collect();
    assert_eq!(got, want, "{} should merge exactly {} and {}", node, a, b);
}
