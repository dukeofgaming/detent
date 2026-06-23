use std::fs;

use cucumber::{given, then, when};
use detent::features::convert_bpmn_to_mdx::adapters::bpmn::{SequenceFlow, StartEvent, Task};
use detent::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;

use super::super::ConvertWorld;

#[given(regex = r#"^the hello-world MDX file "([^"]+)"$"#)]
fn given_mdx_file(world: &mut ConvertWorld, filename: String) {
    let content = fs::read_to_string(super::super::hello_world_asset_path(&filename))
        .unwrap_or_else(|_| panic!("Failed to read {filename}"));
    world.mdx_inputs = vec![detent::features::convert_bpmn_to_mdx::application::compile::MdxInput {
        filename: filename.clone(),
        content,
    }];
    world.unit_last_error = None;
}

#[when("I parse the MDX file")]
fn when_parse_mdx(world: &mut ConvertWorld) {
    let input = &world.mdx_inputs[0];
    match MdxFile::parse(&input.content) {
        Ok(mdx) => {
            world.e2e_file_content = Some(format!("{}|{}", mdx.frontmatter, mdx.body));
        }
        Err(e) => world.unit_last_error = Some(e.to_string()),
    }
}

#[when("I parse the MDX file as a start event")]
fn when_parse_start_event(world: &mut ConvertWorld) {
    let input = &world.mdx_inputs[0];
    let mdx = MdxFile::parse(&input.content).expect("parse MDX");
    match mdx.parse_start_event() {
        Ok(event) => world.e2e_file_content = Some(event.id.clone()),
        Err(e) => world.unit_last_error = Some(e.to_string()),
    }
}

#[when("I parse the MDX file as an end event")]
fn when_parse_end_event(world: &mut ConvertWorld) {
    let input = &world.mdx_inputs[0];
    let mdx = MdxFile::parse(&input.content).expect("parse MDX");
    match mdx.parse_end_event() {
        Ok(event) => world.e2e_file_content = Some(event.id.clone()),
        Err(e) => world.unit_last_error = Some(e.to_string()),
    }
}

#[when("I parse the MDX file as a task")]
fn when_parse_task(world: &mut ConvertWorld) {
    let input = &world.mdx_inputs[0];
    let mdx = MdxFile::parse(&input.content).expect("parse MDX");
    match mdx.parse_task() {
        Ok(task) => {
            world.e2e_file_content = Some(format!(
                "{}|{}|{:?}",
                task.id,
                task.name.as_deref().unwrap_or(""),
                task.documentation.as_ref().map(|d| &d.text)
            ));
        }
        Err(e) => world.unit_last_error = Some(e.to_string()),
    }
}

#[when("I parse the MDX file as a sequence flow")]
fn when_parse_sequence_flow(world: &mut ConvertWorld) {
    let input = &world.mdx_inputs[0];
    let mdx = MdxFile::parse(&input.content).expect("parse MDX");
    match mdx.parse_sequence_flow() {
        Ok(flow) => {
            world.e2e_file_content = Some(format!("{}|{}", flow.source_ref, flow.target_ref));
        }
        Err(e) => world.unit_last_error = Some(e.to_string()),
    }
}

#[when(regex = r#"^I roundtrip the MDX file as a (start event|task|sequence flow)$"#)]
fn when_roundtrip(world: &mut ConvertWorld, kind: String) {
    let input = &world.mdx_inputs[0];
    // Act
    let mdx = MdxFile::parse(&input.content).expect("parse MDX");
    match kind.as_str() {
        "start event" => {
            // Act
            let event = mdx.parse_start_event().expect("parse start event");
            // Act
            let yaml = serde_yaml::to_string(&event).expect("serialize");
            // Act
            let parsed: StartEvent = serde_yaml::from_str(&yaml).expect("deserialize");
            // Assert
            assert_eq!(event, parsed);
        }
        "task" => {
            // Act
            let task = mdx.parse_task().expect("parse task");
            // Act
            let yaml = serde_yaml::to_string(&task).expect("serialize");
            // Act
            let parsed: Task = serde_yaml::from_str(&yaml).expect("deserialize");
            // Assert
            assert_eq!(task, parsed);
        }
        "sequence flow" => {
            // Act
            let flow = mdx.parse_sequence_flow().expect("parse flow");
            // Act
            let yaml = serde_yaml::to_string(&flow).expect("serialize");
            // Act
            let parsed: SequenceFlow = serde_yaml::from_str(&yaml).expect("deserialize");
            // Assert
            assert_eq!(flow, parsed);
        }
        _ => unreachable!(),
    }
}

#[then(regex = r#"^the MDX frontmatter contains "([^"]+)"$"#)]
fn then_frontmatter_contains(world: &mut ConvertWorld, snippet: String) {
    let content = world.e2e_file_content.as_ref().expect("expected parsed content");
    assert!(content.contains(&snippet), "expected {:?} in {:?}", snippet, content);
}

#[then(regex = r#"^the MDX body contains "([^"]+)"$"#)]
fn then_body_contains(world: &mut ConvertWorld, snippet: String) {
    let content = world.e2e_file_content.as_ref().expect("expected parsed content");
    let body = content.split('|').nth(1).unwrap_or(content.as_str());
    assert!(body.contains(&snippet), "expected {:?} in body", snippet);
}

#[then(regex = r#"^the parsed task id is "([^"]+)"$"#)]
fn then_task_id(world: &mut ConvertWorld, expected: String) {
    let content = world.e2e_file_content.as_ref().expect("expected task content");
    assert!(
        content.starts_with(&format!("{expected}|")),
        "expected task id {expected} in {content}"
    );
}

#[then(regex = r#"^the parsed id is "([^"]+)"$"#)]
fn then_parsed_id(world: &mut ConvertWorld, expected: String) {
    assert_eq!(
        world.e2e_file_content.as_deref(),
        Some(expected.as_str()),
        "unexpected parsed id"
    );
}

#[then(regex = r#"^the parsed task name is "([^"]+)"$"#)]
fn then_task_name(world: &mut ConvertWorld, expected: String) {
    let content = world.e2e_file_content.as_ref().expect("expected task content");
    assert!(content.contains(&format!("|{expected}|")), "expected task name {expected} in {content}");
}

#[then("the parsed task has documentation")]
fn then_task_documentation(world: &mut ConvertWorld) {
    let content = world.e2e_file_content.as_ref().expect("expected task content");
    assert!(
        content.contains("Some(\"T\")"),
        "expected documentation in {content}"
    );
}

#[then(regex = r#"^the sequence flow connects "([^"]+)" to "([^"]+)"$"#)]
fn then_flow_endpoints(world: &mut ConvertWorld, source: String, target: String) {
    assert_eq!(
        world.e2e_file_content.as_deref(),
        Some(format!("{source}|{target}").as_str())
    );
}

#[given("all hello-world per-element MDX fixtures")]
fn given_all_mdx(world: &mut ConvertWorld) {
    world.mdx_inputs.clear();
    for name in [
        "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx",
        "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx",
        "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx",
        "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx",
        "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx",
    ] {
        let content = fs::read_to_string(super::super::hello_world_asset_path(name))
            .unwrap_or_else(|_| panic!("Failed to read {name}"));
        world.mdx_inputs.push(
            detent::features::convert_bpmn_to_mdx::application::compile::MdxInput {
                filename: name.to_string(),
                content,
            },
        );
    }
}

#[when("I parse each MDX file")]
fn when_parse_each(world: &mut ConvertWorld) {
    for input in &world.mdx_inputs {
        // Act
        let mdx = MdxFile::parse(&input.content)
            .unwrap_or_else(|_| panic!("Failed to parse {}", input.filename));
        // Assert
        assert!(mdx.frontmatter.contains("type:"), "missing type in {}", input.filename);
        // Assert
        assert!(mdx.frontmatter.contains("id:"), "missing id in {}", input.filename);
    }
}

#[then("all MDX files parse successfully")]
fn then_all_parse(_world: &mut ConvertWorld) {}
