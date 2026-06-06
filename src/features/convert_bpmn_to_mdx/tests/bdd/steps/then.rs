use cucumber::then;

use super::super::ConvertWorld;

#[then("compilation fails")]
fn then_compile_fails(world: &mut ConvertWorld) {
    assert!(
        world.compile_failed,
        "expected compilation to fail but it produced: {:?}",
        world.compile_result.as_ref().map(|d| &d.id)
    );
}

#[then("import fails")]
fn then_import_fails(world: &mut ConvertWorld) {
    assert!(
        world.import_failed,
        "expected import to fail"
    );
}

#[then(regex = r#"^one output filename is "([^"]+)"$"#)]
fn then_output_filename(world: &mut ConvertWorld, expected: String) {
    let outputs = world.import_outputs.as_ref().expect("expected import outputs");
    assert!(
        outputs.iter().any(|o| o.filename == expected),
        "no output with filename {:?}",
        expected
    );
}

#[then(regex = r"^importing the compiled definitions produces (\d+) MDX outputs$")]
fn then_roundtrip(world: &mut ConvertWorld, n: usize) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let outputs = detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx(defs)
        .expect("import_to_mdx failed");
    assert_eq!(outputs.len(), n);
}
