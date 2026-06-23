use cucumber::given;

use super::super::ConvertWorld;
use super::super::write_temp_mdx;

#[given("a temp user task MDX file")]
fn given_user_task(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "user_1.mdx",
        "---\ntype: bpmn:userTask\nid: user_1\nname: Review\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );
}
