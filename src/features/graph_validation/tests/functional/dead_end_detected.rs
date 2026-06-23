use cucumber::given;

use super::super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a dead end "([^"]+)"$"#)]
fn given_dead_end(world: &mut GraphValidationWorld, target: String) {
    world.workflow = Some(super::super::linear_process_with_retargeted_exit(target));
}
