use cucumber::given;

use super::GraphValidationWorld;

#[given("a linear process with no end event")]
fn given_no_end(world: &mut GraphValidationWorld) {
    let mut w = super::linear_process();
    w.nodes.retain(|n| n.id != "end_1");
    world.workflow = Some(w);
}
