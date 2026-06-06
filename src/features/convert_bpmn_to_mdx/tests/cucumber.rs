//! Cucumber BDD scenarios for the convert_bpmn_to_mdx slice.
//!
//! Exposes `run()` so the root cucumber harness can sequence multiple slices
//! inside one tokio runtime. Returns `true` if any scenario failed.

use cucumber::{given, then, when, writer, StatsWriter as _, World, WriterExt as _};
use detent::features::convert_bpmn_to_mdx::adapters::bpmn::{parse_bpmn, Definitions};
use detent::features::convert_bpmn_to_mdx::use_cases::compile::{compile_to_definitions, MdxInput};
use detent::features::convert_bpmn_to_mdx::use_cases::import::{import_to_mdx, MdxOutput};
use std::fs;
use std::path::PathBuf;

const HELLO_WORLD_ASSET_DIR: &str = "src/features/convert_bpmn_to_mdx/tests/assets/hello_world";

fn hello_world_asset_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(HELLO_WORLD_ASSET_DIR)
        .join(name)
}

#[derive(Debug, Default, World)]
pub struct ConvertWorld {
    bpmn_xml: Option<String>,
    mdx_inputs: Vec<MdxInput>,
    compile_result: Option<Definitions>,
    compile_failed: bool,
    import_outputs: Option<Vec<MdxOutput>>,
}

fn minimal_mdx_inputs() -> Vec<MdxInput> {
    vec![
        MdxInput {
            filename: "start_1.mdx".to_string(),
            content: "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "end_1.mdx".to_string(),
            content: "---\ntype: bpmn:endEvent\nid: end_1\nincoming:\n- flow_2\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "task_1.mdx".to_string(),
            content: "---\ntype: bpmn:task\nid: task_1\nname: Do Something\nincoming:\n- flow_1\noutgoing:\n- flow_2\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "flow_1.mdx".to_string(),
            content: "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: task_1\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "flow_2.mdx".to_string(),
            content: "---\ntype: bpmn:sequenceFlow\nid: flow_2\nsourceRef: task_1\ntargetRef: end_1\n---\n"
                .to_string(),
        },
    ]
}

#[given("the hello-world BPMN fixture")]
fn given_hello_world(world: &mut ConvertWorld) {
    let xml = fs::read_to_string(hello_world_asset_path("hello-world.bpmn2"))
        .expect("Failed to read hello-world.bpmn2");
    world.bpmn_xml = Some(xml);
}

#[given("a minimal MDX input set with start, task, end, and two flows")]
fn given_minimal_inputs(world: &mut ConvertWorld) {
    world.mdx_inputs = minimal_mdx_inputs();
}

#[given("no MDX inputs")]
fn given_no_inputs(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![];
}

#[given("an MDX input whose frontmatter omits the type field")]
fn given_missing_type(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![MdxInput {
        filename: "no_type.mdx".to_string(),
        content: "---\nid: start_1\n---\n".to_string(),
    }];
}

#[given("an MDX input set where a sequence flow targets a non-existent node")]
fn given_dangling_flow(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![
        MdxInput {
            filename: "start_1.mdx".to_string(),
            content: "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n"
                .to_string(),
        },
        MdxInput {
            filename: "flow_1.mdx".to_string(),
            content: "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: ghost_task\n---\n"
                .to_string(),
        },
    ];
}

#[when("I import it to MDX")]
fn when_import(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    let defs = parse_bpmn(xml).expect("parse_bpmn failed");
    let outputs = import_to_mdx(&defs).expect("import_to_mdx failed");
    world.import_outputs = Some(outputs);
}

#[when("I compile the MDX inputs to definitions")]
fn when_compile(world: &mut ConvertWorld) {
    match compile_to_definitions(&world.mdx_inputs) {
        Ok(defs) => world.compile_result = Some(defs),
        Err(_) => world.compile_failed = true,
    }
}

#[when("I attempt to compile the MDX inputs to definitions")]
fn when_attempt_compile(world: &mut ConvertWorld) {
    when_compile(world);
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

#[then(
    regex = r"^a process is produced with (\d+) start event, (\d+) task, (\d+) end event, and (\d+) sequence flows$"
)]
fn then_process_shape(
    world: &mut ConvertWorld,
    starts: usize,
    tasks: usize,
    ends: usize,
    flows: usize,
) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let process = defs.process.as_ref().expect("expected a process");
    assert_eq!(process.start_events.len(), starts, "start_events");
    assert_eq!(process.tasks.len(), tasks, "tasks");
    assert_eq!(process.end_events.len(), ends, "end_events");
    assert_eq!(process.sequence_flows.len(), flows, "sequence_flows");
}

#[then("compilation fails")]
fn then_compile_fails(world: &mut ConvertWorld) {
    assert!(
        world.compile_failed,
        "expected compilation to fail but it produced: {:?}",
        world.compile_result.as_ref().map(|d| &d.id)
    );
}

#[then("compilation succeeds")]
fn then_compile_succeeds(world: &mut ConvertWorld) {
    assert!(
        !world.compile_failed && world.compile_result.is_some(),
        "expected compilation to succeed"
    );
}

#[then("the resulting process contains the dangling target id")]
fn then_dangling_preserved(world: &mut ConvertWorld) {
    let defs = world.compile_result.as_ref().expect("expected definitions");
    let process = defs.process.as_ref().expect("expected a process");
    assert!(
        process
            .sequence_flows
            .iter()
            .any(|f| f.target_ref == "ghost_task"),
        "ghost_task target ref was not preserved"
    );
}

pub async fn run() -> bool {
    let features_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/features/convert_bpmn_to_mdx/tests/convert_bpmn_to_mdx.feature"
    );
    let summarized = ConvertWorld::cucumber()
        .with_writer(writer::Basic::stdout().summarized())
        .run(features_path)
        .await;
    summarized.execution_has_failed()
}
