use cucumber::then;

use super::super::ConvertWorld;

#[then(regex = r"^importing the compiled definitions produces (\d+) MDX outputs$")]
fn then_roundtrip(world: &mut ConvertWorld, n: usize) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    // Act
    let outputs = detent::features::convert_bpmn_to_mdx::application::import::import_to_mdx(defs)
        .expect("import_to_mdx failed");
    // Assert
    assert_eq!(outputs.len(), n);
}
