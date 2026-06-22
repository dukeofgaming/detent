use cucumber::given;

use super::GraphValidationWorld;

#[given("a well-formed linear process")]
fn given_linear(world: &mut GraphValidationWorld) {
    world.workflow = Some(super::linear_process());
}
