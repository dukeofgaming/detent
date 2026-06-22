use cucumber::given;

use super::super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a duplicate id "([^"]+)"$"#)]
fn given_duplicate_id(world: &mut GraphValidationWorld, dup_id: String) {
    let mut w = super::super::linear_process();
    w.nodes.iter_mut().find(|n| n.id == "task_1").unwrap().id = dup_id;
    world.workflow = Some(w);
}
