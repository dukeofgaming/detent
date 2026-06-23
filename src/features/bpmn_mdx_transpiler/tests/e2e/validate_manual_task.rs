use cucumber::given;

use super::super::ConvertWorld;
use super::super::write_temp_mdx;

#[given("a temp manual task MDX file")]
fn given_manual_task(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "manual_1.mdx",
        "---\ntype: bpmn:manualTask\nid: manual_1\nname: Manual Step\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );
}
