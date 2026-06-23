use cucumber::given;
use detent::features::bpmn_mdx_transpiler::application::compile::MdxInput;

use super::super::ConvertWorld;

#[given("an MDX input whose frontmatter omits the type field")]
fn given_missing_type(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![MdxInput {
        filename: "no_type.mdx".to_string(),
        content: "---\nid: start_1\n---\n".to_string(),
    }];
}
