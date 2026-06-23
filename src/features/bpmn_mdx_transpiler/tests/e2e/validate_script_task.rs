use cucumber::given;

use super::super::ConvertWorld;
use super::super::write_temp_mdx;

#[given("a temp script task MDX file")]
fn given_script_task(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "script_1.mdx",
        "---\ntype: bpmn:scriptTask\nid: script_1\nname: Run Script\nscriptFormat: javascript\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );
}
