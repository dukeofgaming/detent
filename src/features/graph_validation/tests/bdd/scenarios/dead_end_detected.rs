use cucumber::given;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a dead end "([^"]+)"$"#)]
fn given_dead_end(world: &mut GraphValidationWorld, target: String) {
    world.workflow = Some(super::linear_process_with_retargeted_exit(target));
}
