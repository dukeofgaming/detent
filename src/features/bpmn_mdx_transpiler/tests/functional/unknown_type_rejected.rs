use cucumber::given;
use detent::features::bpmn_mdx_transpiler::application::compile::MdxInput;

use super::super::ConvertWorld;

#[given("an MDX input with unknown element type")]
fn given_unknown_type(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![MdxInput {
        filename: "unknown.mdx".to_string(),
        content: "---\ntype: bpmn:unknownElement\nid: x_1\n---\n".to_string(),
    }];
}
