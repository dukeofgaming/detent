use cucumber::then;

use super::ConvertWorld;

#[then(regex = r"^importing the compiled definitions produces (\d+) MDX outputs$")]
fn then_roundtrip(world: &mut ConvertWorld, n: usize) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let outputs = detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx(defs)
        .expect("import_to_mdx failed");
    assert_eq!(outputs.len(), n);
}
