use cucumber::given;
use detent::features::convert_bpmn_to_mdx::use_cases::compile::MdxInput;

use super::super::ConvertWorld;

#[given("an MDX input with unknown element type")]
fn given_unknown_type(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![MdxInput {
        filename: "unknown.mdx".to_string(),
        content: "---\ntype: bpmn:unknownElement\nid: x_1\n---\n".to_string(),
    }];
}
