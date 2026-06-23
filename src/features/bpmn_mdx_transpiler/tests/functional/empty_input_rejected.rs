use cucumber::given;

use super::super::ConvertWorld;

#[given("no MDX inputs")]
fn given_no_inputs(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![];
}
