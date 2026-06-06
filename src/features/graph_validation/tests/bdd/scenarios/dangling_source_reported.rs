use cucumber::given;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with a dangling source "([^"]+)"$"#)]
fn given_dangling_source(world: &mut GraphValidationWorld, source: String) {
    let mut p = super::linear_process();
    p.sequence_flows[0].source_ref = source;
    world.process = Some(p);
}
