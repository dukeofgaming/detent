use cucumber::given;

use super::super::ConvertWorld;

#[given("blog-post BPMN file path")]
fn given_blog_post_path(world: &mut ConvertWorld) {
    world.e2e_output_file = Some(super::super::fixture_path("blog_post/blog-post.bpmn2"));
}
