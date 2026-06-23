use cucumber::given;
use detent::features::bpmn_mdx_transpiler::application::compile::MdxInput;

use super::super::ConvertWorld;

#[given("an MDX input with no frontmatter")]
fn given_no_frontmatter(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![MdxInput {
        filename: "bad.mdx".to_string(),
        content: "# No frontmatter".to_string(),
    }];
}
