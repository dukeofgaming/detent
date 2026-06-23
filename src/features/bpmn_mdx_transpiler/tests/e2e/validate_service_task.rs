use cucumber::given;

use super::super::ConvertWorld;
use super::super::write_temp_mdx;

#[given("a temp service task MDX file")]
fn given_service_task(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "svc_1.mdx",
        "---\ntype: bpmn:serviceTask\nid: svc_1\nname: Call Service\nimplementation: Java\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );
}
