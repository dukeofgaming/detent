use cucumber::given;

use super::super::GraphValidationWorld;

#[given("a linear process with no start event")]
fn given_no_start(world: &mut GraphValidationWorld) {
    let mut w = super::super::linear_process();
    w.nodes.retain(|n| n.id != "start_1");
    world.workflow = Some(w);
}
