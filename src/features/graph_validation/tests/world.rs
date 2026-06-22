use cucumber::{writer, StatsWriter as _, World, WriterExt as _};
use detent::features::graph_validation::domain::workflow::Workflow;
use std::collections::HashMap;

mod fixtures;
mod steps;
mod functional;
mod integration;
mod unit;

pub(crate) use fixtures::branching_process;
pub(crate) use fixtures::linear_process;
pub(crate) use fixtures::linear_process_with_retargeted_exit;

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
    pub found_node: Option<String>,
    pub last_successors: Vec<String>,
}

pub(crate) fn analysis_of(world: &GraphValidationWorld) -> &FlowAnalysis {
    world
        .analysis
        .as_ref()
        .expect("process flow not analyzed; missing a 'When I analyze the process flow' step")
}

pub async fn run() -> bool {
    let features_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/features/graph_validation/tests"
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
        .filter_run(features_path, |_, _, scenario| {
            !scenario.tags.iter().any(|tag| tag == "ignore")
        })
        .await;
    summarized.execution_has_failed()
}
