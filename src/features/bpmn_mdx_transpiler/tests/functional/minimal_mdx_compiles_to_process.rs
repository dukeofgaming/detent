use cucumber::{given, then};
use detent::features::bpmn_mdx_transpiler::application::compile::MdxInput;

use super::super::ConvertWorld;

#[given("a minimal MDX input set with start, task, end, and two flows")]
fn given_minimal_inputs(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![
        MdxInput {
            filename: "start_1.mdx".to_string(),
            content: "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "end_1.mdx".to_string(),
            content: "---\ntype: bpmn:endEvent\nid: end_1\nincoming:\n- flow_2\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "task_1.mdx".to_string(),
            content:
                "---\ntype: bpmn:task\nid: task_1\nname: Do Something\nincoming:\n- flow_1\noutgoing:\n- flow_2\n---\n"
                    .to_string(),
        },
        MdxInput {
            filename: "flow_1.mdx".to_string(),
            content: "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: task_1\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "flow_2.mdx".to_string(),
            content: "---\ntype: bpmn:sequenceFlow\nid: flow_2\nsourceRef: task_1\ntargetRef: end_1\n---\n"
                .to_string(),
        },
    ];
}

#[then(
    regex = r"^a process is produced with (\d+) start event, (\d+) task, (\d+) end event, and (\d+) sequence flows$"
)]
fn then_process_shape(
    world: &mut ConvertWorld,
    starts: usize,
    tasks: usize,
    ends: usize,
    flows: usize,
) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let process = defs.process.as_ref().expect("expected a process");
    assert_eq!(process.start_events.len(), starts, "start_events");
    assert_eq!(process.tasks.len(), tasks, "tasks");
    assert_eq!(process.end_events.len(), ends, "end_events");
    assert_eq!(process.sequence_flows.len(), flows, "sequence_flows");
}
