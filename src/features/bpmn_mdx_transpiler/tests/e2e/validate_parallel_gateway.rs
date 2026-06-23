use cucumber::given;

use super::super::ConvertWorld;
use super::super::write_temp_mdx;

#[given("a temp parallel gateway MDX file")]
fn given_parallel_gateway(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "gateway_1.mdx",
        "---\ntype: bpmn:parallelGateway\nid: gateway_1\ngatewayDirection: Diverging\nincoming:\n- flow_in\noutgoing:\n- flow_a\n- flow_b\n---\n",
    );
}
