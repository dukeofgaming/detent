use cucumber::{given, then, when};

use super::ConvertWorld;

const HELLO_WORLD_BPMN: &str =
    include_str!("../../../assets/hello_world/hello-world.bpmn");
const HELLO_WORLD_BPMN2: &str =
    include_str!("../../../assets/hello_world/hello-world.bpmn2");

#[given(regex = r#"^hello-world BPMN content from "(hello-world\.bpmn|hello-world\.bpmn2)"$"#)]
fn given_fixture(world: &mut ConvertWorld, name: String) {
    world.bpmn_xml = Some(match name.as_str() {
        "hello-world.bpmn" => HELLO_WORLD_BPMN.to_string(),
        "hello-world.bpmn2" => HELLO_WORLD_BPMN2.to_string(),
        _ => unreachable!(),
    });
    world.unit_last_error = None;
}

#[when("I validate the BPMN against XSD")]
fn when_validate_xsd(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    match detent::features::convert_bpmn_to_mdx::infrastructure::validate_bpmn_xsd(xml) {
        Ok(()) => world.e2e_last_success = true,
        Err(e) => {
            world.e2e_last_success = false;
            world.unit_last_error = Some(format!("{e:?}"));
        }
    }
}

#[when(regex = r#"^I validate BPMN with invalid element in "(hello-world\.bpmn|hello-world\.bpmn2)"$"#)]
fn when_validate_invalid_element(world: &mut ConvertWorld, name: String) {
    let content = match name.as_str() {
        "hello-world.bpmn" => HELLO_WORLD_BPMN,
        "hello-world.bpmn2" => HELLO_WORLD_BPMN2,
        _ => unreachable!(),
    };
    let invalid = content
        .replace("<bpmn2:task", "<bpmn2:notARealTask")
        .replace("</bpmn2:task>", "</bpmn2:notARealTask>");
    world.e2e_last_success =
        detent::features::convert_bpmn_to_mdx::infrastructure::validate_bpmn_xsd(&invalid).is_ok();
}

#[when(regex = r#"^I validate BPMN missing targetNamespace in "(hello-world\.bpmn|hello-world\.bpmn2)"$"#)]
fn when_validate_missing_attr(world: &mut ConvertWorld, name: String) {
    let content = match name.as_str() {
        "hello-world.bpmn" => HELLO_WORLD_BPMN,
        "hello-world.bpmn2" => HELLO_WORLD_BPMN2,
        _ => unreachable!(),
    };
    let invalid = content.replacen("targetNamespace=", "removedTargetNamespace=", 1);
    world.e2e_last_success =
        detent::features::convert_bpmn_to_mdx::infrastructure::validate_bpmn_xsd(&invalid).is_ok();
}

#[then("XSD validation succeeds")]
fn then_xsd_ok(world: &mut ConvertWorld) {
    assert!(world.e2e_last_success, "expected XSD ok: {:?}", world.unit_last_error);
}

#[then("XSD validation fails")]
fn then_xsd_fails(world: &mut ConvertWorld) {
    assert!(!world.e2e_last_success, "expected XSD failure");
}
