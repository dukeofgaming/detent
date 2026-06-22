use cucumber::{given, then};
use detent::features::convert_bpmn_to_mdx::use_cases::compile::MdxInput;

use super::super::ConvertWorld;

#[given("an MDX input set with a service task, a script task, and an exclusive gateway")]
fn given_rich_inputs(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![
        MdxInput {
            filename: "start_1.mdx".to_string(),
            content: "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_in\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "gateway_1.mdx".to_string(),
            content:
                "---\ntype: bpmn:exclusiveGateway\nid: gateway_1\ngatewayDirection: Diverging\nincoming:\n- flow_in\noutgoing:\n- flow_svc\n- flow_script\n---\n"
                    .to_string(),
        },
        MdxInput {
            filename: "svc_1.mdx".to_string(),
            content:
                "---\ntype: bpmn:serviceTask\nid: svc_1\nname: Call Service\nimplementation: Java\nincoming:\n- flow_svc\noutgoing:\n- flow_svc_out\n---\n"
                    .to_string(),
        },
        MdxInput {
            filename: "script_1.mdx".to_string(),
            content:
                "---\ntype: bpmn:scriptTask\nid: script_1\nname: Run Script\nscriptFormat: javascript\nincoming:\n- flow_script\noutgoing:\n- flow_script_out\n---\n"
                    .to_string(),
        },
        MdxInput {
            filename: "end_1.mdx".to_string(),
            content:
                "---\ntype: bpmn:endEvent\nid: end_1\nincoming:\n- flow_svc_out\n- flow_script_out\n---\n"
                    .to_string(),
        },
    ];
}

#[then("the process has 1 service task, 1 script task, and 1 exclusive gateway")]
fn then_rich_shape(world: &mut ConvertWorld) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let process = defs.process.as_ref().expect("expected a process");
    assert_eq!(process.service_tasks.len(), 1, "service_tasks");
    assert_eq!(process.script_tasks.len(), 1, "script_tasks");
    assert_eq!(process.exclusive_gateways.len(), 1, "exclusive_gateways");
    assert_eq!(
        process.service_tasks[0].implementation,
        Some("Java".to_string())
    );
    assert_eq!(
        process.script_tasks[0].script_format,
        Some("javascript".to_string())
    );
    assert_eq!(
        process.exclusive_gateways[0].gateway_direction,
        Some("Diverging".to_string())
    );
}
