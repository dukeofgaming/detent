const HELLO_WORLD_BPMN: &str =
    include_str!("../assets/hello_world/hello-world.bpmn");
const HELLO_WORLD_BPMN2: &str =
    include_str!("../assets/hello_world/hello-world.bpmn2");

fn hello_world_fixtures() -> [(&'static str, &'static str); 2] {
    [
        ("hello-world.bpmn", HELLO_WORLD_BPMN),
        ("hello-world.bpmn2", HELLO_WORLD_BPMN2),
    ]
}

#[test]
fn test_valid_hello_world_files_pass_xsd_validation() {
    for (name, content) in hello_world_fixtures() {
        let result =
            detent::features::convert_bpmn_to_mdx::adapters::bpmn::validate_bpmn_xsd(content);
        assert!(
            result.is_ok(),
            "Expected {} to pass XSD validation: {:?}",
            name,
            result
        );
    }
}

#[test]
fn test_invalid_element_fails_xsd_validation() {
    for (name, content) in hello_world_fixtures() {
        let invalid_bpmn = content
            .replace("<bpmn2:task", "<bpmn2:notARealTask")
            .replace("</bpmn2:task>", "</bpmn2:notARealTask>");
        let result =
            detent::features::convert_bpmn_to_mdx::adapters::bpmn::validate_bpmn_xsd(&invalid_bpmn);
        assert!(
            result.is_err(),
            "Expected {} with invalid element to fail",
            name
        );
    }
}

#[test]
fn test_missing_required_attribute_fails_xsd_validation() {
    for (name, content) in hello_world_fixtures() {
        let invalid_bpmn = content.replacen("targetNamespace=", "removedTargetNamespace=", 1);
        let result =
            detent::features::convert_bpmn_to_mdx::adapters::bpmn::validate_bpmn_xsd(&invalid_bpmn);
        assert!(
            result.is_err(),
            "Expected {} without targetNamespace to fail",
            name
        );
    }
}
