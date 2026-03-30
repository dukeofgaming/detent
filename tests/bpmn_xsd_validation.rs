//! Integration tests for BPMN XSD validation
//!
//! These tests validate BPMN files against the BPMN 2.0 XSD schema.

#![cfg(feature = "xsd-validation")]

use detent::bpmn::validate_bpmn_xsd;

const HELLO_WORLD_BPMN: &str = include_str!("assets/processes/hello-world/hello-world.bpmn");
const HELLO_WORLD_BPMN2: &str = include_str!("assets/processes/hello-world/hello-world.bpmn2");

fn hello_world_fixtures() -> [(&'static str, &'static str); 2] {
    [
        ("hello-world.bpmn", HELLO_WORLD_BPMN),
        ("hello-world.bpmn2", HELLO_WORLD_BPMN2),
    ]
}

#[test]
fn test_valid_hello_world_files_pass_xsd_validation() {
    for (name, content) in hello_world_fixtures() {
        let result = validate_bpmn_xsd(content);
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

        let result = validate_bpmn_xsd(&invalid_bpmn);
        assert!(
            result.is_err(),
            "Expected {} with invalid element to fail XSD validation",
            name
        );
    }
}

#[test]
fn test_missing_required_attribute_fails_xsd_validation() {
    for (name, content) in hello_world_fixtures() {
        let invalid_bpmn = content.replacen("targetNamespace=", "removedTargetNamespace=", 1);

        let result = validate_bpmn_xsd(&invalid_bpmn);
        assert!(
            result.is_err(),
            "Expected {} without targetNamespace to fail XSD validation",
            name
        );
    }
}
