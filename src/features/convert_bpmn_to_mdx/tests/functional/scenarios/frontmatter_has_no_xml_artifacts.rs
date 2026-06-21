use cucumber::then;

use super::ConvertWorld;

#[then("no output contains '@' or '$text' in its frontmatter")]
fn then_no_xml_artifacts(world: &mut ConvertWorld) {
    let outputs = world.import_outputs.as_ref().expect("expected import outputs");
    for output in outputs {
        assert!(
            !output.content.contains("'@"),
            "{} contains '@' key prefix",
            output.filename
        );
        assert!(
            !output.content.contains("\"@"),
            "{} contains \"@\" key prefix",
            output.filename
        );
        assert!(
            !output.content.contains("$text"),
            "{} contains $text key",
            output.filename
        );
    }
}
