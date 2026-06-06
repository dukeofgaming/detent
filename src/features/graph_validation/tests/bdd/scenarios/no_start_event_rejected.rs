use cucumber::given;

use super::GraphValidationWorld;

#[given("a linear process with no start event")]
fn given_no_start(world: &mut GraphValidationWorld) {
    let mut p = super::linear_process();
    p.start_events.clear();
    world.process = Some(p);
}
