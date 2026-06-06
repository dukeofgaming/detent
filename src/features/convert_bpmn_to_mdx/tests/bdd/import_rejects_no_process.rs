use cucumber::given;

use super::ConvertWorld;

#[given("a BPMN definition with no process")]
fn given_no_process(world: &mut ConvertWorld) {
    world.bpmn_xml = Some(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL"
  id="def_1" targetNamespace="http://bpmn.io/schema/bpmn">
</bpmn2:definitions>"#
            .to_string(),
    );
}
