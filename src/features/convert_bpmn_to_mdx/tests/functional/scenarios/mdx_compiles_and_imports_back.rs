use cucumber::when;

use super::ConvertWorld;

#[when("I import the compiled definitions to MDX")]
fn when_import_compiled(world: &mut ConvertWorld) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let outputs = detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx(defs)
        .expect("import_to_mdx failed");
    world.import_outputs = Some(outputs);
}
