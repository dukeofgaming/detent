use std::fs;

use cucumber::given;

use super::super::ConvertWorld;

#[given("the hello-world BPMN fixture")]
fn given_hello_world(world: &mut ConvertWorld) {
    let xml = fs::read_to_string(super::super::hello_world_asset_path("hello-world.bpmn2"))
        .expect("Failed to read hello-world.bpmn2");
    world.bpmn_xml = Some(xml);
}

#[given(regex = r"^the (blog-post|tdd) BPMN fixture$")]
fn given_parametrized_fixture(world: &mut ConvertWorld, name: String) {
    let xml = match name.as_str() {
        "blog-post" => include_str!("../../assets/blog_post/blog-post.bpmn2").to_string(),
        "tdd" => include_str!("../../assets/tdd/tdd.bpmn2").to_string(),
        _ => unreachable!(),
    };
    world.bpmn_xml = Some(xml);
}
