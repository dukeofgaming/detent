use cucumber::given;

use super::super::ConvertWorld;
use super::super::write_temp_mdx;

#[given("a temp gateway MDX file without id")]
fn given_gateway_without_id(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "gateway_bad.mdx",
        "---\ntype: bpmn:exclusiveGateway\nid: \"\"\ngatewayDirection: Diverging\n---\n",
    );
}
