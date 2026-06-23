use std::fs;

use cucumber::given;

use super::super::ConvertWorld;

// Used by: functional (roundtrip outline), integration (all fixture scenarios)
#[given(regex = r"^the (hello-world|blog-post|tdd) BPMN fixture$")]
fn given_parametrized_fixture(world: &mut ConvertWorld, name: String) {
    let xml = match name.as_str() {
        "hello-world" => {
            fs::read_to_string(super::super::hello_world_asset_path("hello-world.bpmn2"))
                .expect("Failed to read hello-world.bpmn2")
        }
        "blog-post" => {
            fs::read_to_string(super::super::fixture_path("blog_post/blog-post.bpmn2"))
                .expect("Failed to read blog-post.bpmn2")
        }
        "tdd" => {
            fs::read_to_string(super::super::fixture_path("tdd/tdd.bpmn2"))
                .expect("Failed to read tdd.bpmn2")
        }
        _ => unreachable!(),
    };
    world.bpmn_xml = Some(xml);
}
