use cucumber::given;

use super::GraphValidationWorld;

#[given("a linear process with no end event")]
fn given_no_end(world: &mut GraphValidationWorld) {
    let mut p = super::linear_process();
    p.end_events.clear();
    world.process = Some(p);
}
