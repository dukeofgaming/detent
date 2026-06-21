use cucumber::when;

use super::super::ConvertWorld;

// A single compile step backs both the success-path phrasing
// ("I compile...") and the failure-path phrasing ("I attempt to compile...").
#[when("I compile the MDX inputs to definitions")]
#[when("I attempt to compile the MDX inputs to definitions")]
fn when_compile(world: &mut ConvertWorld) {
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
    // Act
    match detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(xml) {
        Ok(defs) => {
            // Act
            match detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx(&defs) {
                Ok(outputs) => world.import_outputs = Some(outputs),
                Err(_) => world.import_failed = true,
            }
        }
        Err(_) => world.import_failed = true,
    }
}

#[when("I import the parsed definitions to MDX")]
fn when_import_parsed(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("parsed_defs must be set");
    let outputs = detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx(defs)
        .expect("import_to_mdx failed");
    world.import_outputs = Some(outputs);
}
