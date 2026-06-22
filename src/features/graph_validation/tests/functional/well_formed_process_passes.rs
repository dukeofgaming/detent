use cucumber::given;

use super::super::GraphValidationWorld;

#[given("a well-formed linear process")]
fn given_linear(world: &mut GraphValidationWorld) {
    world.workflow = Some(super::super::linear_process());
}
