use cucumber::{writer, StatsWriter as _, World, WriterExt as _};
use detent::features::graph_validation::domain::workflow::Workflow;
use std::collections::HashMap;

#[path = "../fixtures/linear_process.rs"]
mod fixtures;
#[path = "../fixtures/branching_process.rs"]
mod branching_fixture;
mod steps;

#[path = "scenarios/well_formed_process_passes.rs"]
mod well_formed_process_passes;
#[path = "scenarios/no_start_event_rejected.rs"]
mod no_start_event_rejected;
#[path = "scenarios/no_end_event_rejected.rs"]
mod no_end_event_rejected;
#[path = "scenarios/dangling_target_reported.rs"]
mod dangling_target_reported;
#[path = "scenarios/duplicate_id_rejected.rs"]
mod duplicate_id_rejected;
#[path = "scenarios/orphan_node_detected.rs"]
mod orphan_node_detected;
#[path = "scenarios/dangling_source_reported.rs"]
mod dangling_source_reported;
#[path = "scenarios/duplicate_flow_id_rejected.rs"]
mod duplicate_flow_id_rejected;
#[path = "scenarios/dead_end_detected.rs"]
mod dead_end_detected;
#[path = "scenarios/process_flow_analysis.rs"]
mod process_flow_analysis;
#[path = "scenarios/branching_process_validated.rs"]
mod branching_process_validated;
#[path = "scenarios/parametrized_fixture_validation.rs"]
mod parametrized_fixture_validation;

use branching_fixture::branching_process;
use fixtures::{linear_process, linear_process_with_retargeted_exit};

#[derive(Debug, Default)]
pub struct FlowAnalysis {
    pub successors: HashMap<String, Vec<String>>,
    pub predecessors: HashMap<String, Vec<String>>,
    pub entry_nodes: Vec<String>,
    pub exit_nodes: Vec<String>,
    pub reachable: HashMap<String, Vec<String>>,
}

#[derive(Debug, Default, World)]
pub struct GraphValidationWorld {
    workflow: Option<Workflow>,
    analysis: Option<FlowAnalysis>,
    errors: Option<Vec<String>>,
    succeeded: bool,
}

fn analysis_of(world: &GraphValidationWorld) -> &FlowAnalysis {
    world
        .analysis
        .as_ref()
        .expect("process flow not analyzed; missing a 'When I analyze the process flow' step")
}

pub async fn run() -> bool {
    let features_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/features/graph_validation/tests/bdd/slice.feature"
    );
    let json_file = std::fs::File::create(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/cucumber-report/graph_validation.json"
        ),
    )
    .expect("Failed to create JSON output");
    let junit_file = std::fs::File::create(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/cucumber-report/graph_validation.junit.xml"
        ),
    )
    .expect("Failed to create JUnit output");
    let json_writer = writer::Json::for_tee(json_file).normalized();
    let junit_writer = writer::JUnit::for_tee(junit_file, 0).normalized();
    let combined = writer::Basic::stdout()
        .summarized()
        .tee(json_writer)
        .tee(junit_writer);
    let summarized = GraphValidationWorld::cucumber()
        .with_writer(combined)
        .run(features_path)
        .await;
    summarized.execution_has_failed()
}
