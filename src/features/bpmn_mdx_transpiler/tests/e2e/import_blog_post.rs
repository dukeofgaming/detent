use cucumber::when;

use super::super::ConvertWorld;
use super::super::run_detent;

#[when("I import blog-post BPMN to the temp output directory")]
fn when_import_blog_post(world: &mut ConvertWorld) {
    let bpmn = super::super::fixture_path("blog_post/blog-post.bpmn2")
        .to_string_lossy()
        .into_owned();
    let out = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .to_string_lossy()
        .into_owned();
    run_detent(world, &["import", &bpmn, "--output-directory", &out]);
}
