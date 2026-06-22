use cucumber::given;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a duplicate flow id "([^"]+)"$"#)]
fn given_duplicate_flow_id(world: &mut GraphValidationWorld, dup_id: String) {
    let mut w = super::linear_process();
    w.flows[1].id = dup_id;
    world.workflow = Some(w);
}
