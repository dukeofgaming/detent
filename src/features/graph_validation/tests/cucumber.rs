//! Cucumber BDD scenarios for the graph_validation slice.
//!
//! Exposes `run()` so the root cucumber harness can sequence multiple slices
//! inside one tokio runtime. Returns `true` if any scenario failed.

use cucumber::{given, then, when, writer, StatsWriter as _, World, WriterExt as _};
use detent::features::graph_validation::domain::bpmn::{
    EndEvent, Process, SequenceFlow, StartEvent, Task,
};
use detent::features::graph_validation::domain::graph::Graph;

#[derive(Debug, Default, World)]
pub struct GraphValidationWorld {
    process: Option<Process>,
    errors: Option<Vec<String>>,
    succeeded: bool,
}

fn linear_process() -> Process {
    Process {
        id: "process_1".to_string(),
        name: Some("Linear Process".to_string()),
        is_executable: Some(true),
        process_type: None,
        documentation: None,
        start_events: vec![StartEvent {
            id: "start_1".to_string(),
            name: None,
            outgoing: vec!["flow_1".to_string()],
            documentation: None,
        }],
        end_events: vec![EndEvent {
            id: "end_1".to_string(),
            name: None,
            incoming: vec!["flow_2".to_string()],
            documentation: None,
        }],
        tasks: vec![Task {
            id: "task_1".to_string(),
            name: Some("Do Something".to_string()),
            incoming: vec!["flow_1".to_string()],
            outgoing: vec!["flow_2".to_string()],
            documentation: None,
        }],
        service_tasks: vec![],
        script_tasks: vec![],
        exclusive_gateways: vec![],
        parallel_gateways: vec![],
        sequence_flows: vec![
            SequenceFlow {
                id: "flow_1".to_string(),
                name: None,
                source_ref: "start_1".to_string(),
                target_ref: "task_1".to_string(),
                condition_expression: None,
                documentation: None,
            },
            SequenceFlow {
                id: "flow_2".to_string(),
                name: None,
                source_ref: "task_1".to_string(),
                target_ref: "end_1".to_string(),
                condition_expression: None,
                documentation: None,
            },
        ],
    }
}

#[given("a linear process with one start event, one task, and one end event")]
fn given_linear(world: &mut GraphValidationWorld) {
    world.process = Some(linear_process());
}

#[given("a linear process with no start event")]
fn given_no_start(world: &mut GraphValidationWorld) {
    let mut p = linear_process();
    p.start_events.clear();
    world.process = Some(p);
}

#[given("a linear process with no end event")]
fn given_no_end(world: &mut GraphValidationWorld) {
    let mut p = linear_process();
    p.end_events.clear();
    world.process = Some(p);
}

#[given(regex = r#"^a linear process whose first sequence flow targets a missing node "([^"]+)"$"#)]
fn given_dangling_target(world: &mut GraphValidationWorld, target: String) {
    let mut p = linear_process();
    p.sequence_flows[1].target_ref = target;
    world.process = Some(p);
}

#[given(regex = r#"^a linear process whose task reuses the start event id "([^"]+)"$"#)]
fn given_duplicate_id(world: &mut GraphValidationWorld, dup_id: String) {
    let mut p = linear_process();
    p.tasks[0].id = dup_id;
    world.process = Some(p);
}

#[given(regex = r#"^a linear process with an orphan task "([^"]+)"$"#)]
fn given_orphan(world: &mut GraphValidationWorld, orphan_id: String) {
    let mut p = linear_process();
    p.tasks.push(Task {
        id: orphan_id,
        name: Some("Orphan".to_string()),
        incoming: vec![],
        outgoing: vec![],
        documentation: None,
    });
    world.process = Some(p);
}

#[when("I validate the process graph")]
fn when_validate(world: &mut GraphValidationWorld) {
    let process = world.process.as_ref().expect("process must be set by a Given step");
    let graph = Graph::new(process);
    match graph.validate() {
        Ok(()) => world.succeeded = true,
        Err(errs) => world.errors = Some(errs),
    }
}

#[then("validation succeeds")]
fn then_succeeds(world: &mut GraphValidationWorld) {
    assert!(
        world.succeeded,
        "expected validation to succeed but got errors: {:?}",
        world.errors
    );
}

#[then(regex = r#"^validation fails reporting "([^"]+)"$"#)]
fn then_fails_with(world: &mut GraphValidationWorld, snippet: String) {
    let errs = world
        .errors
        .as_ref()
        .expect("expected validation to fail but it succeeded");
    assert!(
        errs.iter().any(|e| e.contains(&snippet)),
        "no error contained {:?}; errors were: {:?}",
        snippet,
        errs
    );
}

pub async fn run() -> bool {
    let features_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/features/graph_validation/tests/graph_validation.feature"
    );
    let summarized = GraphValidationWorld::cucumber()
        .with_writer(writer::Basic::stdout().summarized())
        .run(features_path)
        .await;
    summarized.execution_has_failed()
}
