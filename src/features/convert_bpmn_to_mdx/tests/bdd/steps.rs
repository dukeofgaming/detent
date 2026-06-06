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

#[then("compilation fails")]
fn then_compile_fails(world: &mut ConvertWorld) {
    assert!(
        world.compile_failed,
        "expected compilation to fail but it produced: {:?}",
        world.compile_result.as_ref().map(|d| &d.id)
    );
}
