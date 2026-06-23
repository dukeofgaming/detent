use cucumber::{given, then};
use detent::features::bpmn_mdx_transpiler::application::compile::MdxInput;

use super::super::ConvertWorld;

#[given("an MDX input set where a sequence flow targets a non-existent node")]
fn given_dangling_flow(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![
        MdxInput {
            filename: "start_1.mdx".to_string(),
            content: "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "flow_1.mdx".to_string(),
            content: "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: ghost_task\n---\n"
                .to_string(),
        },
    ];
}

#[then("compilation succeeds")]
fn then_compile_succeeds(world: &mut ConvertWorld) {
    assert!(
        !world.compile_failed && world.compile_result.is_some(),
        "expected compilation to succeed"
    );
}

#[then("the resulting process contains the dangling target id")]
fn then_dangling_preserved(world: &mut ConvertWorld) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let process = defs.process.as_ref().expect("expected a process");
    assert!(
        process
            .sequence_flows
            .iter()
            .any(|f| f.target_ref == "ghost_task"),
        "ghost_task target ref was not preserved"
    );
}
