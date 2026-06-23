use cucumber::{then, when};

use super::super::ConvertWorld;

#[when("I import it to MDX")]
fn when_import(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    let defs = detent::features::bpmn_mdx_transpiler::adapters::bpmn::parse_bpmn(xml)
        .expect("parse_bpmn failed");
    let outputs = detent::features::bpmn_mdx_transpiler::application::import::import_to_mdx(&defs)
        .expect("import_to_mdx failed");
    world.import_outputs = Some(outputs);
}

#[then(regex = r"^(\d+) MDX outputs are produced$")]
fn then_n_outputs(world: &mut ConvertWorld, n: usize) {
    let outputs = world.import_outputs.as_ref().expect("expected import outputs");
    assert_eq!(outputs.len(), n);
}

#[then("each output contains a frontmatter block")]
fn then_all_have_frontmatter(world: &mut ConvertWorld) {
    let outputs = world.import_outputs.as_ref().expect("expected import outputs");
    for output in outputs {
        assert!(
            output.content.starts_with("---\n"),
            "{} missing opening ---",
            output.filename
        );
        assert!(
            output.content.contains("\n---\n"),
            "{} missing closing ---",
            output.filename
        );
    }
}

#[then("each output contains its BPMN type")]
fn then_all_have_bpmn_type(world: &mut ConvertWorld) {
    let outputs = world.import_outputs.as_ref().expect("expected import outputs");
    for output in outputs {
        assert!(
            output.content.contains("type: bpmn:"),
            "{} missing type field",
            output.filename
        );
    }
}
