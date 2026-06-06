use cucumber::given;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a duplicate id "([^"]+)"$"#)]
fn given_duplicate_id(world: &mut GraphValidationWorld, dup_id: String) {
    let mut p = super::linear_process();
    p.tasks[0].id = dup_id;
    world.process = Some(p);
}
