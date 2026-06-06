use detent::features::convert_bpmn_to_mdx::use_cases::compile::{compile_to_definitions, MdxInput};

use super::hello_world_asset_path;

#[test]
fn test_compile_with_hello_world_reference_files() {
    let inputs = vec![
        MdxInput { filename: "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")).unwrap() },
        MdxInput { filename: "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")).unwrap() },
        MdxInput { filename: "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx")).unwrap() },
        MdxInput { filename: "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_4083739B-66F0-4B92-A348-A37DF3B29083.mdx")).unwrap() },
        MdxInput { filename: "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx")).unwrap() },
    ];
    let defs = compile_to_definitions(&inputs).unwrap();
    let process = defs.process.as_ref().unwrap();
    assert_eq!(process.tasks[0].name, Some("Hello World".to_string()));
}
