use cucumber::given;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a dangling source "([^"]+)"$"#)]
fn given_dangling_source(world: &mut GraphValidationWorld, source: String) {
    let mut w = super::linear_process();
    w.flows[0].source = source;
    world.workflow = Some(w);
}
