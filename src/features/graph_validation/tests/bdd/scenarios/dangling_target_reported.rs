use cucumber::given;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a dangling target "([^"]+)"$"#)]
fn given_dangling_target(world: &mut GraphValidationWorld, target: String) {
    world.process = Some(super::linear_process_with_retargeted_exit(target));
}
