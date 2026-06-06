use cucumber::{then, when};

use super::ConvertWorld;

#[when("I compile the MDX inputs to definitions")]
fn when_compile(world: &mut ConvertWorld) {
    let defs = detent::features::convert_bpmn_to_mdx::use_cases::compile::compile_to_definitions(
        &world.mdx_inputs,
    );
    match defs {
        Ok(defs) => world.compile_result = Some(defs),
        Err(_) => world.compile_failed = true,
    }
}

#[when("I attempt to compile the MDX inputs to definitions")]
fn when_attempt_compile(world: &mut ConvertWorld) {
    let defs = detent::features::convert_bpmn_to_mdx::use_cases::compile::compile_to_definitions(
        &world.mdx_inputs,
    );
    match defs {
        Ok(defs) => world.compile_result = Some(defs),
        Err(_) => world.compile_failed = true,
    }
}

#[when("I attempt to import the BPMN to MDX")]
fn when_attempt_import(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    match detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(xml) {
        Ok(defs) => {
            match detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx(&defs) {
                Ok(outputs) => world.import_outputs = Some(outputs),
                Err(_) => world.import_failed = true,
            }
        }
        Err(_) => world.import_failed = true,
    }
}

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

#[then(regex = r"^importing the compiled definitions produces (\d+) MDX outputs$")]
fn then_roundtrip(world: &mut ConvertWorld, n: usize) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let outputs = detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx(defs)
        .expect("import_to_mdx failed");
    assert_eq!(outputs.len(), n);
}
