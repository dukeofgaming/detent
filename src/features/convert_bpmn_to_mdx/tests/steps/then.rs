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
