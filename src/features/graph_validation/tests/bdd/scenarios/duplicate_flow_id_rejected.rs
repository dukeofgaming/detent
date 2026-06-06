use cucumber::given;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a duplicate flow id "([^"]+)"$"#)]
fn given_duplicate_flow_id(world: &mut GraphValidationWorld, dup_id: String) {
    let mut p = super::linear_process();
    p.sequence_flows[1].id = dup_id;
    world.process = Some(p);
}
