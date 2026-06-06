use cucumber::given;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a dangling target "([^"]+)"$"#)]
fn given_dangling_target(world: &mut GraphValidationWorld, target: String) {
    let mut p = super::linear_process();
    p.sequence_flows[1].target_ref = target;
    world.process = Some(p);
}
